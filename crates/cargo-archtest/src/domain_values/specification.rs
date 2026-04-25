use crate::domain_values::AccessRule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    pub layer_names: Vec<String>,
    pub access_rules: Vec<AccessRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_modules: Option<Vec<String>>,
}
