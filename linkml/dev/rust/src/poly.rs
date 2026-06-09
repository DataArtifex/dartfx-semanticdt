#![allow(non_camel_case_types)]

use crate::*;
use crate::poly_containers::*;


pub trait SemanticDataType   {

    fn id<'a>(&'a self) -> &'a str;
    // fn id_mut(&mut self) -> &mut &'a str;
    // fn set_id(&mut self, value: String);

    fn name<'a>(&'a self) -> &'a str;
    // fn name_mut(&mut self) -> &mut &'a str;
    // fn set_name(&mut self, value: String);

    fn description<'a>(&'a self) -> &'a str;
    // fn description_mut(&mut self) -> &mut &'a str;
    // fn set_description(&mut self, value: String);

    fn version<'a>(&'a self) -> &'a str;
    // fn version_mut(&mut self) -> &mut &'a str;
    // fn set_version(&mut self, value: String);

    fn concepts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ConceptReference>>;
    // fn concepts_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::ConceptReference>>;
    // fn set_concepts<E>(&mut self, value: Option<&Vec<E>>) where E: Into<ConceptReference>;

    fn storage_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::StorageType>>;
    // fn storage_types_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::StorageType>>;
    // fn set_storage_types<E>(&mut self, value: Option<&Vec<E>>) where E: Into<StorageType>;

    fn classification<'a>(&'a self) -> Option<&'a crate::ClassificationSystem>;
    // fn classification_mut(&mut self) -> &mut Option<&'a crate::ClassificationSystem>;
    // fn set_classification<E>(&mut self, value: Option<E>) where E: Into<ClassificationSystem>;

    fn validation_rules<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ValidationRule>>;
    // fn validation_rules_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::ValidationRule>>;
    // fn set_validation_rules<E>(&mut self, value: Option<&Vec<E>>) where E: Into<ValidationRule>;

    fn generation_rules<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::GenerationRule>>;
    // fn generation_rules_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::GenerationRule>>;
    // fn set_generation_rules<E>(&mut self, value: Option<&Vec<E>>) where E: Into<GenerationRule>;

    fn agent_instructions<'a>(&'a self) -> Option<&'a crate::AgentInstruction>;
    // fn agent_instructions_mut(&mut self) -> &mut Option<&'a crate::AgentInstruction>;
    // fn set_agent_instructions<E>(&mut self, value: Option<E>) where E: Into<AgentInstruction>;

    fn agent_skills<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::AgentSkill>>;
    // fn agent_skills_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::AgentSkill>>;
    // fn set_agent_skills<E>(&mut self, value: Option<&Vec<E>>) where E: Into<AgentSkill>;

    fn scope<'a>(&'a self) -> Option<&'a crate::ScopeContext>;
    // fn scope_mut(&mut self) -> &mut Option<&'a crate::ScopeContext>;
    // fn set_scope<E>(&mut self, value: Option<E>) where E: Into<ScopeContext>;

    fn display_format<'a>(&'a self) -> Option<&'a crate::DisplayFormat>;
    // fn display_format_mut(&mut self) -> &mut Option<&'a crate::DisplayFormat>;
    // fn set_display_format<E>(&mut self, value: Option<E>) where E: Into<DisplayFormat>;

    fn examples<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Example>>;
    // fn examples_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Example>>;
    // fn set_examples<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Example>;


}

impl SemanticDataType for crate::SemanticDataType {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn name<'a>(&'a self) -> &'a str {
        return &self.name[..];
    }
        fn description<'a>(&'a self) -> &'a str {
        return &self.description[..];
    }
        fn version<'a>(&'a self) -> &'a str {
        return &self.version[..];
    }
        fn concepts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ConceptReference>> {
        return self.concepts.as_ref();
    }
        fn storage_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::StorageType>> {
        return self.storage_types.as_ref();
    }
        fn classification<'a>(&'a self) -> Option<&'a crate::ClassificationSystem> {
        return self.classification.as_ref();
    }
        fn validation_rules<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ValidationRule>> {
        return self.validation_rules.as_ref();
    }
        fn generation_rules<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::GenerationRule>> {
        return self.generation_rules.as_ref();
    }
        fn agent_instructions<'a>(&'a self) -> Option<&'a crate::AgentInstruction> {
        return self.agent_instructions.as_ref();
    }
        fn agent_skills<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::AgentSkill>> {
        return self.agent_skills.as_ref();
    }
        fn scope<'a>(&'a self) -> Option<&'a crate::ScopeContext> {
        return self.scope.as_ref();
    }
        fn display_format<'a>(&'a self) -> Option<&'a crate::DisplayFormat> {
        return self.display_format.as_ref();
    }
        fn examples<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Example>> {
        return self.examples.as_ref();
    }
}


