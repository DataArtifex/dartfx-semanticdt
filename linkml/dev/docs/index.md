# Semantic Data Type Schema

A schema defining Semantic Data Types (SDT) situated above physical database or programming types. Encompasses metadata, mappings, validation, generation, and agentic instructions.

URI: https://w3id.org/dartfx/semanticdt

Name: semantic-data-type



## Classes

| Class | Description |
| --- | --- |
| [AgentInstruction](AgentInstruction.md) | Machine and agentic parsing, transformation, or reasoning guidelines |
| [AgentSkill](AgentSkill.md) | Executable tools, functions, or workflows related to the semantic type |
| [ClassificationSystem](ClassificationSystem.md) | Controlled vocabulary or coding scheme (making this a categorical variable) |
| [CodeSnippet](CodeSnippet.md) | A concrete piece of code or expression implementing logic (e |
| [ConceptReference](ConceptReference.md) | Reference to a concept in an external ontology or vocabulary |
| [DisplayFormat](DisplayFormat.md) | Guidelines for displaying and masking data |
| [Example](Example.md) | Concrete instance of values |
| [GenerationRule](GenerationRule.md) | Directive for synthetic data generation |
| [GeospatialCoverage](GeospatialCoverage.md) | Geospatial coverage boundaries represented by description and/or formal codes... |
| [Resource](Resource.md) | An external resource, citation, or reference related to the Semantic Data Typ... |
| [ScopeContext](ScopeContext.md) | Contextual boundaries of the data type |
| [SemanticDataType](SemanticDataType.md) | A high-level data type associated with a distinct concept, validation/generat... |
| [StorageType](StorageType.md) | Physical or logical storage data type binding |
| [TemporalCoverage](TemporalCoverage.md) | Temporal coverage boundaries represented by description, start/end dates, or ... |
| [ValidationRule](ValidationRule.md) | A rule defining syntactical or logical validation of values |



## Slots

| Slot | Description |
| --- | --- |
| [agent_instructions](agent_instructions.md) | Specific guidelines and instructions for AI agents handling this data type |
| [agent_skills](agent_skills.md) | Tools, functions, or prompts associated with this data type |
| [algorithm](algorithm.md) | Algorithmic checker name (e |
| [api_definition](api_definition.md) | API endpoint or function signature |
| [citation](citation.md) | A formal bibliographic citation for the resource |
| [classification](classification.md) | Code lists or classification schemes that categorize this variable |
| [code](code.md) | The actual code snippet or expression (e |
| [code_snippets](code_snippets.md) | Environment-specific code snippets or expressions implementing the validation... |
| [codes](codes.md) | A list of formal codes or URIs (e |
| [concepts](concepts.md) | A list of conceptual ontology alignments or mapping references representing t... |
| [data_type](data_type.md) | Data type identifier in the target environment (e |
| [description](description.md) | A human-readable description of the Semantic Data Type |
| [display_format](display_format.md) | Formatting rules for displaying values of this type |
| [display_template](display_template.md) | Output representation template |
| [domain](domain.md) | Specific field or subject domain (e |
| [end_date](end_date.md) | End date of the coverage formatted strictly according to ISO 8601 (YYYY-MM-DD... |
| [examples](examples.md) | Curated valid and invalid examples for documentation and testing |
| [format_modifier](format_modifier.md) | Optional modifiers (e |
| [formatting_regex](formatting_regex.md) | Regex used to transform raw data to formatted data |
| [generation_rules](generation_rules.md) | Templates or directives for generating synthetic valid data |
| [generator_type](generator_type.md) | Type of generator (e |
| [geospatial_coverage](geospatial_coverage.md) | Geospatial region or country coverage details |
| [governing_institution](governing_institution.md) | Governing body (e |
| [guidelines](guidelines.md) | Text instructions for agents |
| [id](id.md) | Unique identifier/URI for the Semantic Data Type |
| [input_mask](input_mask.md) | Mask for input forms (e |
| [is_valid](is_valid.md) | Indication of whether the example value is valid under the SDT |
| [iso_date](iso_date.md) | A date string formatted strictly according to ISO 8601 (YYYY-MM-DD) |
| [iso_period](iso_period.md) | ISO 8601 time interval or period representation (e |
| [match_type](match_type.md) | Relationship to the concept (e |
| [message](message.md) | Error message when validation fails |
| [name](name.md) | Human-readable name of the Semantic Data Type |
| [pattern](pattern.md) | Regular expression pattern for validation |
| [pref_label](pref_label.md) | Preferred label of the concept in the ontology |
| [publisher](publisher.md) | The entity or publisher responsible for making the resource available |
| [resources](resources.md) | External resources and references related to the Semantic Data Type |
| [rule_type](rule_type.md) | Type of validation (e |
| [scope](scope.md) | Contextual scope of the data type (temporal, geographic, etc |
| [start_date](start_date.md) | Start date of the coverage formatted strictly according to ISO 8601 (YYYY-MM-... |
| [storage_types](storage_types.md) | Mappings to physical storage types across different environments |
| [system_prompt_snippet](system_prompt_snippet.md) | System prompt instructions for configuring agents with this data type |
| [target_environment](target_environment.md) | Target platform, framework, database, or language |
| [template](template.md) | Seed template or expression |
| [temporal_coverage](temporal_coverage.md) | Temporal coverage details |
| [title](title.md) | Human-readable name of the resource |
| [tool_name](tool_name.md) | Name of the tool or function |
| [uri](uri.md) | URI of the concept (e |
| [url](url.md) | The URL/URI of the resource |
| [validation_rules](validation_rules.md) | Rules to validate physical representations of the data type |
| [value](value.md) | Example value representation |
| [version](version.md) | The version of the Semantic Data Type instance definition (e |
| [vocabulary](vocabulary.md) | Name of the vocabulary or ontology (e |


## Enumerations

| Enumeration | Description |
| --- | --- |
| [TargetEnvironmentEnum](TargetEnvironmentEnum.md) | Recommended vocabulary of target systems and development environments |


## Types

| Type | Description |
| --- | --- |
| [Boolean](Boolean.md) | A binary (true or false) value |
| [Curie](Curie.md) | a compact URI |
| [Date](Date.md) | a date (year, month and day) in an idealized calendar |
| [DateOrDatetime](DateOrDatetime.md) | Either a date or a datetime |
| [Datetime](Datetime.md) | The combination of a date and time |
| [Decimal](Decimal.md) | A real number with arbitrary precision that conforms to the xsd:decimal speci... |
| [Double](Double.md) | A real number that conforms to the xsd:double specification |
| [Float](Float.md) | A real number that conforms to the xsd:float specification |
| [Integer](Integer.md) | An integer |
| [Jsonpath](Jsonpath.md) | A string encoding a JSON Path |
| [Jsonpointer](Jsonpointer.md) | A string encoding a JSON Pointer |
| [Ncname](Ncname.md) | Prefix part of CURIE |
| [Nodeidentifier](Nodeidentifier.md) | A URI, CURIE or BNODE that represents a node in a model |
| [Objectidentifier](Objectidentifier.md) | A URI or CURIE that represents an object in the model |
| [Sparqlpath](Sparqlpath.md) | A string encoding a SPARQL Property Path |
| [String](String.md) | A character string |
| [Time](Time.md) | A time object represents a (local) time of day, independent of any particular... |
| [Uri](Uri.md) | a complete URI |
| [Uriorcurie](Uriorcurie.md) | a URI or a CURIE |


## Subsets

| Subset | Description |
| --- | --- |
