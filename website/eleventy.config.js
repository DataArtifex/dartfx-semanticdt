module.exports = function(eleventyConfig) {
  // Pass through styles and client-side scripts
  eleventyConfig.addPassthroughCopy("src/css");
  eleventyConfig.addPassthroughCopy("src/js");

  // Copy raw library YAML definitions to /definitions
  eleventyConfig.addPassthroughCopy({ "../library": "definitions" });

  // Custom filter to pretty-print JSON for download links
  eleventyConfig.addFilter("stringify", function(value) {
    return JSON.stringify(value, null, 2);
  });

  // Custom filter to format code snippets or handle spaces
  eleventyConfig.addFilter("trim", function(value) {
    return value ? value.trim() : "";
  });

  return {
    dir: {
      input: "src",
      output: "_site",
      includes: "_includes"
    },
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk",
    dataTemplateEngine: "njk",
    pathPrefix: "/dartfx-semanticdt/"
  };
};