pub trait ConceptReference   {

    fn uri<'a>(&'a self) -> &'a str;
    // fn uri_mut(&mut self) -> &mut &'a str;
    // fn set_uri(&mut self, value: String);

    fn vocabulary<'a>(&'a self) -> Option<&'a str>;
    // fn vocabulary_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_vocabulary(&mut self, value: Option<&'a str>);

    fn pref_label<'a>(&'a self) -> Option<&'a str>;
    // fn pref_label_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_pref_label(&mut self, value: Option<&'a str>);

    fn match_type<'a>(&'a self) -> Option<&'a str>;
    // fn match_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_match_type(&mut self, value: Option<&'a str>);


}

impl ConceptReference for crate::ConceptReference {
        fn uri<'a>(&'a self) -> &'a str {
        return &self.uri[..];
    }
        fn vocabulary<'a>(&'a self) -> Option<&'a str> {
        return self.vocabulary.as_deref();
    }
        fn pref_label<'a>(&'a self) -> Option<&'a str> {
        return self.pref_label.as_deref();
    }
        fn match_type<'a>(&'a self) -> Option<&'a str> {
        return self.match_type.as_deref();
    }
}


pub trait StorageType   {

    fn target_environment<'a>(&'a self) -> &'a str;
    // fn target_environment_mut(&mut self) -> &mut &'a str;
    // fn set_target_environment(&mut self, value: String);

    fn data_type<'a>(&'a self) -> &'a str;
    // fn data_type_mut(&mut self) -> &mut &'a str;
    // fn set_data_type(&mut self, value: String);

    fn format_modifier<'a>(&'a self) -> Option<&'a str>;
    // fn format_modifier_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_format_modifier(&mut self, value: Option<&'a str>);


}

impl StorageType for crate::StorageType {
        fn target_environment<'a>(&'a self) -> &'a str {
        return &self.target_environment[..];
    }
        fn data_type<'a>(&'a self) -> &'a str {
        return &self.data_type[..];
    }
        fn format_modifier<'a>(&'a self) -> Option<&'a str> {
        return self.format_modifier.as_deref();
    }
}


pub trait ClassificationSystem   {

    fn name<'a>(&'a self) -> &'a str;
    // fn name_mut(&mut self) -> &mut &'a str;
    // fn set_name(&mut self, value: String);

    fn uri<'a>(&'a self) -> Option<&'a str>;
    // fn uri_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_uri(&mut self, value: Option<&'a str>);

    fn version<'a>(&'a self) -> Option<&'a str>;
    // fn version_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_version(&mut self, value: Option<&'a str>);


}

impl ClassificationSystem for crate::ClassificationSystem {
        fn name<'a>(&'a self) -> &'a str {
        return &self.name[..];
    }
        fn uri<'a>(&'a self) -> Option<&'a str> {
        return self.uri.as_deref();
    }
        fn version<'a>(&'a self) -> Option<&'a str> {
        return self.version.as_deref();
    }
}


pub trait ValidationRule   {

    fn rule_type<'a>(&'a self) -> &'a str;
    // fn rule_type_mut(&mut self) -> &mut &'a str;
    // fn set_rule_type(&mut self, value: String);

    fn pattern<'a>(&'a self) -> Option<&'a str>;
    // fn pattern_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_pattern(&mut self, value: Option<&'a str>);

    fn algorithm<'a>(&'a self) -> Option<&'a str>;
    // fn algorithm_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_algorithm(&mut self, value: Option<&'a str>);

    fn message<'a>(&'a self) -> Option<&'a str>;
    // fn message_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_message(&mut self, value: Option<&'a str>);

    fn code_snippets<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::CodeSnippet>>;
    // fn code_snippets_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::CodeSnippet>>;
    // fn set_code_snippets<E>(&mut self, value: Option<&Vec<E>>) where E: Into<CodeSnippet>;


}

