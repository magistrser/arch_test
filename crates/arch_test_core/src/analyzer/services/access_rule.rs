use std::collections::HashSet;
use std::fmt::Debug;

use velcro::hash_set;

use crate::analyzer::domain_values::access_rules::{
    MayNotAccess, MayNotBeAccessedBy, MayOnlyAccess, MayOnlyBeAccessedBy,
    NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess,
};
use crate::analyzer::domain_values::RuleViolationType;
use crate::analyzer::entities::RuleViolation;
use crate::analyzer::services::cyclic_dependency::{
    contains_cyclic_dependency, contains_cyclic_dependency_on_any_level,
};
use crate::parser::entities::ModuleNode;
use crate::parser::materials::ModuleTree;
use std::collections::hash_map::RandomState;

/// Check if a node is the root crate module (lib.rs or main.rs)
/// The root crate module is identified by having no parent and being named "crate"
fn is_root_crate_module(node: &ModuleNode) -> bool {
    node.parent_index().is_none() && node.module_name() == "crate"
}

pub trait AccessRule: Debug {
    fn check(&self, module_tree: &ModuleTree, excluded_modules: &HashSet<String>) -> Result<(), RuleViolation>;
    fn validate(&self, layer_names: &HashSet<String>) -> bool;
}

impl AccessRule for MayOnlyAccess {
    fn check(&self, module_tree: &ModuleTree, excluded_modules: &HashSet<String>) -> Result<(), RuleViolation> {
        let tree = module_tree.tree();
        for node in tree.iter().filter(|node| {
            let node_path = node.get_fully_qualified_path(tree);
            if is_module_excluded(&node_path, excluded_modules) {
                return false;
            }
            node.module_name() == self.accessor()
                || has_parent_matching_name(
                    &hash_set![self.accessor().clone()],
                    node.index(),
                    tree,
                )
        }) {
            if let Some(use_relation) = node
                .use_relations(tree, module_tree.possible_uses(), false)
                .iter()
                .find(|use_relation| {
                    let used_node_path = tree[use_relation.used_object().node_index()].get_fully_qualified_path(tree);
                    if is_module_excluded(&used_node_path, excluded_modules) {
                        return false;
                    }
                    !self.accessed().contains(
                        tree[use_relation.used_object().node_index()].module_name(),
                    ) && !has_parent_matching_name(
                        self.accessed(),
                        use_relation.used_object().node_index(),
                        tree,
                    ) && (!self.when_same_parent()
                        || tree[use_relation.used_object().node_index()]
                            .parent_index()
                            == node.parent_index())
                })
            {
                return Err(RuleViolation::new(
                    RuleViolationType::SingleLocation,
                    Box::new(self.clone()),
                    vec![use_relation.clone()],
                ));
            }
        }
        Ok(())
    }

    fn validate(&self, layer_names: &HashSet<String, RandomState>) -> bool {
        layer_names.contains(self.accessor())
            && self
                .accessed()
                .iter()
                .all(|layer| layer_names.contains(layer))
    }
}

impl AccessRule for MayNotAccess {
    fn check(&self, module_tree: &ModuleTree, excluded_modules: &HashSet<String>) -> Result<(), RuleViolation> {
        let tree = module_tree.tree();
        for node in tree.iter().filter(|node| {
            let node_path = node.get_fully_qualified_path(tree);
            if is_module_excluded(&node_path, excluded_modules) {
                return false;
            }
            node.module_name() == self.accessor()
                || has_parent_matching_name(
                    &hash_set![self.accessor().clone()],
                    node.index(),
                    tree,
                )
        }) {
            if let Some(use_relation) = node
                .use_relations(tree, module_tree.possible_uses(), false)
                .iter()
                .find(|use_relation| {
                    let used_node_path = tree[use_relation.used_object().node_index()].get_fully_qualified_path(tree);
                    if is_module_excluded(&used_node_path, excluded_modules) {
                        return false;
                    }
                    (self.accessed().contains(
                        tree[use_relation.used_object().node_index()].module_name(),
                    ) || has_parent_matching_name(
                        self.accessed(),
                        use_relation.used_object().node_index(),
                        tree,
                    )) && (!self.when_same_parent()
                        || tree[use_relation.used_object().node_index()]
                            .parent_index()
                            == node.parent_index())
                })
            {
                return Err(RuleViolation::new(
                    RuleViolationType::SingleLocation,
                    Box::new(self.clone()),
                    vec![use_relation.clone()],
                ));
            }
        }
        Ok(())
    }

