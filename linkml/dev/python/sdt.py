from __future__ import annotations

import re
import sys
from datetime import (
    date,
    datetime,
    time
)
from decimal import Decimal
from enum import Enum
from typing import (
    Any,
    ClassVar,
    Literal,
    Optional,
    Union
)

from pydantic import (
    BaseModel,
    ConfigDict,
    Field,
    RootModel,
    SerializationInfo,
    SerializerFunctionWrapHandler,
    field_validator,
    model_serializer
)


metamodel_version = "1.11.0"
version = "0.1.0-dev"


class ConfiguredBaseModel(BaseModel):
    model_config = ConfigDict(
        serialize_by_alias = True,
        validate_by_name = True,
        validate_assignment = True,
        validate_default = True,
        extra = "forbid",
        arbitrary_types_allowed = True,
        use_enum_values = True,
        strict = False,
    )





class LinkMLMeta(RootModel):
    root: dict[str, Any] = {}
    model_config = ConfigDict(frozen=True)

    def __getattr__(self, key:str):
        return getattr(self.root, key)

    def __getitem__(self, key:str):
        return self.root[key]

    def __setitem__(self, key:str, value):
        self.root[key] = value

    def __contains__(self, key:str) -> bool:
        return key in self.root


linkml_meta = LinkMLMeta({'default_prefix': 'sdt',
     'default_range': 'string',
     'description': 'A LinkML schema defining Semantic Data Types (SDTs). An SDT '
                    'serves as an intelligent bridge between high-level data type '
                    'concepts and the ways they are commonly represented in files, '
                    'databases, and programming environments. By formalizing '
                    'metadata, mappings, validation, generation, and agentic '
                    'instructions, this model enables both humans and AI agents to '
                    'develop a deep, actionable understanding of the underlying '
                    'data.',
     'id': 'https://w3id.org/dartfx/semanticdt',
     'imports': ['linkml:types'],
     'license': 'https://creativecommons.org/publicdomain/zero/1.0/',
     'name': 'semantic-data-type',
     'prefixes': {'linkml': {'prefix_prefix': 'linkml',
                             'prefix_reference': 'https://w3id.org/linkml/'},
                  'sdt': {'prefix_prefix': 'sdt',
                          'prefix_reference': 'https://w3id.org/dartfx/semanticdt/'},
                  'skos': {'prefix_prefix': 'skos',
                           'prefix_reference': 'http://www.w3.org/2004/02/skos/core#'},
                  'xsd': {'prefix_prefix': 'xsd',
                          'prefix_reference': 'http://www.w3.org/2001/XMLSchema#'}},
     'source_file': '/Users/pascal/Library/CloudStorage/Dropbox/git-dartfx/dartfx-semanticdt/linkml/dev/sdt.yaml',
     'title': 'Semantic Data Type Schema'} )

class TargetEnvironmentEnum(str, Enum):
    """
    Recommended vocabulary of target systems and development environments.
    """
    csharp = "csharp"
    """
    C# programming environment.
    """
    generic = "generic"
    """
    Platform-independent representation.
    """
    go = "go"
    """
    Go programming environment.
    """
    java = "java"
    """
    Java programming environment.
    """
    javascript = "javascript"
    """
    JavaScript programming environment.
    """
    python = "python"
    """
    Python programming environment.
    """
    r = "r"
    """
    R statistical computing environment.
    """
    rust = "rust"
    """
    Rust programming environment.
    """
    sas = "sas"
    """
    SAS statistical computing environment.
    """
    spss = "spss"
    """
    SPSS statistical computing environment.
    """
    sql = "sql"
    """
    Standard Structured Query Language (ANSI).
    """
    sql_bigquery = "sql-bigquery"
    """
    Google BigQuery database environment.
    """
    sql_clickhouse = "sql-clickhouse"
    """
    ClickHouse database environment.
    """
    sql_mssql = "sql-mssql"
    """
    Microsoft SQL Server database environment.
    """
    sql_mysql = "sql-mysql"
    """
    MySQL database environment.
    """
    sql_oracle = "sql-oracle"
    """
    Oracle database environment.
    """
    sql_postgresql = "sql-postgresql"
    """
    PostgreSQL database environment.
    """
    sql_redshift = "sql-redshift"
    """
    Amazon Redshift database environment.
    """
    sql_snowflake = "sql-snowflake"
    """
    Snowflake database environment.
    """
    sql_sqlite = "sql-sqlite"
    """
    SQLite database environment.
    """
    stata = "stata"
    """
    Stata statistical software environment.
    """
    typescript = "typescript"
    """
    TypeScript programming environment.
    """
    xsd = "xsd"
    """
    XML Schema Definition.
    """