impl ValidationRule for crate::ValidationRule {
        fn rule_type<'a>(&'a self) -> &'a str {
        return &self.rule_type[..];
    }
        fn pattern<'a>(&'a self) -> Option<&'a str> {
        return self.pattern.as_deref();
    }
        fn algorithm<'a>(&'a self) -> Option<&'a str> {
        return self.algorithm.as_deref();
    }
        fn message<'a>(&'a self) -> Option<&'a str> {
        return self.message.as_deref();
    }
        fn code_snippets<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::CodeSnippet>> {
        return self.code_snippets.as_ref();
    }
}


pub trait CodeSnippet   {

    fn target_environment<'a>(&'a self) -> &'a str;
    // fn target_environment_mut(&mut self) -> &mut &'a str;
    // fn set_target_environment(&mut self, value: String);

    fn code<'a>(&'a self) -> &'a str;
    // fn code_mut(&mut self) -> &mut &'a str;
    // fn set_code(&mut self, value: String);


}

impl CodeSnippet for crate::CodeSnippet {
        fn target_environment<'a>(&'a self) -> &'a str {
        return &self.target_environment[..];
    }
        fn code<'a>(&'a self) -> &'a str {
        return &self.code[..];
    }
}


pub trait GenerationRule   {

    fn generator_type<'a>(&'a self) -> &'a str;
    // fn generator_type_mut(&mut self) -> &mut &'a str;
    // fn set_generator_type(&mut self, value: String);

    fn template<'a>(&'a self) -> Option<&'a str>;
    // fn template_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_template(&mut self, value: Option<&'a str>);

    fn code_snippets<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::CodeSnippet>>;
    // fn code_snippets_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::CodeSnippet>>;
    // fn set_code_snippets<E>(&mut self, value: Option<&Vec<E>>) where E: Into<CodeSnippet>;


}

impl GenerationRule for crate::GenerationRule {
        fn generator_type<'a>(&'a self) -> &'a str {
        return &self.generator_type[..];
    }
        fn template<'a>(&'a self) -> Option<&'a str> {
        return self.template.as_deref();
    }
        fn code_snippets<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::CodeSnippet>> {
        return self.code_snippets.as_ref();
    }
}


pub trait AgentInstruction   {

    fn guidelines<'a>(&'a self) -> &'a str;
    // fn guidelines_mut(&mut self) -> &mut &'a str;
    // fn set_guidelines(&mut self, value: String);

    fn system_prompt_snippet<'a>(&'a self) -> Option<&'a str>;
    // fn system_prompt_snippet_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_system_prompt_snippet(&mut self, value: Option<&'a str>);


}

impl AgentInstruction for crate::AgentInstruction {
        fn guidelines<'a>(&'a self) -> &'a str {
        return &self.guidelines[..];
    }
        fn system_prompt_snippet<'a>(&'a self) -> Option<&'a str> {
        return self.system_prompt_snippet.as_deref();
    }
}


pub trait AgentSkill   {

    fn tool_name<'a>(&'a self) -> &'a str;
    // fn tool_name_mut(&mut self) -> &mut &'a str;
    // fn set_tool_name(&mut self, value: String);

    fn description<'a>(&'a self) -> Option<&'a str>;
    // fn description_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_description(&mut self, value: Option<&'a str>);

    fn api_definition<'a>(&'a self) -> Option<&'a str>;
    // fn api_definition_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_api_definition(&mut self, value: Option<&'a str>);


}

impl AgentSkill for crate::AgentSkill {
        fn tool_name<'a>(&'a self) -> &'a str {
        return &self.tool_name[..];
    }
        fn description<'a>(&'a self) -> Option<&'a str> {
        return self.description.as_deref();
    }
        fn api_definition<'a>(&'a self) -> Option<&'a str> {
        return self.api_definition.as_deref();
    }
}


pub trait ScopeContext   {

    fn geospatial_coverage<'a>(&'a self) -> Option<&'a crate::GeospatialCoverage>;
    // fn geospatial_coverage_mut(&mut self) -> &mut Option<&'a crate::GeospatialCoverage>;
    // fn set_geospatial_coverage<E>(&mut self, value: Option<E>) where E: Into<GeospatialCoverage>;

    fn temporal_coverage<'a>(&'a self) -> Option<&'a crate::TemporalCoverage>;
    // fn temporal_coverage_mut(&mut self) -> &mut Option<&'a crate::TemporalCoverage>;
    // fn set_temporal_coverage<E>(&mut self, value: Option<E>) where E: Into<TemporalCoverage>;

    fn domain<'a>(&'a self) -> Option<&'a str>;
    // fn domain_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_domain(&mut self, value: Option<&'a str>);

    fn governing_institution<'a>(&'a self) -> Option<&'a str>;
    // fn governing_institution_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_governing_institution(&mut self, value: Option<&'a str>);


}