    fn validate(&self, layer_names: &HashSet<String, RandomState>) -> bool {
        layer_names.contains(self.accessor())
            && self
                .accessed()
                .iter()
                .all(|layer| layer_names.contains(layer))
    }
}

impl AccessRule for MayOnlyBeAccessedBy {
    fn check(&self, module_tree: &ModuleTree, excluded_modules: &HashSet<String>) -> Result<(), RuleViolation> {
        let tree = module_tree.tree();
        for node in tree.iter().filter(|node| {
            let node_path = node.get_fully_qualified_path(tree);
            if is_module_excluded(&node_path, excluded_modules) {
                return false;
            }
            // Skip the root crate module (lib.rs/main.rs) - it's not a layer
            !is_root_crate_module(node)
                && !self.accessors().contains(node.module_name())
                && !has_parent_matching_name(self.accessors(), node.index(), tree)
        }) {
            if let Some(use_relation) = node
                .use_relations(tree, module_tree.possible_uses(), false)
                .iter()
                .find(|use_relation| {
                    let used_node_path = tree[use_relation.used_object().node_index()].get_fully_qualified_path(tree);
                    if is_module_excluded(&used_node_path, excluded_modules) {
                        return false;
                    }
                    (self.accessed()
                        == tree[use_relation.used_object().node_index()]
                            .module_name()
                        || has_parent_matching_name(
                            &hash_set![self.accessed().clone()],
                            use_relation.used_object().node_index(),
                            tree,
                        ))
                        && (!self.when_same_parent()
                            || tree[use_relation.used_object().node_index()]
                                .parent_index()
                                == node.parent_index())
                })
            {
                return Err(RuleViolation::new(
                    RuleViolationType::SingleLocation,
                    Box::new(self.clone()),
                    vec![use_relation.clone()],
                ));
            }
        }
        Ok(())
    }

    fn validate(&self, layer_names: &HashSet<String, RandomState>) -> bool {
        layer_names.contains(self.accessed())
            && self
                .accessors()
                .iter()
                .all(|layer| layer_names.contains(layer))
    }
}

impl AccessRule for MayNotBeAccessedBy {
    fn check(&self, module_tree: &ModuleTree, excluded_modules: &HashSet<String>) -> Result<(), RuleViolation> {
        let tree = module_tree.tree();
        for node in tree.iter().filter(|node| {
            let node_path = node.get_fully_qualified_path(tree);
            if is_module_excluded(&node_path, excluded_modules) {
                return false;
            }
            self.accessors().contains(node.module_name())
                || has_parent_matching_name(self.accessors(), node.index(), tree)
        }) {
            if let Some(use_relation) = node
                .use_relations(tree, module_tree.possible_uses(), false)
                .iter()
                .find(|use_relation| {
                    let used_node_path = tree[use_relation.used_object().node_index()].get_fully_qualified_path(tree);
                    if is_module_excluded(&used_node_path, excluded_modules) {
                        return false;
                    }
                    (self.accessed()
                        == tree[use_relation.used_object().node_index()]
                            .module_name()
                        || has_parent_matching_name(
                            &hash_set![self.accessed().clone()],
                            use_relation.used_object().node_index(),
                            tree,
                        ))
                        && (!self.when_same_parent()
                            || tree[use_relation.used_object().node_index()]
                                .parent_index()
                                == node.parent_index())
                })
            {
                return Err(RuleViolation::new(
                    RuleViolationType::SingleLocation,
                    Box::new(self.clone()),
                    vec![use_relation.clone()],
                ));
            }
        }
        Ok(())
    }

    fn validate(&self, layer_names: &HashSet<String, RandomState>) -> bool {
        layer_names.contains(self.accessed())
            && self
                .accessors()
                .iter()
                .all(|layer| layer_names.contains(layer))
    }
}

