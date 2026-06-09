import subprocess
import sys
from pathlib import Path


def main():
    root = Path(__file__).resolve().parent.parent
    schema = root / "linkml" / "dev" / "sdt.yaml"

    if not schema.exists():
        print(f"Error: Schema not found at {schema}")
        sys.exit(1)

    print(f"Using schema: {schema}")

    schema_dir = schema.parent  # e.g., root / "linkml" / "dev"

    # 1. Pydantic
    pydantic_dir = schema_dir / "python"
    pydantic_dir.mkdir(parents=True, exist_ok=True)
    pydantic_out = pydantic_dir / "sdt.py"
    print(f"Generating Pydantic models -> {pydantic_out}")
    subprocess.run(["gen-pydantic", str(schema)], stdout=pydantic_out.open("w"), check=True)

    # 2. TypeScript
    ts_dir = schema_dir / "typescript"
    ts_dir.mkdir(parents=True, exist_ok=True)
    ts_out = ts_dir / "sdt.ts"
    print(f"Generating TypeScript interfaces -> {ts_out}")
    subprocess.run(["gen-typescript", str(schema)], stdout=ts_out.open("w"), check=True)

    # 3. Rust (crate mode)
    rust_dir = schema_dir / "rust"
    rust_dir.mkdir(parents=True, exist_ok=True)
    print(f"Generating Rust models -> {rust_dir}")
    # gen-rust options: --output specifies the directory, --force overwrites if exists
    subprocess.run(["gen-rust", "--output", str(rust_dir), "--force", str(schema)], check=True)

    # 4. PlantUML
    puml_dir = schema_dir / "plantuml"
    puml_dir.mkdir(parents=True, exist_ok=True)
    puml_out = puml_dir / "sdt.puml"
    print(f"Generating PlantUML diagram -> {puml_out}")
    subprocess.run(["gen-plantuml", str(schema)], stdout=puml_out.open("w"), check=True)

    # 5. Documentation
    doc_dir = schema_dir / "docs"
    doc_dir.mkdir(parents=True, exist_ok=True)
    print(f"Generating Markdown documentation -> {doc_dir}")
    subprocess.run(["gen-doc", "-d", str(doc_dir), str(schema)], check=True)

    # 6. RDF (Turtle)
    rdf_dir = schema_dir / "rdf"
    rdf_dir.mkdir(parents=True, exist_ok=True)
    rdf_out = rdf_dir / "sdt.ttl"
    print(f"Generating RDF Turtle -> {rdf_out}")
    subprocess.run(["gen-rdf", str(schema)], stdout=rdf_out.open("w"), check=True)

    # 7. SHACL
    shacl_dir = schema_dir / "shacl"
    shacl_dir.mkdir(parents=True, exist_ok=True)
    shacl_out = shacl_dir / "sdt.shacl.ttl"
    print(f"Generating SHACL shapes -> {shacl_out}")
    subprocess.run(["gen-shacl", str(schema)], stdout=shacl_out.open("w"), check=True)

    # 8. JSON-LD Context and Schema
    jsonld_dir = schema_dir / "jsonld"
    jsonld_dir.mkdir(parents=True, exist_ok=True)
    jsonld_context_out = jsonld_dir / "sdt.context.jsonld"
    print(f"Generating JSON-LD context -> {jsonld_context_out}")
    subprocess.run(["gen-jsonld-context", str(schema)], stdout=jsonld_context_out.open("w"), check=True)

    jsonld_out = jsonld_dir / "sdt.jsonld"
    print(f"Generating JSON-LD schema -> {jsonld_out}")
    subprocess.run(["gen-jsonld", str(schema)], stdout=jsonld_out.open("w"), check=True)

    # 9. OWL
    owl_dir = schema_dir / "owl"
    owl_dir.mkdir(parents=True, exist_ok=True)
    owl_out = owl_dir / "sdt.owl.ttl"
    print(f"Generating OWL ontology -> {owl_out}")
    subprocess.run(["gen-owl", str(schema)], stdout=owl_out.open("w"), check=True)

    # 10. JSON Schema
    jsonschema_dir = schema_dir / "jsonschema"
    jsonschema_dir.mkdir(parents=True, exist_ok=True)
    jsonschema_out = jsonschema_dir / "sdt.schema.json"
    print(f"Generating JSON Schema -> {jsonschema_out}")
    subprocess.run(["gen-json-schema", str(schema)], stdout=jsonschema_out.open("w"), check=True)

    # 11. Java
    java_dir = schema_dir / "java"
    java_dir.mkdir(parents=True, exist_ok=True)
    print(f"Generating Java classes -> {java_dir}")
    subprocess.run(
        [
            "gen-java",
            "--package",
            "org.dataartifex.sdt",
            "--output-directory",
            str(java_dir),
            str(schema),
        ],
        check=True,
    )

    print("All LinkML outputs generated successfully!")


if __name__ == "__main__":
    main()
