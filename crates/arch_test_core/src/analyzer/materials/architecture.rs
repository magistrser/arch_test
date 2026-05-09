use std::collections::HashSet;

use crate::analyzer::domain_values::RuleViolationType;
use crate::analyzer::entities::RuleViolation;
use crate::analyzer::services::AccessRule;
use crate::parser::entities::ModuleNode;
use crate::parser::materials::ModuleTree;

/// This is the central object that holds the architecture rules and executes them.
/// It allows defining layers, access rules, and optionally excluding specific modules from checks.
///
/// Example:
/// ```ignore
/// let architecture = Architecture::new(hash_set!["analyzer".to_owned(), "parser".to_owned()])
/// .with_excluded_modules(hash_set!["crate::tests::integration".to_owned()])
/// .with_access_rule(NoParentAccess)
/// .with_access_rule(NoModuleCyclicDependencies)
/// .with_access_rule(NoLayerCyclicDependencies)
/// ...
/// .with_access_rule(MayNotAccess::new(
///     "materials".to_owned(),
///     hash_set!["tests".to_owned()],
///     true,
/// ));
/// ```
#[derive(Debug)]
pub struct Architecture<'r> {
    layer_names: HashSet<String>,
    subdomain_names: HashSet<String>,
    excluded_modules: HashSet<String>,
    access_rules: Vec<Box<dyn AccessRule + 'r>>,
}

impl<'r> Architecture<'r> {
    pub fn new(layer_names: HashSet<String>) -> Self {
        Architecture {
            layer_names,
            subdomain_names: HashSet::default(),
            excluded_modules: HashSet::default(),
            access_rules: Vec::default(),
        }
    }

    pub fn with_subdomain_names(mut self, subdomain_names: HashSet<String>) -> Self {
        self.subdomain_names = subdomain_names;
        self
    }
    
    pub fn with_excluded_modules(mut self, excluded_modules: HashSet<String>) -> Self {
        self.excluded_modules = excluded_modules;
        self
    }

    pub fn with_access_rule(mut self, access_rule: impl AccessRule + 'r) -> Self {
        self.access_rules.push(Box::new(access_rule));
        self
    }

    pub fn validate_access_rules(&'r self) -> Result<(), RuleViolation<'r>> {
        for access_rule in self.access_rules.iter() {
            if !access_rule.validate(&self.layer_names) {
                return Err(RuleViolation::new(
                    RuleViolationType::LayerDoNotExist,
                    Box::new(access_rule),
                    vec![],
                ));
            }
        }
        Ok(())
    }

    pub fn subdomain_names(&self) -> &HashSet<String> {
        &self.subdomain_names
    }
    
    pub fn check_access_rules(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation<'_>> {
        for access_rule in self.access_rules.iter() {
            access_rule.check(module_tree, &self.excluded_modules, &self.subdomain_names)?;
        }
        Ok(())
    }

    pub fn check_complete_layer_specification(
        &self,
        module_tree: &ModuleTree,
    ) -> Result<(), RuleViolation<'_>> {
        let tree: &Vec<ModuleNode> = module_tree.tree();
        if tree.iter().any(|node| {
            let node_path = node.get_fully_qualified_path(tree);
            if self.is_module_excluded(&node_path) {
                return false;
            }
            if node.parent_index().is_none() {
                return false;
            }
            !self.is_node_or_ancestor_in_set(node.index(), tree, &self.layer_names)
                && !self.is_node_or_ancestor_in_set(node.index(), tree, &self.subdomain_names)
        }) {
            return Err(RuleViolation::new(
                RuleViolationType::IncompleteLayerSpecification,
                Box::new(()),
                vec![],
            ));
        }
        Ok(())
    }

    /// Walk from node up to root. Return true if ANY ancestor's module_name is in set.
    fn is_node_or_ancestor_in_set(
        &self,
        mut node_index: usize,
        tree: &[ModuleNode],
        names: &HashSet<String>,
    ) -> bool {
        loop {
            if names.contains(tree[node_index].module_name()) {
                return true;
            }
            match tree[node_index].parent_index() {
                Some(parent_idx) => node_index = parent_idx,
                None => return false,
            }
        }
    }

    /// Check if a module is excluded from architecture checks.
    /// Supports exact match and prefix matching (if exclusion ends with "::").
    pub fn is_module_excluded(&self, fully_qualified_path: &str) -> bool {
        // Check exact match
        if self.excluded_modules.contains(fully_qualified_path) {
            return true;
        }
        // Check prefix match (for exclusions ending with "::")
        for excl in &self.excluded_modules {
            if excl.ends_with("::") && fully_qualified_path.starts_with(excl) {
                return true;
            }
        }
        false
    }
}