class SemanticDataType(ConfiguredBaseModel):
    """
    A high-level data type associated with a distinct concept, validation/generation rules, and metadata.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    id: str = Field(default=..., description="""Unique identifier/URI for the Semantic Data Type.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    name: str = Field(default=..., description="""Human-readable name of the Semantic Data Type.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType', 'ClassificationSystem']} })
    description: str = Field(default=..., description="""A human-readable description of the Semantic Data Type.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType',
                       'AgentSkill',
                       'GeospatialCoverage',
                       'TemporalCoverage',
                       'Example',
                       'Resource']} })
    version: str = Field(default=..., description="""The version of the Semantic Data Type instance definition (e.g., 1.0.0).""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType', 'ClassificationSystem']} })
    concepts: Optional[list[ConceptReference]] = Field(default=None, description="""A list of conceptual ontology alignments or mapping references representing this data type in external systems (e.g., SKOS concepts, Wikidata items, or vocabularies like DDI-CDI and schema.org). This should list multiple representations of the same underlying semantic concept, not distinct concepts.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    storage_types: Optional[list[StorageType]] = Field(default=None, description="""Mappings to physical storage types across different environments. A mapping for the 'generic' environment is required to define the platform-independent base type.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    classification: Optional[ClassificationSystem] = Field(default=None, description="""Code lists or classification schemes that categorize this variable.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    validation_rules: Optional[list[ValidationRule]] = Field(default=None, description="""Rules to validate physical representations of the data type.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    generation_rules: Optional[list[GenerationRule]] = Field(default=None, description="""Templates or directives for generating synthetic valid data.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    agent_instructions: Optional[AgentInstruction] = Field(default=None, description="""Specific guidelines and instructions for AI agents handling this data type.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    agent_skills: Optional[list[AgentSkill]] = Field(default=None, description="""Tools, functions, or prompts associated with this data type.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    scope: Optional[ScopeContext] = Field(default=None, description="""Contextual scope of the data type (temporal, geographic, etc.).""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    display_format: Optional[DisplayFormat] = Field(default=None, description="""Formatting rules for displaying values of this type.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    examples: Optional[list[Example]] = Field(default=None, description="""Curated valid and invalid examples for documentation and testing.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })
    resources: Optional[list[Resource]] = Field(default=None, description="""External resources and references related to the Semantic Data Type.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType']} })


class ConceptReference(ConfiguredBaseModel):
    """
    Reference to a concept in an external ontology or vocabulary.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    uri: str = Field(default=..., description="""URI of the concept (e.g., SKOS URI).""", json_schema_extra = { "linkml_meta": {'domain_of': ['ConceptReference', 'ClassificationSystem']} })
    vocabulary: Optional[str] = Field(default=None, description="""Name of the vocabulary or ontology (e.g., SKOS, DDI-CDI, Wikidata, LCSH).""", json_schema_extra = { "linkml_meta": {'domain_of': ['ConceptReference']} })
    pref_label: Optional[str] = Field(default=None, description="""Preferred label of the concept in the ontology.""", json_schema_extra = { "linkml_meta": {'domain_of': ['ConceptReference']} })
    match_type: Optional[str] = Field(default="exactMatch", description="""Relationship to the concept (e.g., exactMatch, closeMatch, broadMatch, narrowMatch).""", json_schema_extra = { "linkml_meta": {'domain_of': ['ConceptReference'], 'ifabsent': 'string(exactMatch)'} })


class StorageType(ConfiguredBaseModel):
    """
    Physical or logical storage data type binding.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    target_environment: str = Field(default=..., description="""Target platform, framework, database, or language. Prefer using values from the TargetEnvironmentEnum controlled vocabulary where applicable. Custom values not in the enum are permitted since this slot is of type string.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StorageType', 'CodeSnippet']} })
    data_type: str = Field(default=..., description="""Data type identifier in the target environment (e.g., string, varchar, integer, str).""", json_schema_extra = { "linkml_meta": {'domain_of': ['StorageType']} })
    format_modifier: Optional[str] = Field(default=None, description="""Optional modifiers (e.g., length, precision).""", json_schema_extra = { "linkml_meta": {'domain_of': ['StorageType']} })


class ClassificationSystem(ConfiguredBaseModel):
    """
    Controlled vocabulary or coding scheme (making this a categorical variable).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    name: str = Field(default=..., description="""Name of the classification (e.g., ISO-3166-1, ICD-10).""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType', 'ClassificationSystem']} })
    uri: Optional[str] = Field(default=None, description="""URI to the standard classification.""", json_schema_extra = { "linkml_meta": {'domain_of': ['ConceptReference', 'ClassificationSystem']} })
    version: Optional[str] = Field(default=None, description="""Version of the classification scheme.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType', 'ClassificationSystem']} })


class ValidationRule(ConfiguredBaseModel):
    """
    A rule defining syntactical or logical validation of values.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    rule_type: str = Field(default=..., description="""Type of validation (e.g., regex, checksum, range).""", json_schema_extra = { "linkml_meta": {'domain_of': ['ValidationRule']} })
    pattern: Optional[str] = Field(default=None, description="""Regular expression pattern for validation.""", json_schema_extra = { "linkml_meta": {'domain_of': ['ValidationRule']} })
    algorithm: Optional[str] = Field(default=None, description="""Algorithmic checker name (e.g., luhn, modulo-11, isbn13).""", json_schema_extra = { "linkml_meta": {'domain_of': ['ValidationRule']} })
    message: Optional[str] = Field(default=None, description="""Error message when validation fails.""", json_schema_extra = { "linkml_meta": {'domain_of': ['ValidationRule']} })
    code_snippets: Optional[list[CodeSnippet]] = Field(default=None, description="""Environment-specific code snippets or expressions implementing the validation logic.""", json_schema_extra = { "linkml_meta": {'domain_of': ['ValidationRule', 'GenerationRule']} })


class CodeSnippet(ConfiguredBaseModel):
    """
    A concrete piece of code or expression implementing logic (e.g., validation, generation) in a specific environment.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    target_environment: str = Field(default=..., description="""Target platform, framework, database, or language. Prefer using values from the TargetEnvironmentEnum controlled vocabulary where applicable. Custom values not in the enum are permitted since this slot is of type string.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StorageType', 'CodeSnippet']} })
    code: str = Field(default=..., description="""The actual code snippet or expression (e.g., Python function/lambda, SQL expression).""", json_schema_extra = { "linkml_meta": {'domain_of': ['CodeSnippet']} })


class GenerationRule(ConfiguredBaseModel):
    """
    Directive for synthetic data generation.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    generator_type: str = Field(default=..., description="""Type of generator (e.g., regex_fuzzer, template, faker_provider, custom).""", json_schema_extra = { "linkml_meta": {'domain_of': ['GenerationRule']} })
    template: Optional[str] = Field(default=None, description="""Seed template or expression.""", json_schema_extra = { "linkml_meta": {'domain_of': ['GenerationRule']} })
    code_snippets: Optional[list[CodeSnippet]] = Field(default=None, description="""Environment-specific code snippets or expressions implementing the generator logic.""", json_schema_extra = { "linkml_meta": {'domain_of': ['ValidationRule', 'GenerationRule']} })


class AgentInstruction(ConfiguredBaseModel):
    """
    Machine and agentic parsing, transformation, or reasoning guidelines.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    guidelines: str = Field(default=..., description="""Text instructions for agents.""", json_schema_extra = { "linkml_meta": {'domain_of': ['AgentInstruction']} })
    system_prompt_snippet: Optional[str] = Field(default=None, description="""System prompt instructions for configuring agents with this data type.""", json_schema_extra = { "linkml_meta": {'domain_of': ['AgentInstruction']} })


class AgentSkill(ConfiguredBaseModel):
    """
    Executable tools, functions, or workflows related to the semantic type.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    tool_name: str = Field(default=..., description="""Name of the tool or function.""", json_schema_extra = { "linkml_meta": {'domain_of': ['AgentSkill']} })
    description: Optional[str] = Field(default=None, description="""What this skill does.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType',
                       'AgentSkill',
                       'GeospatialCoverage',
                       'TemporalCoverage',
                       'Example',
                       'Resource']} })
    api_definition: Optional[str] = Field(default=None, description="""API endpoint or function signature.""", json_schema_extra = { "linkml_meta": {'domain_of': ['AgentSkill']} })


class ScopeContext(ConfiguredBaseModel):
    """
    Contextual boundaries of the data type.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    geospatial_coverage: Optional[GeospatialCoverage] = Field(default=None, description="""Geospatial region or country coverage details.""", json_schema_extra = { "linkml_meta": {'domain_of': ['ScopeContext']} })
    temporal_coverage: Optional[TemporalCoverage] = Field(default=None, description="""Temporal coverage details.""", json_schema_extra = { "linkml_meta": {'domain_of': ['ScopeContext']} })
    domain: Optional[str] = Field(default=None, description="""Specific field or subject domain (e.g., Healthcare, Finance).""", json_schema_extra = { "linkml_meta": {'domain_of': ['ScopeContext']} })
    governing_institution: Optional[str] = Field(default=None, description="""Governing body (e.g., ISO, W3C, IRS).""", json_schema_extra = { "linkml_meta": {'domain_of': ['ScopeContext']} })


class GeospatialCoverage(ConfiguredBaseModel):
    """
    Geospatial coverage boundaries represented by description and/or formal codes or URIs.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    description: Optional[str] = Field(default=None, description="""Human-readable description of the geospatial coverage (e.g., United States, Canada, Global).""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType',
                       'AgentSkill',
                       'GeospatialCoverage',
                       'TemporalCoverage',
                       'Example',
                       'Resource']} })
    codes: Optional[list[str]] = Field(default=None, description="""A list of formal codes or URIs (e.g., ISO country/subdivision codes, GeoNames URIs, Wikidata URIs).""", json_schema_extra = { "linkml_meta": {'domain_of': ['GeospatialCoverage']} })


class TemporalCoverage(ConfiguredBaseModel):
    """
    Temporal coverage boundaries represented by description, start/end dates, or ISO 8601 periods/durations.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    description: Optional[str] = Field(default=None, description="""Human-readable description of the temporal coverage (e.g., '21st Century', '2020 Census').""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType',
                       'AgentSkill',
                       'GeospatialCoverage',
                       'TemporalCoverage',
                       'Example',
                       'Resource']} })
    start_date: Optional[str] = Field(default=None, description="""Start date of the coverage formatted strictly according to ISO 8601 (YYYY-MM-DD).""", json_schema_extra = { "linkml_meta": {'domain_of': ['TemporalCoverage'], 'is_a': 'iso_date'} })
    end_date: Optional[str] = Field(default=None, description="""End date of the coverage formatted strictly according to ISO 8601 (YYYY-MM-DD).""", json_schema_extra = { "linkml_meta": {'domain_of': ['TemporalCoverage'], 'is_a': 'iso_date'} })
    iso_period: Optional[str] = Field(default=None, description="""ISO 8601 time interval or period representation (e.g., '2020-01-01/2020-12-31', 'P1Y').""", json_schema_extra = { "linkml_meta": {'domain_of': ['TemporalCoverage']} })

    @field_validator('start_date')
    def pattern_start_date(cls, v):
        pattern=re.compile(r"^\d{4}-\d{2}-\d{2}$")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid start_date format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid start_date format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('end_date')
    def pattern_end_date(cls, v):
        pattern=re.compile(r"^\d{4}-\d{2}-\d{2}$")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid end_date format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid end_date format: {v}"
            raise ValueError(err_msg)
        return v


class DisplayFormat(ConfiguredBaseModel):
    """
    Guidelines for displaying and masking data.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    input_mask: Optional[str] = Field(default=None, description="""Mask for input forms (e.g., 999-99-9999).""", json_schema_extra = { "linkml_meta": {'domain_of': ['DisplayFormat']} })
    display_template: Optional[str] = Field(default=None, description="""Output representation template.""", json_schema_extra = { "linkml_meta": {'domain_of': ['DisplayFormat']} })
    formatting_regex: Optional[str] = Field(default=None, description="""Regex used to transform raw data to formatted data.""", json_schema_extra = { "linkml_meta": {'domain_of': ['DisplayFormat']} })


class Example(ConfiguredBaseModel):
    """
    Concrete instance of values.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    value: str = Field(default=..., description="""Example value representation.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Example']} })
    is_valid: bool = Field(default=..., description="""Indication of whether the example value is valid under the SDT.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Example']} })
    description: Optional[str] = Field(default=None, description="""Description of the specific test case this example represents.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType',
                       'AgentSkill',
                       'GeospatialCoverage',
                       'TemporalCoverage',
                       'Example',
                       'Resource']} })


class Resource(ConfiguredBaseModel):
    """
    An external resource, citation, or reference related to the Semantic Data Type (using simple Dublin Core elements).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://w3id.org/dartfx/semanticdt'})

    url: str = Field(default=..., description="""The URL/URI of the resource.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Resource']} })
    title: Optional[str] = Field(default=None, description="""Human-readable name of the resource.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Resource']} })
    description: Optional[str] = Field(default=None, description="""A brief summary or account of the resource.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SemanticDataType',
                       'AgentSkill',
                       'GeospatialCoverage',
                       'TemporalCoverage',
                       'Example',
                       'Resource']} })
    citation: Optional[str] = Field(default=None, description="""A formal bibliographic citation for the resource.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Resource']} })
    publisher: Optional[str] = Field(default=None, description="""The entity or publisher responsible for making the resource available.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Resource']} })


# Model rebuild
# see https://pydantic-docs.helpmanual.io/usage/models/#rebuilding-a-model
SemanticDataType.model_rebuild()
ConceptReference.model_rebuild()
StorageType.model_rebuild()
ClassificationSystem.model_rebuild()
ValidationRule.model_rebuild()
CodeSnippet.model_rebuild()
GenerationRule.model_rebuild()
AgentInstruction.model_rebuild()
AgentSkill.model_rebuild()
ScopeContext.model_rebuild()
GeospatialCoverage.model_rebuild()
TemporalCoverage.model_rebuild()
DisplayFormat.model_rebuild()
Example.model_rebuild()
Resource.model_rebuild()
