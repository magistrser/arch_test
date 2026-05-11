use std::collections::HashSet;

/// # Restricted — black_list for libraries
/// This access rule forbids the use of specified external crates in specific layers.
/// Any crate listed in `restricted_crates` that is imported by a module belonging
/// to one of the `layer_names` triggers a violation.
#[derive(Debug, Clone)]
pub struct Restricted {
    layer_names: HashSet<String>,
    restricted_crates: HashSet<String>,
}

impl Restricted {
    pub fn new(layer_names: HashSet<String>, restricted_crates: HashSet<String>) -> Self {
        Restricted {
            layer_names,
            restricted_crates,
        }
    }

    pub fn layer_names(&self) -> &HashSet<String> {
        &self.layer_names
    }

    pub fn restricted_crates(&self) -> &HashSet<String> {
        &self.restricted_crates
    }
}
