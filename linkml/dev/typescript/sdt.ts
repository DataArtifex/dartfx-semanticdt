export type SemanticDataTypeId = string;
/**
* Recommended vocabulary of target systems and development environments.
*/
export enum TargetEnvironmentEnum {

    /** C# programming environment. */
    csharp = "csharp",
    /** Platform-independent representation. */
    generic = "generic",
    /** Go programming environment. */
    go = "go",
    /** Java programming environment. */
    java = "java",
    /** JavaScript programming environment. */
    javascript = "javascript",
    /** Python programming environment. */
    python = "python",
    /** R statistical computing environment. */
    r = "r",
    /** Rust programming environment. */
    rust = "rust",
    /** SAS statistical computing environment. */
    sas = "sas",
    /** SPSS statistical computing environment. */
    spss = "spss",
    /** Standard Structured Query Language (ANSI). */
    sql = "sql",
    /** Google BigQuery database environment. */
    sql_bigquery = "sql-bigquery",
    /** ClickHouse database environment. */
    sql_clickhouse = "sql-clickhouse",
    /** Microsoft SQL Server database environment. */
    sql_mssql = "sql-mssql",
    /** MySQL database environment. */
    sql_mysql = "sql-mysql",
    /** Oracle database environment. */
    sql_oracle = "sql-oracle",
    /** PostgreSQL database environment. */
    sql_postgresql = "sql-postgresql",
    /** Amazon Redshift database environment. */
    sql_redshift = "sql-redshift",
    /** Snowflake database environment. */
    sql_snowflake = "sql-snowflake",
    /** SQLite database environment. */
    sql_sqlite = "sql-sqlite",
    /** Stata statistical software environment. */
    stata = "stata",
    /** TypeScript programming environment. */
    typescript = "typescript",
    /** XML Schema Definition. */
    xsd = "xsd",
};


/**
 * A high-level data type associated with a distinct concept, validation/generation rules, and metadata.
 */
export interface SemanticDataType {
    /** Unique identifier/URI for the Semantic Data Type. */
    id: string,
    /** Human-readable name of the Semantic Data Type. */
    name: string,
    /** A human-readable description of the Semantic Data Type. */
    description: string,
    /** The version of the Semantic Data Type instance definition (e.g., 1.0.0). */
    version: string,
    /** A list of conceptual ontology alignments or mapping references representing this data type in external systems (e.g., SKOS concepts, Wikidata items, or vocabularies like DDI-CDI and schema.org). This should list multiple representations of the same underlying semantic concept, not distinct concepts. */
    concepts?: ConceptReference[],
    /** Mappings to physical storage types across different environments. A mapping for the 'generic' environment is required to define the platform-independent base type. */
    storage_types?: StorageType[],
    /** Code lists or classification schemes that categorize this variable. */
    classification?: ClassificationSystem,
    /** Rules to validate physical representations of the data type. */
    validation_rules?: ValidationRule[],
    /** Templates or directives for generating synthetic valid data. */
    generation_rules?: GenerationRule[],
    /** Specific guidelines and instructions for AI agents handling this data type. */
    agent_instructions?: AgentInstruction,
    /** Tools, functions, or prompts associated with this data type. */
    agent_skills?: AgentSkill[],
    /** Contextual scope of the data type (temporal, geographic, etc.). */
    scope?: ScopeContext,
    /** Formatting rules for displaying values of this type. */
    display_format?: DisplayFormat,
    /** Curated valid and invalid examples for documentation and testing. */
    examples?: Example[],
}


/**
 * Reference to a concept in an external ontology or vocabulary.
 */
export interface ConceptReference {
    /** URI of the concept (e.g., SKOS URI). */
    uri: string,
    /** Name of the vocabulary or ontology (e.g., SKOS, DDI-CDI, Wikidata, LCSH). */
    vocabulary?: string,
    /** Preferred label of the concept in the ontology. */
    pref_label?: string,
    /** Relationship to the concept (e.g., exactMatch, closeMatch, broadMatch, narrowMatch). */
    match_type?: string,
}


/**
 * Physical or logical storage data type binding.
 */
export interface StorageType {
    /** Target platform, framework, database, or language. Prefer using values from the TargetEnvironmentEnum controlled vocabulary where applicable. Custom values not in the enum are permitted since this slot is of type string. */
    target_environment: string,
    /** Data type identifier in the target environment (e.g., string, varchar, integer, str). */
    data_type: string,
    /** Optional modifiers (e.g., length, precision). */
    format_modifier?: string,
}


