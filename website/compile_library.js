const fs = require("fs");
const path = require("path");
const yaml = require("js-yaml");

const libraryDir = path.resolve(__dirname, "../library");
const outputDir = path.resolve(__dirname, "src/_data");
const outputFile = path.join(outputDir, "sdt_library.json");

// Ensure output directory exists
if (!fs.existsSync(outputDir)) {
  fs.mkdirSync(outputDir, { recursive: true });
}

function getYamlFiles(dir, files = []) {
  if (!fs.existsSync(dir)) {
    console.warn(`Library directory not found at: ${dir}`);
    return files;
  }
  const list = fs.readdirSync(dir);
  for (const item of list) {
    const fullPath = path.join(dir, item);
    const stat = fs.statSync(fullPath);
    if (stat.isDirectory()) {
      getYamlFiles(fullPath, files);
    } else if (item.endsWith(".yaml") || item.endsWith(".yml")) {
      files.push(fullPath);
    }
  }
  return files;
}

try {
  console.log(`Searching for SDT definitions in ${libraryDir}...`);
  const yamlFiles = getYamlFiles(libraryDir);
  console.log(`Found ${yamlFiles.length} definitions.`);

  const library = [];

  for (const filePath of yamlFiles) {
    const content = fs.readFileSync(filePath, "utf8");
    const doc = yaml.load(content);

    if (!doc.id || !doc.name) {
      console.warn(`Skipping invalid SDT (missing id or name) at ${filePath}`);
      continue;
    }

    // Compute relative path details
    const relativePath = path.relative(libraryDir, filePath); // e.g. "us/identifiers/ssn.yaml"
    const parsedPath = path.parse(relativePath); // dir: "us/identifiers", name: "ssn"

    // Build slug and categories
    const category = parsedPath.dir || "global";
    const key = parsedPath.name;
    const slug = `${category.replace(/\//g, "-")}-${key}`; // e.g. "us-identifiers-ssn" or "global-isbn"

    doc.meta = {
      relativePath,
      category,
      key,
      slug,
      filePath: path.relative(path.resolve(__dirname, ".."), filePath)
    };

    library.push(doc);
  }

  // Sort by category then by name
  library.sort((a, b) => {
    if (a.meta.category !== b.meta.category) {
      return a.meta.category.localeCompare(b.meta.category);
    }
    return a.name.localeCompare(b.name);
  });

  fs.writeFileSync(outputFile, JSON.stringify(library, null, 2), "utf8");
  console.log(`Successfully compiled ${library.length} SDT definitions to ${outputFile}`);

  // Compile LinkML Schema
  const schemaFile = path.resolve(__dirname, "../linkml/dev/sdt.yaml");
  const schemaOutputFile = path.join(outputDir, "sdt_schema.json");
  console.log(`Parsing LinkML schema from ${schemaFile}...`);
  if (fs.existsSync(schemaFile)) {
    const schemaContent = fs.readFileSync(schemaFile, "utf8");
    const schemaDoc = yaml.load(schemaContent);
    fs.writeFileSync(schemaOutputFile, JSON.stringify(schemaDoc, null, 2), "utf8");
    console.log(`Successfully compiled schema to ${schemaOutputFile}`);
  } else {
    console.warn(`Schema file not found at ${schemaFile}`);
  }
} catch (err) {
  console.error("Failed to compile SDT library or schema:", err);
  process.exit(1);
}
