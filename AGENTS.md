# AI Agent Instructions

Welcome, fellow AI. This file provides context and instructions for working on this repository effectively.

## Project Specifications

The project focuses on Semantic Data Types (SDT).

A **Semantic Data Type (SDT)** is a high-level data type situated above traditional programming or database data types. It ties directly to a distinct real-world concept and contains rich metadata that constrains its definition, usage, validation, and generation. It is designed to be machine-readable and agent-actionable as a primary goal, while remaining fully understandable by humans.

This concept shares similarities with a DDI-CDI (Data Documentation Initiative - Cross Domain Integration) Represented Variable, but is more specialized, richer, and tailored for programmatic and AI agent consumption.

### Common Examples

- **Standard Identifiers**: US Social Security Number (SSN), ISBN, ZIP code, ISO Country Codes.
- **Classifications/Codes**: ICD-9/10 clinical codes, ISO-3166 country codes.
- **Scientific/Geospatial**: Latitude/Longitude coordinates, timestamps with timezone context.

### Characteristics & Capabilities

A Semantic Data Type is a structured specification that can define:

- **Conceptual Modeling**: Association with a specific semantic concept, ontology, or URI. Ideally aligned on SKOS.
- **Physical/Logical Storage Mappings**: Mapping to common storage data types across different environments (e.g., generic, standard XML Schema `xsd`, database types like SQL/PostgreSQL, programming languages like Python). These are mapped using classifiers or controlled vocabularies (CV).
- **Classification Systems**: Association with structured code lists or classifications (e.g., ICD-9, ISO-3166), turning the type into a categorical variable.
- **Validation Rules**:
  - Regular expressions (regex) for structural validation and pattern-based data extraction.
  - Multi-layered validation ranging from simple schema checks to complex algorithmic validations (e.g., Luhn checksum algorithm for credit card numbers, modulo-11 for ISBNs).
- **Data Generation Rules**: Directives and templates for generating syntactically and semantically valid synthetic data.
- **Human & Machine Instructions**: Custom documentation including human-readable descriptions, formatting guidelines, and clear agentic instructions on how to parse, transform, or reason with the type.
- **Agentic Skills**: Direct associations with tools, functions, or prompts that AI agents can execute to manipulate or validate the data type.
- **Contextual Scope/Coverage**: Definitions of the applicable scope, including geographic boundaries, temporal coverage, specific domains, or governing institutions.
- **Display & Presentation Formats**: Instructions for rendering the data, such as input masks, display formats, or specific formatting regexes.
- **Examples**: A curated collection of valid and invalid examples for testing, documentation, and agent training.

### Project Deliverables

To support the creation, validation, and usage of SDTs, this repository implements:

