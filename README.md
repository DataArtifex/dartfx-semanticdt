# dartfx-semanticdt

[![PyPI - Version](https://img.shields.io/pypi/v/dartfx-semanticdt.svg)](https://pypi.org/project/dartfx-semanticdt)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/dartfx-semanticdt.svg)](https://pypi.org/project/dartfx-semanticdt)
[![CI](https://github.com/DataArtifex/dartfx-semanticdt/actions/workflows/test.yml/badge.svg)](https://github.com/DataArtifex/dartfx-semanticdt/actions/workflows/test.yml)
[![License](https://img.shields.io/github/license/DataArtifex/dartfx-semanticdt.svg)](https://github.com/DataArtifex/dartfx-semanticdt/blob/main/LICENSE.txt)
[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit)](https://github.com/pre-commit/pre-commit)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)
[![DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/DataArtifex/dartfx-semanticdt)

> [!WARNING]
> **Early Release**: This project is in an early preview stage and is intended solely for development and testing purposes. It is not ready for production use.

**dartfx-semanticdt** is a LinkML based model designed to define, validate, generate, and document **Semantic Data Types (SDTs)**.

An SDT establishes a formal bridge between conceptual representations and physical storage or programming data types (such as `varchar` or `int`). By capturing key aspects—including meaning and purpose, usage, formatting, and validation rules—it enables both humans and machines to gain a deep, actionable understanding of the associated data.

The project also hosts a library of widely used semantic data types.

---

## Key Features

- **Conceptual Modeling**: Link programming variables to real-world concepts, ontologies, or URIs (aligned with SKOS).
- **Physical & Logical Mappings**: Map high-level data concepts to concrete storage types (e.g., XML Schema `xsd` types, SQL/PostgreSQL types, Python types) using controlled vocabularies.
- **Classification Systems**: Association with structured code lists or classifications (e.g., ICD-9, ISO-3166), turning the type into a categorical variable.
- **Validation Rules**: Define syntax structures via regular expressions (regex) or algorithm-based validators (such as the Luhn algorithm or modulo checksums).
- **Data Generation**: Directives and templates for generating syntactically and semantically valid synthetic data for testing and mock services.
- **Agentic Instructions**: Metadata explicitly designed to guide AI agents in how to parse, validate, and manipulate variables conforming to the SDT.
- **Display Formats**: Custom output masks, display templates, and UI constraints.

---

## Project Deliverables

This repository is organized around four primary deliverables:

1. **LinkML Model**: A formal [LinkML](https://linkml.io/) schema that defines the structure and metadata constraints of Semantic Data Types.
2. **Python Utilities**: A library for parsing SDT specifications, validating input data against SDT rules, and generating synthetic examples.
3. **Curated Global Library**: A collection of ready-to-use, standardized Semantic Data Type definitions (e.g., standard identifiers, currencies, classification codes).
4. **Static Portal**: An Eleventy (11ty)-based website hosting interactive documentation and the global library catalog.

---

## Installation

This project recommends using [uv](https://github.com/astral-sh/uv) for fast and reliable Python package and environment management.

### Development Environment (with uv)
```bash
# Clone the repository
git clone https://github.com/DataArtifex/dartfx-semanticdt.git
cd dartfx-semanticdt

# Install dependencies and set up virtual environment
uv sync

# Install git hooks
uv run pre-commit install
```

### Development Environment (with Hatch)
Alternatively, you can manage environments with [Hatch](https://hatch.pypa.io/):
```bash
# Run tests
hatch run test

# Enter the virtual environment shell
hatch shell
```

---

## Usage

*Note: The programmatic interface is in active development.*

Below is an illustrative overview of how you will be able to load, validate, and generate data using `dartfx-semanticdt`:

```python
from dartfx.semanticdt import SemanticDataType, Validator

# Load a Semantic Data Type definition (e.g., US Social Security Number)
ssn_sdt = SemanticDataType.from_yaml("library/us/ssn.yaml")

# Check if a value conforms to the Semantic Data Type rules
is_valid = Validator.validate(value="000-12-3456", sdt=ssn_sdt)
print(f"Is valid SSN: {is_valid}")  # True

# Generate valid synthetic example data
synthetic_data = sdt_generator.generate(sdt=ssn_sdt, count=5)
print(synthetic_data)
# ['123-45-6789', '987-65-4321', ...]
```

---

## Development & Testing

### Running Tests
```bash
uv run pytest
```

### Code Style & Formatting
We strictly follow Ruff guidelines. Make sure to format and lint your changes:
```bash
uv run ruff check --fix .
uv run ruff format .
```

### Type Checking
Run type checks using `pyrefly`:
```bash
uv run pyrefly check src tests
```

### Building Documentation
The documentation is built with Sphinx:
```bash
# Build documentation HTML
hatch run docs:build

# Serve documentation locally
hatch run docs:serve
```

---

## Roadmap

- [x] Define the LinkML schema for Semantic Data Types (`linkml/dev/sdt.yaml`).
- [ ] Implement core Pydantic models in Python representing the SDT schema.
- [ ] Develop data validation engine supporting regex and checksum algorithms.
- [ ] Build synthetic data generator following SDT generation patterns.
- [ ] Seed the global library with core types (SSN, ISBN, ZIP, ISO Country/Currency codes).
- [ ] Set up the Eleventy (11ty) static site generator portal.

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](./CONTRIBUTING.md) and our [Code of Conduct](./CODE_OF_CONDUCT.md) for guidelines on how to submit pull requests and contribute to the project.

---

## License

This project is licensed under the MIT License. See [LICENSE.txt](./LICENSE.txt) for details.