impl AccessRule for NoParentAccess {
    fn check(&self, module_tree: &ModuleTree, excluded_modules: &HashSet<String>) -> Result<(), RuleViolation> {
        let tree = module_tree.tree();
        for node in tree.iter().filter(|node| {
            let node_path = node.get_fully_qualified_path(tree);
            if is_module_excluded(&node_path, excluded_modules) {
                return false;
            }
            node.parent_index().is_some()
        }) {
            if let Some(use_relation) = node
                .use_relations(tree, module_tree.possible_uses(), false)
                .iter()
                .find(|use_relation| {
                    let used_node_path = tree[use_relation.used_object().node_index()].get_fully_qualified_path(tree);
                    if is_module_excluded(&used_node_path, excluded_modules) {
                        return false;
                    }
                    node.parent_index().is_some()
                        && node.parent_index().unwrap() == use_relation.used_object().node_index()
                })
            {
                return Err(RuleViolation::new(
                    RuleViolationType::SingleLocation,
                    Box::new(self.clone()),
                    vec![use_relation.clone()],
                ));
            }
        }
        Ok(())
    }

    fn validate(&self, _layer_names: &HashSet<String, RandomState>) -> bool {
        true
    }
}

impl AccessRule for NoModuleCyclicDependencies {
    fn check(&self, module_tree: &ModuleTree, excluded_modules: &HashSet<String>) -> Result<(), RuleViolation> {
        let tree = module_tree.tree();
        if let Some(involved) = contains_cyclic_dependency(module_tree) {
            // Filter out any violations involving excluded modules
            let filtered_involved: Vec<_> = involved.into_iter()
                .filter(|rel| {
                    let using_path = tree[rel.using_object().node_index()].get_fully_qualified_path(tree);
                    let used_path = tree[rel.used_object().node_index()].get_fully_qualified_path(tree);
                    !is_module_excluded(&using_path, excluded_modules) &&
                    !is_module_excluded(&used_path, excluded_modules)
                })
                .collect();
            
            if !filtered_involved.is_empty() {
                return Err(RuleViolation::new(
                    RuleViolationType::Cycle,
                    Box::new(self.clone()),
                    filtered_involved,
                ));
            }
        }
        Ok(())
    }

    fn validate(&self, _layer_names: &HashSet<String, RandomState>) -> bool {
        true
    }
}

impl AccessRule for NoLayerCyclicDependencies {
    fn check(&self, module_tree: &ModuleTree, excluded_modules: &HashSet<String>) -> Result<(), RuleViolation> {
        let tree = module_tree.tree();
        if let Some(involved) = contains_cyclic_dependency_on_any_level(module_tree) {
            // Filter out any violations involving excluded modules
            let filtered_involved: Vec<_> = involved.into_iter()
                .filter(|rel| {
                    let using_path = tree[rel.using_object().node_index()].get_fully_qualified_path(tree);
                    let used_path = tree[rel.used_object().node_index()].get_fully_qualified_path(tree);
                    !is_module_excluded(&using_path, excluded_modules) &&
                    !is_module_excluded(&used_path, excluded_modules)
                })
                .collect();
            
            if !filtered_involved.is_empty() {
                return Err(RuleViolation::new(
                    RuleViolationType::Cycle,
                    Box::new(self.clone()),
                    filtered_involved,
                ));
            }
        }
        Ok(())
    }

    fn validate(&self, _layer_names: &HashSet<String, RandomState>) -> bool {
        true
    }
}

fn has_parent_matching_name(
    accessor_name: &HashSet<String>,
    mut node_index: usize,
    tree: &[ModuleNode],
) -> bool {
    while let Some(parent_index) = tree[node_index].parent_index() {
        if accessor_name.contains(tree[parent_index].module_name()) {
            return true;
        }
        node_index = parent_index;
    }
    false
}

/// Check if a module is excluded from architecture checks.
/// Supports exact match and prefix matching (if exclusion ends with "::").
fn is_module_excluded(fully_qualified_path: &str, excluded_modules: &HashSet<String>) -> bool {
    // Check exact match
    if excluded_modules.contains(fully_qualified_path) {
        return true;
    }
    // Check prefix match (for exclusions ending with "::")
    for excl in excluded_modules {
        if excl.ends_with("::") && fully_qualified_path.starts_with(excl) {
            return true;
        }
    }
    false
}