1. **LinkML Model**: A formal [LinkML (Linked Data Modeling Language)](https://linkml.io/) schema that defines the structure and metadata constraints of Semantic Data Types.
2. **Python Utilities**: A Python library providing functions for parsing, validating, generating, and converting SDT definitions, as well as executing verification checks on variables matching an SDT.
3. **Curated Global Library**: A collection of pre-defined, standardized Semantic Data Types (e.g., standard identifiers, currencies, codes) ready for reuse.
4. **Documentation & Portal (Eleventy/11ty)**: A static website built with 11ty that showcases the curated library, serves the schema, provides interactive documentation, and is hosted on GitHub Pages.

## Project Stack

- **Language**: Python 3.12+ (Strictly required)
- **Dependency Management & Workflow**: [uv](https://github.com/astral-sh/uv) (Recommended) and [Hatch](https://hatch.pypa.io/).
- **Linting & Formatting**: [Ruff](https://docs.astral.sh/ruff/) (extremely fast linter/formatter).
- **Git Hooks**: [pre-commit](https://pre-commit.com/) (ensures code quality before commits).
- **Testing**: [pytest](https://docs.pytest.org/) with [coverage](https://coverage.readthedocs.io/).
- **Documentation**: [Sphinx](https://www.sphinx-doc.org/) with [MyST-Parser](https://myst-parser.readthedocs.io/) (Markdown support) and [Read the Docs theme](https://sphinx-rtd-theme.readthedocs.io/).
- **Version Control**: Git.

## Bootstrapping a New Project

To rename the project and package from the template defaults:

1. Run `./rename.sh "new-project-name" "new_package_name"`
2. Run `uv sync` to refresh the environment.
3. **DeepWiki**: Register the new project at [DeepWiki.com](https://deepwiki.com/) to enable AI-optimized documentation indexing.

## Environment Management

This project uses `hatch` for environment management, but `uv` is preferred for speed.

- To run tests: `uv run pytest` or `hatch run test`
- To check types: `uv run pyrefly check src tests` or `hatch run types:check`
- To build docs: `uv run sphinx-build -b html docs/source docs/build/html` or `hatch run docs:build`

## Coding Standards

- Follow PEP 8.
- Use type hints for all public APIs.
- Docstrings should be in Google style or NumPy style (Sphinx compatible).
- Prefer `pathlib` over `os.path`.
- Prefer Pydantic for modeling over Python data classs or other similar package
- Prefer Pydantic for modeling over Python datar Pandas or other similar packages
- **Dependency note**: Pydantic and Polars are not yet listed in `pyproject.toml`. Add them as dependencies before writing code that uses them.
- Strictly follow the project's Ruff configuration. Run `uv run ruff check .` and `uv run ruff format .` to ensure compliance before submitting changes.
- Type checking is performed with `pyrefly`; run `uv run pyrefly check src tests` and resolve all errors before submitting.

## Testing Policy

- All new features must be accompanied by tests.
- Maintain or improve test coverage.
- Use `pytest` fixtures for setup/teardown.
- Tests are located in the `tests/` directory.

## Documentation Policy

- Documentation is located in the `docs/source` directory.
- Main documentation is in `.rst` or `.md` (via MyST).
- Keep `README.md` up to date with core installation and usage instructions.
- Keep a dedicated `IMPLEMENTATION.md` document up to date that describes the package/code technical implementation.
- Maintain `CHANGELOG.md` with every significant change, ensuring the latest version is always at the top using the version number as heading (e.g., `## [0.1.0]`). Use short, concise bullet points.

## Version Management

- This project uses **dynamic versioning** via Hatch.
- The source of truth for the version is located in: `src/dartfx/semanticdt/__about__.py`.
- To bump versions, modify that file manually or use `hatch version <segment>` (e.g., `hatch version minor`).
- Follow [Semantic Versioning (SemVer)](https://semver.org/).

## Secret Management

- **Local Development**: Use a `.env` file in the project root for local environment variables and secrets.
- **Loading**: Secrets are automatically loaded in tests via `tests/conftest.py` using `python-dotenv`.
- **Git Hygiene**: Never commit `.env` files. Ensure they are covered by `.gitignore`.
- **CI/CD**: Add secrets to GitHub Repository Secrets for use in GitHub Actions. Reference them in workflows as `${{ secrets.SECRET_NAME }}`.

## GitHub Actions CI/CD

- **CI**: Located in `.github/workflows/test.yml`. Runs tests and linting on push/PR to `main` across Ubuntu, macOS, and Windows.
- **Docs**: Located in `.github/workflows/sphinx.yaml`. Builds and deploys documentation to GitHub Pages on push to `main`.
- All workflows use `astral-sh/setup-uv` for fast execution and caching.

## Working with this Repo

1. **Analysis**: Always start by reviewing `pyproject.toml` and `src/` structure.
2. **Context**: Check `KIs/` (Knowledge Items) directory at the repo root if it exists — markdown files there contain domain-specific context and design decisions.
3. **Execution**: Use `uv` or `hatch` for running scripts and tests.
4. **Validation**: Always run `uv run ruff check .`, `uv run ruff format .`, and `uv run pytest` before finalizing changes.
5. **Contributions**: Follow the conventions in `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md`.
