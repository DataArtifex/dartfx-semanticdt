#![allow(non_camel_case_types)]

#[cfg(feature = "serde")]
mod serde_utils;
pub mod poly;
pub mod poly_containers;
#[cfg(feature = "stubgen")]
pub mod stub_utils;

#[cfg(feature = "serde")]
use serde_yml as _ ;
#[cfg(feature = "pyo3")]
use pyo3::{FromPyObject,prelude::*};
#[cfg(feature = "stubgen")]
use pyo3_stub_gen::{define_stub_info_gatherer,derive::gen_stub_pyclass,derive::gen_stub_pymethods};
#[cfg(feature = "serde")]
use serde::{Deserialize,Serialize,de::IntoDeserializer};
use serde_value::Value;
#[cfg(feature = "serde")]
use serde_path_to_error;
use std::collections::HashMap;
use std::collections::BTreeMap;

// Types

pub type string = String;
pub type integer = String;
pub type boolean = String;
pub type float = f64;
pub type double = f64;
pub type decimal = String;
pub type time = String;
pub type date = String;
pub type datetime = String;
pub type date_or_datetime = String;
pub type uriorcurie = String;
pub type curie = String;
pub type uri = String;
pub type ncname = String;
pub type objectidentifier = String;
pub type nodeidentifier = String;
pub type jsonpointer = String;
pub type jsonpath = String;
pub type sparqlpath = String;

// Slots

pub type target_environment = String;
pub type iso_date = String;
pub type id = String;
pub type name = String;
pub type description = String;
pub type version = String;
pub type concepts = Vec<ConceptReference>;
pub type storage_types = Vec<StorageType>;
pub type classification = ClassificationSystem;
pub type validation_rules = Vec<ValidationRule>;
pub type generation_rules = Vec<GenerationRule>;
pub type agent_instructions = AgentInstruction;
pub type agent_skills = Vec<AgentSkill>;
pub type scope = ScopeContext;
pub type display_format = DisplayFormat;
pub type examples = Vec<Example>;
pub type resources = Vec<Resource>;
pub type uri = String;
pub type vocabulary = String;
pub type pref_label = String;
pub type match_type = String;
pub type data_type = String;
pub type format_modifier = String;
pub type rule_type = String;
pub type pattern = String;
pub type algorithm = String;
pub type message = String;
pub type code_snippets = String;
pub type code = String;
pub type generator_type = String;
pub type template = String;
pub type guidelines = String;
pub type system_prompt_snippet = String;
pub type tool_name = String;
pub type api_definition = String;
pub type geospatial_coverage = GeospatialCoverage;
pub type temporal_coverage = TemporalCoverage;
pub type domain = String;
pub type governing_institution = String;
pub type codes = Vec<String>;
pub type start_date = String;
pub type end_date = String;
pub type iso_period = String;
pub type input_mask = String;
pub type display_template = String;
pub type formatting_regex = String;
pub type value = String;
pub type is_valid = bool;
pub type url = String;
pub type title = String;
pub type citation = String;
pub type publisher = String;

// Enums

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TargetEnvironmentEnum {
    Csharp,
    Generic,
    Go,
    Java,
    Javascript,
    Python,
    R,
    Rust,
    Sas,
    Spss,
    Sql,
    Sql-bigquery,
    Sql-clickhouse,
    Sql-mssql,
    Sql-mysql,
    Sql-oracle,
    Sql-postgresql,
    Sql-redshift,
    Sql-snowflake,
    Sql-sqlite,
    Stata,
    Typescript,
    Xsd,
}

