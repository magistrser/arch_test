use std::collections::HashSet;

use super::RuleScope;

/// # Available - white_list for libraries
/// This access rule restricts the use of external crates (including standard library)
/// to specific layers. Only the crates listed in `allowed_crates` can be used in modules
/// belonging to the specified `layer_names`.
///
/// If `when_same_parent` is `true`, the rule is only applied within modules that share
/// the same parent as the target layers.
#[derive(Debug, Clone)]
pub struct Available {
    layer_names: HashSet<String>,
    allowed_crates: HashSet<String>,
    scope: RuleScope,
}

impl Available {
    pub fn new(
        layer_names: HashSet<String>,
        allowed_crates: HashSet<String>,
        scope: RuleScope,
    ) -> Self {
        Available {
            layer_names,
            allowed_crates,
            scope,
        }
    }

    pub fn layer_names(&self) -> &HashSet<String> {
        &self.layer_names
    }

    pub fn allowed_crates(&self) -> &HashSet<String> {
        &self.allowed_crates
    }

    pub fn scope(&self) -> &RuleScope {
        &self.scope
    }
}