impl ScopeContext for crate::ScopeContext {
        fn geospatial_coverage<'a>(&'a self) -> Option<&'a crate::GeospatialCoverage> {
        return self.geospatial_coverage.as_ref();
    }
        fn temporal_coverage<'a>(&'a self) -> Option<&'a crate::TemporalCoverage> {
        return self.temporal_coverage.as_ref();
    }
        fn domain<'a>(&'a self) -> Option<&'a str> {
        return self.domain.as_deref();
    }
        fn governing_institution<'a>(&'a self) -> Option<&'a str> {
        return self.governing_institution.as_deref();
    }
}


pub trait GeospatialCoverage   {

    fn description<'a>(&'a self) -> Option<&'a str>;
    // fn description_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_description(&mut self, value: Option<&'a str>);

    fn codes<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn codes_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_codes(&mut self, value: Option<&Vec<String>>);


}

impl GeospatialCoverage for crate::GeospatialCoverage {
        fn description<'a>(&'a self) -> Option<&'a str> {
        return self.description.as_deref();
    }
        fn codes<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.codes.as_ref();
    }
}


pub trait TemporalCoverage   {

    fn description<'a>(&'a self) -> Option<&'a str>;
    // fn description_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_description(&mut self, value: Option<&'a str>);

    fn start_date<'a>(&'a self) -> Option<&'a str>;
    // fn start_date_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_start_date(&mut self, value: Option<&'a str>);

    fn end_date<'a>(&'a self) -> Option<&'a str>;
    // fn end_date_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_end_date(&mut self, value: Option<&'a str>);

    fn iso_period<'a>(&'a self) -> Option<&'a str>;
    // fn iso_period_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_iso_period(&mut self, value: Option<&'a str>);


}

impl TemporalCoverage for crate::TemporalCoverage {
        fn description<'a>(&'a self) -> Option<&'a str> {
        return self.description.as_deref();
    }
        fn start_date<'a>(&'a self) -> Option<&'a str> {
        return self.start_date.as_deref();
    }
        fn end_date<'a>(&'a self) -> Option<&'a str> {
        return self.end_date.as_deref();
    }
        fn iso_period<'a>(&'a self) -> Option<&'a str> {
        return self.iso_period.as_deref();
    }
}


pub trait DisplayFormat   {

    fn input_mask<'a>(&'a self) -> Option<&'a str>;
    // fn input_mask_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_input_mask(&mut self, value: Option<&'a str>);

    fn display_template<'a>(&'a self) -> Option<&'a str>;
    // fn display_template_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_display_template(&mut self, value: Option<&'a str>);

    fn formatting_regex<'a>(&'a self) -> Option<&'a str>;
    // fn formatting_regex_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_formatting_regex(&mut self, value: Option<&'a str>);


}

impl DisplayFormat for crate::DisplayFormat {
        fn input_mask<'a>(&'a self) -> Option<&'a str> {
        return self.input_mask.as_deref();
    }
        fn display_template<'a>(&'a self) -> Option<&'a str> {
        return self.display_template.as_deref();
    }
        fn formatting_regex<'a>(&'a self) -> Option<&'a str> {
        return self.formatting_regex.as_deref();
    }
}


pub trait Example   {

    fn value<'a>(&'a self) -> &'a str;
    // fn value_mut(&mut self) -> &mut &'a str;
    // fn set_value(&mut self, value: String);

    fn is_valid(&self) -> bool;
    // fn is_valid_mut(&mut self) -> &mut bool;
    // fn set_is_valid(&mut self, value: bool);

    fn description<'a>(&'a self) -> Option<&'a str>;
    // fn description_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_description(&mut self, value: Option<&'a str>);


}

impl Example for crate::Example {
        fn value<'a>(&'a self) -> &'a str {
        return &self.value[..];
    }
        fn is_valid(&self) -> bool {
        return self.is_valid;
    }
        fn description<'a>(&'a self) -> Option<&'a str> {
        return self.description.as_deref();
    }
}