impl core::fmt::Display for TargetEnvironmentEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TargetEnvironmentEnum::Csharp => f.write_str("csharp"),
            TargetEnvironmentEnum::Generic => f.write_str("generic"),
            TargetEnvironmentEnum::Go => f.write_str("go"),
            TargetEnvironmentEnum::Java => f.write_str("java"),
            TargetEnvironmentEnum::Javascript => f.write_str("javascript"),
            TargetEnvironmentEnum::Python => f.write_str("python"),
            TargetEnvironmentEnum::R => f.write_str("r"),
            TargetEnvironmentEnum::Rust => f.write_str("rust"),
            TargetEnvironmentEnum::Sas => f.write_str("sas"),
            TargetEnvironmentEnum::Spss => f.write_str("spss"),
            TargetEnvironmentEnum::Sql => f.write_str("sql"),
            TargetEnvironmentEnum::Sql-bigquery => f.write_str("sql-bigquery"),
            TargetEnvironmentEnum::Sql-clickhouse => f.write_str("sql-clickhouse"),
            TargetEnvironmentEnum::Sql-mssql => f.write_str("sql-mssql"),
            TargetEnvironmentEnum::Sql-mysql => f.write_str("sql-mysql"),
            TargetEnvironmentEnum::Sql-oracle => f.write_str("sql-oracle"),
            TargetEnvironmentEnum::Sql-postgresql => f.write_str("sql-postgresql"),
            TargetEnvironmentEnum::Sql-redshift => f.write_str("sql-redshift"),
            TargetEnvironmentEnum::Sql-snowflake => f.write_str("sql-snowflake"),
            TargetEnvironmentEnum::Sql-sqlite => f.write_str("sql-sqlite"),
            TargetEnvironmentEnum::Stata => f.write_str("stata"),
            TargetEnvironmentEnum::Typescript => f.write_str("typescript"),
            TargetEnvironmentEnum::Xsd => f.write_str("xsd"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for TargetEnvironmentEnum {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            TargetEnvironmentEnum::Csharp => "csharp",
            TargetEnvironmentEnum::Generic => "generic",
            TargetEnvironmentEnum::Go => "go",
            TargetEnvironmentEnum::Java => "java",
            TargetEnvironmentEnum::Javascript => "javascript",
            TargetEnvironmentEnum::Python => "python",
            TargetEnvironmentEnum::R => "r",
            TargetEnvironmentEnum::Rust => "rust",
            TargetEnvironmentEnum::Sas => "sas",
            TargetEnvironmentEnum::Spss => "spss",
            TargetEnvironmentEnum::Sql => "sql",
            TargetEnvironmentEnum::Sql-bigquery => "sql-bigquery",
            TargetEnvironmentEnum::Sql-clickhouse => "sql-clickhouse",
            TargetEnvironmentEnum::Sql-mssql => "sql-mssql",
            TargetEnvironmentEnum::Sql-mysql => "sql-mysql",
            TargetEnvironmentEnum::Sql-oracle => "sql-oracle",
            TargetEnvironmentEnum::Sql-postgresql => "sql-postgresql",
            TargetEnvironmentEnum::Sql-redshift => "sql-redshift",
            TargetEnvironmentEnum::Sql-snowflake => "sql-snowflake",
            TargetEnvironmentEnum::Sql-sqlite => "sql-sqlite",
            TargetEnvironmentEnum::Stata => "stata",
            TargetEnvironmentEnum::Typescript => "typescript",
            TargetEnvironmentEnum::Xsd => "xsd",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for TargetEnvironmentEnum {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "csharp" | "Csharp" => Ok(TargetEnvironmentEnum::Csharp),
                "generic" | "Generic" => Ok(TargetEnvironmentEnum::Generic),
                "go" | "Go" => Ok(TargetEnvironmentEnum::Go),
                "java" | "Java" => Ok(TargetEnvironmentEnum::Java),
                "javascript" | "Javascript" => Ok(TargetEnvironmentEnum::Javascript),
                "python" | "Python" => Ok(TargetEnvironmentEnum::Python),
                "r" | "R" => Ok(TargetEnvironmentEnum::R),
                "rust" | "Rust" => Ok(TargetEnvironmentEnum::Rust),
                "sas" | "Sas" => Ok(TargetEnvironmentEnum::Sas),
                "spss" | "Spss" => Ok(TargetEnvironmentEnum::Spss),
                "sql" | "Sql" => Ok(TargetEnvironmentEnum::Sql),
                "sql-bigquery" | "Sql-bigquery" => Ok(TargetEnvironmentEnum::Sql-bigquery),
                "sql-clickhouse" | "Sql-clickhouse" => Ok(TargetEnvironmentEnum::Sql-clickhouse),
                "sql-mssql" | "Sql-mssql" => Ok(TargetEnvironmentEnum::Sql-mssql),
                "sql-mysql" | "Sql-mysql" => Ok(TargetEnvironmentEnum::Sql-mysql),
                "sql-oracle" | "Sql-oracle" => Ok(TargetEnvironmentEnum::Sql-oracle),
                "sql-postgresql" | "Sql-postgresql" => Ok(TargetEnvironmentEnum::Sql-postgresql),
                "sql-redshift" | "Sql-redshift" => Ok(TargetEnvironmentEnum::Sql-redshift),
                "sql-snowflake" | "Sql-snowflake" => Ok(TargetEnvironmentEnum::Sql-snowflake),
                "sql-sqlite" | "Sql-sqlite" => Ok(TargetEnvironmentEnum::Sql-sqlite),
                "stata" | "Stata" => Ok(TargetEnvironmentEnum::Stata),
                "typescript" | "Typescript" => Ok(TargetEnvironmentEnum::Typescript),
                "xsd" | "Xsd" => Ok(TargetEnvironmentEnum::Xsd),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for TargetEnvironmentEnum: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(TargetEnvironmentEnum)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for TargetEnvironmentEnum {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['csharp', 'generic', 'go', 'java', 'javascript', 'python', 'r', 'rust', 'sas', 'spss', 'sql', 'sql-bigquery', 'sql-clickhouse', 'sql-mssql', 'sql-mysql', 'sql-oracle', 'sql-postgresql', 'sql-redshift', 'sql-snowflake', 'sql-sqlite', 'stata', 'typescript', 'xsd']",
            "typing".into(),
        )
    }
}

// Classes

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SemanticDataType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub concepts: Option<Vec<ConceptReference>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub storage_types: Option<Vec<StorageType>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub classification: Option<ClassificationSystem>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub validation_rules: Option<Vec<ValidationRule>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub generation_rules: Option<Vec<GenerationRule>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub agent_instructions: Option<AgentInstruction>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub agent_skills: Option<Vec<AgentSkill>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub scope: Option<ScopeContext>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub display_format: Option<DisplayFormat>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: Option<Vec<Example>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub resources: Option<Vec<Resource>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl SemanticDataType {
    #[new]
    #[pyo3(signature = (id, name, description, version, concepts=None, storage_types=None, classification=None, validation_rules=None, generation_rules=None, agent_instructions=None, agent_skills=None, scope=None, display_format=None, examples=None, resources=None))]
    pub fn new(id: String, name: String, description: String, version: String, concepts: Option<serde_utils::PyValue<Vec<ConceptReference>>>, storage_types: Option<serde_utils::PyValue<Vec<StorageType>>>, classification: Option<serde_utils::PyValue<ClassificationSystem>>, validation_rules: Option<serde_utils::PyValue<Vec<ValidationRule>>>, generation_rules: Option<serde_utils::PyValue<Vec<GenerationRule>>>, agent_instructions: Option<serde_utils::PyValue<AgentInstruction>>, agent_skills: Option<serde_utils::PyValue<Vec<AgentSkill>>>, scope: Option<serde_utils::PyValue<ScopeContext>>, display_format: Option<serde_utils::PyValue<DisplayFormat>>, examples: Option<serde_utils::PyValue<Vec<Example>>>, resources: Option<serde_utils::PyValue<Vec<Resource>>>) -> Self {
        let concepts = concepts.map(|v| v.into_inner());
        let storage_types = storage_types.map(|v| v.into_inner());
        let classification = classification.map(|v| v.into_inner());
        let validation_rules = validation_rules.map(|v| v.into_inner());
        let generation_rules = generation_rules.map(|v| v.into_inner());
        let agent_instructions = agent_instructions.map(|v| v.into_inner());
        let agent_skills = agent_skills.map(|v| v.into_inner());
        let scope = scope.map(|v| v.into_inner());
        let display_format = display_format.map(|v| v.into_inner());
        let examples = examples.map(|v| v.into_inner());
        let resources = resources.map(|v| v.into_inner());
        SemanticDataType{id, name, description, version, concepts, storage_types, classification, validation_rules, generation_rules, agent_instructions, agent_skills, scope, display_format, examples, resources}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SemanticDataType>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SemanticDataType> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SemanticDataType>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SemanticDataType",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for SemanticDataType {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a SemanticDataType from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ConceptReference {
    pub uri: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub vocabulary: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pref_label: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub match_type: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ConceptReference {
    #[new]
    #[pyo3(signature = (uri, vocabulary=None, pref_label=None, match_type=None))]
    pub fn new(uri: String, vocabulary: Option<String>, pref_label: Option<String>, match_type: Option<String>) -> Self {
        ConceptReference{uri, vocabulary, pref_label, match_type}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ConceptReference>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ConceptReference> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ConceptReference>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ConceptReference",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct StorageType {
    pub target_environment: String,
    pub data_type: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub format_modifier: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl StorageType {
    #[new]
    #[pyo3(signature = (target_environment, data_type, format_modifier=None))]
    pub fn new(target_environment: String, data_type: String, format_modifier: Option<String>) -> Self {
        StorageType{target_environment, data_type, format_modifier}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<StorageType>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<StorageType> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<StorageType>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StorageType",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ClassificationSystem {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub uri: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub version: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ClassificationSystem {
    #[new]
    #[pyo3(signature = (name, uri=None, version=None))]
    pub fn new(name: String, uri: Option<String>, version: Option<String>) -> Self {
        ClassificationSystem{name, uri, version}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ClassificationSystem>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ClassificationSystem> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ClassificationSystem>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ClassificationSystem",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ValidationRule {
    pub rule_type: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pattern: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub algorithm: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub message: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_snippets: Option<Vec<CodeSnippet>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ValidationRule {
    #[new]
    #[pyo3(signature = (rule_type, pattern=None, algorithm=None, message=None, code_snippets=None))]
    pub fn new(rule_type: String, pattern: Option<String>, algorithm: Option<String>, message: Option<String>, code_snippets: Option<serde_utils::PyValue<Vec<CodeSnippet>>>) -> Self {
        let code_snippets = code_snippets.map(|v| v.into_inner());
        ValidationRule{rule_type, pattern, algorithm, message, code_snippets}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ValidationRule>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ValidationRule> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ValidationRule>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ValidationRule",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct CodeSnippet {
    pub target_environment: String,
    pub code: String
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl CodeSnippet {
    #[new]
    #[pyo3(signature = (target_environment, code))]
    pub fn new(target_environment: String, code: String) -> Self {
        CodeSnippet{target_environment, code}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<CodeSnippet>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<CodeSnippet> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<CodeSnippet>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid CodeSnippet",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct GenerationRule {
    pub generator_type: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub template: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub code_snippets: Option<Vec<CodeSnippet>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl GenerationRule {
    #[new]
    #[pyo3(signature = (generator_type, template=None, code_snippets=None))]
    pub fn new(generator_type: String, template: Option<String>, code_snippets: Option<serde_utils::PyValue<Vec<CodeSnippet>>>) -> Self {
        let code_snippets = code_snippets.map(|v| v.into_inner());
        GenerationRule{generator_type, template, code_snippets}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<GenerationRule>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<GenerationRule> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<GenerationRule>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid GenerationRule",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AgentInstruction {
    pub guidelines: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub system_prompt_snippet: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl AgentInstruction {
    #[new]
    #[pyo3(signature = (guidelines, system_prompt_snippet=None))]
    pub fn new(guidelines: String, system_prompt_snippet: Option<String>) -> Self {
        AgentInstruction{guidelines, system_prompt_snippet}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AgentInstruction>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AgentInstruction> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AgentInstruction>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AgentInstruction",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AgentSkill {
    pub tool_name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub api_definition: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl AgentSkill {
    #[new]
    #[pyo3(signature = (tool_name, description=None, api_definition=None))]
    pub fn new(tool_name: String, description: Option<String>, api_definition: Option<String>) -> Self {
        AgentSkill{tool_name, description, api_definition}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AgentSkill>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AgentSkill> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AgentSkill>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AgentSkill",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ScopeContext {
    #[cfg_attr(feature = "serde", serde(default))]
    pub geospatial_coverage: Option<GeospatialCoverage>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub temporal_coverage: Option<TemporalCoverage>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub domain: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub governing_institution: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ScopeContext {
    #[new]
    #[pyo3(signature = (geospatial_coverage=None, temporal_coverage=None, domain=None, governing_institution=None))]
    pub fn new(geospatial_coverage: Option<serde_utils::PyValue<GeospatialCoverage>>, temporal_coverage: Option<serde_utils::PyValue<TemporalCoverage>>, domain: Option<String>, governing_institution: Option<String>) -> Self {
        let geospatial_coverage = geospatial_coverage.map(|v| v.into_inner());
        let temporal_coverage = temporal_coverage.map(|v| v.into_inner());
        ScopeContext{geospatial_coverage, temporal_coverage, domain, governing_institution}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ScopeContext>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ScopeContext> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ScopeContext>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ScopeContext",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct GeospatialCoverage {
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub codes: Option<Vec<String>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl GeospatialCoverage {
    #[new]
    #[pyo3(signature = (description=None, codes=None))]
    pub fn new(description: Option<String>, codes: Option<Vec<String>>) -> Self {
        GeospatialCoverage{description, codes}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<GeospatialCoverage>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<GeospatialCoverage> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<GeospatialCoverage>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid GeospatialCoverage",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct TemporalCoverage {
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub start_date: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub end_date: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub iso_period: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl TemporalCoverage {
    #[new]
    #[pyo3(signature = (description=None, start_date=None, end_date=None, iso_period=None))]
    pub fn new(description: Option<String>, start_date: Option<String>, end_date: Option<String>, iso_period: Option<String>) -> Self {
        TemporalCoverage{description, start_date, end_date, iso_period}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TemporalCoverage>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TemporalCoverage> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TemporalCoverage>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TemporalCoverage",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct DisplayFormat {
    #[cfg_attr(feature = "serde", serde(default))]
    pub input_mask: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub display_template: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub formatting_regex: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl DisplayFormat {
    #[new]
    #[pyo3(signature = (input_mask=None, display_template=None, formatting_regex=None))]
    pub fn new(input_mask: Option<String>, display_template: Option<String>, formatting_regex: Option<String>) -> Self {
        DisplayFormat{input_mask, display_template, formatting_regex}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<DisplayFormat>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<DisplayFormat> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<DisplayFormat>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid DisplayFormat",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Example {
    pub value: String,
    pub is_valid: bool,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Example {
    #[new]
    #[pyo3(signature = (value, is_valid, description=None))]
    pub fn new(value: String, is_valid: bool, description: Option<String>) -> Self {
        Example{value, is_valid, description}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Example>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Example> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Example>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Example",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Resource {
    pub url: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub citation: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub publisher: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Resource {
    #[new]
    #[pyo3(signature = (url, title=None, description=None, citation=None, publisher=None))]
    pub fn new(url: String, title: Option<String>, description: Option<String>, citation: Option<String>, publisher: Option<String>) -> Self {
        Resource{url, title, description, citation, publisher}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Resource>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Resource> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Resource>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Resource",
        ))
    }
}






#[cfg(feature = "stubgen")]
define_stub_info_gatherer!(stub_info);