/**
 * Controlled vocabulary or coding scheme (making this a categorical variable).
 */
export interface ClassificationSystem {
    /** Name of the classification (e.g., ISO-3166-1, ICD-10). */
    name: string,
    /** URI to the standard classification. */
    uri?: string,
    /** Version of the classification scheme. */
    version?: string,
}


/**
 * A rule defining syntactical or logical validation of values.
 */
export interface ValidationRule {
    /** Type of validation (e.g., regex, checksum, range). */
    rule_type: string,
    /** Regular expression pattern for validation. */
    pattern?: string,
    /** Algorithmic checker name (e.g., luhn, modulo-11, isbn13). */
    algorithm?: string,
    /** Error message when validation fails. */
    message?: string,
    /** Environment-specific code snippets or expressions implementing the validation logic. */
    code_snippets?: CodeSnippet[],
}


/**
 * A concrete piece of code or expression implementing logic (e.g., validation, generation) in a specific environment.
 */
export interface CodeSnippet {
    /** Target platform, framework, database, or language. Prefer using values from the TargetEnvironmentEnum controlled vocabulary where applicable. Custom values not in the enum are permitted since this slot is of type string. */
    target_environment: string,
    /** The actual code snippet or expression (e.g., Python function/lambda, SQL expression). */
    code: string,
}


/**
 * Directive for synthetic data generation.
 */
export interface GenerationRule {
    /** Type of generator (e.g., regex_fuzzer, template, faker_provider, custom). */
    generator_type: string,
    /** Seed template or expression. */
    template?: string,
    /** Environment-specific code snippets or expressions implementing the generator logic. */
    code_snippets?: CodeSnippet[],
}


/**
 * Machine and agentic parsing, transformation, or reasoning guidelines.
 */
export interface AgentInstruction {
    /** Text instructions for agents. */
    guidelines: string,
    /** System prompt instructions for configuring agents with this data type. */
    system_prompt_snippet?: string,
}


/**
 * Executable tools, functions, or workflows related to the semantic type.
 */
export interface AgentSkill {
    /** Name of the tool or function. */
    tool_name: string,
    /** What this skill does. */
    description?: string,
    /** API endpoint or function signature. */
    api_definition?: string,
}


/**
 * Contextual boundaries of the data type.
 */
export interface ScopeContext {
    /** Geospatial region or country coverage details. */
    geospatial_coverage?: GeospatialCoverage,
    /** Temporal coverage details. */
    temporal_coverage?: TemporalCoverage,
    /** Specific field or subject domain (e.g., Healthcare, Finance). */
    domain?: string,
    /** Governing body (e.g., ISO, W3C, IRS). */
    governing_institution?: string,
}


/**
 * Geospatial coverage boundaries represented by description and/or formal codes or URIs.
 */
export interface GeospatialCoverage {
    /** Human-readable description of the geospatial coverage (e.g., United States, Canada, Global). */
    description?: string,
    /** A list of formal codes or URIs (e.g., ISO country/subdivision codes, GeoNames URIs, Wikidata URIs). */
    codes?: string[],
}


/**
 * Temporal coverage boundaries represented by description, start/end dates, or ISO 8601 periods/durations.
 */
export interface TemporalCoverage {
    /** Human-readable description of the temporal coverage (e.g., '21st Century', '2020 Census'). */
    description?: string,
    /** Start date of the coverage formatted strictly according to ISO 8601 (YYYY-MM-DD). */
    start_date?: string,
    /** End date of the coverage formatted strictly according to ISO 8601 (YYYY-MM-DD). */
    end_date?: string,
    /** ISO 8601 time interval or period representation (e.g., '2020-01-01/2020-12-31', 'P1Y'). */
    iso_period?: string,
}


/**
 * Guidelines for displaying and masking data.
 */
export interface DisplayFormat {
    /** Mask for input forms (e.g., 999-99-9999). */
    input_mask?: string,
    /** Output representation template. */
    display_template?: string,
    /** Regex used to transform raw data to formatted data. */
    formatting_regex?: string,
}


/**
 * Concrete instance of values.
 */
export interface Example {
    /** Example value representation. */
    value: string,
    /** Indication of whether the example value is valid under the SDT. */
    is_valid: boolean,
    /** Description of the specific test case this example represents. */
    description?: string,
}
