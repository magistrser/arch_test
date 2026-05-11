#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessRule {
    NoParentAccess,
    NoModuleCyclicDependencies,
    NoLayerCyclicDependencies,
    MayOnlyAccess {
        accessor: String,
        accessed: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        when_same_parent: Option<bool>,
    },
    MayNotAccess {
        accessor: String,
        accessed: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        when_same_parent: Option<bool>,
    },
    MayOnlyBeAccessedBy {
        accessors: Vec<String>,
        accessed: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        when_same_parent: Option<bool>,
    },
    MayNotBeAccessedBy {
        accessors: Vec<String>,
        accessed: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        when_same_parent: Option<bool>,
    },
    Available {
        layer_names: Vec<String>,
        allowed_crates: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        when_same_parent: Option<bool>,
    },
    Restricted {
        layer_names: Vec<String>,
        restricted_crates: Vec<String>,
    },
}
