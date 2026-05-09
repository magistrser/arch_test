use std::collections::HashSet;

use super::RuleScope;

/// # `Accessed` may only be accessed by `accessors` relation
/// This access rule relation states that the `accessors` layers may only access the specified `accessed` layer.
/// As layer name it attempts to match either the module name or the parent module name, which is the directory the files were placed in.
/// If `when_same_parent` is `true`, the access rule is only applied within the same scope of modules that share the same parent.
#[derive(Debug, Clone)]
pub struct MayOnlyBeAccessedBy {
    accessors: HashSet<String>,
    accessed: String,
    scope: RuleScope,
}

impl MayOnlyBeAccessedBy {
    pub fn new(accessed: String, accessor_layers: HashSet<String>, scope: RuleScope) -> Self {
        MayOnlyBeAccessedBy {
            accessors: accessor_layers,
            accessed,
            scope,
        }
    }

    pub fn accessors(&self) -> &HashSet<String> {
        &self.accessors
    }

    pub fn accessed(&self) -> &String {
        &self.accessed
    }
    
    pub fn scope(&self) -> &RuleScope {
        &self.scope
    }
}
