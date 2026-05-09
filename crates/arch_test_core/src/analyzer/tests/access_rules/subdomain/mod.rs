use velcro::hash_set;

use crate::analyzer::services::get_module_subdomain;
use crate::parser::materials::ModuleTree;

#[test]
fn test_get_module_subdomain_flat_structure() {
    // Build a simple flat tree: crate -> file_1
    // No subdomain names defined, so should return None
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/layer_dependency_direction/main.rs");
    let tree = module_tree.tree();
    let subdomain_names = &hash_set![];

    // Find a non-root node
    for node in tree.iter() {
        if node.parent_index().is_some() && node.module_name() != "crate" {
            let result = get_module_subdomain(node.index(), tree, subdomain_names);
            assert!(result.is_none(), "Flat structure should have no subdomain");
            break;
        }
    }
}

#[test]
fn test_get_module_subdomain_single_subdomain() {
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/subdomain/main.rs");
    let tree = module_tree.tree();
    let subdomain_names = hash_set!["fixation_processing".to_owned()];

    // Find the entity.rs node - it should have "fixation_processing" as subdomain
    let mut found_entity = false;
    for node in tree.iter() {
        if node.module_name() == "entity" {
            let result = get_module_subdomain(node.index(), tree, &subdomain_names);
            assert!(result.is_some(), "entity should have a subdomain");
            assert_eq!(result.unwrap(), "fixation_processing");
            found_entity = true;
            break;
        }
    }
    assert!(found_entity, "Should have found entity node");
}

#[test]
fn test_get_module_subdomain_deep_nesting() {
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/subdomain/main.rs");
    let tree = module_tree.tree();
    let subdomain_names = hash_set!["fixation_processing".to_owned(), "fixation_view".to_owned()];

    // entity is deeply nested: crate -> fixation_processing -> domain -> entity
    // It should find "fixation_processing" as the nearest subdomain
    for node in tree.iter() {
        if node.module_name() == "entity" {
            let result = get_module_subdomain(node.index(), tree, &subdomain_names);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), "fixation_processing");
            break;
        }
    }

    // repo is also deeply nested: crate -> fixation_processing -> infrastructure -> repo
    // It should also find "fixation_processing"
    for node in tree.iter() {
        if node.module_name() == "repo" {
            let result = get_module_subdomain(node.index(), tree, &subdomain_names);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), "fixation_processing");
            break;
        }
    }
}

#[test]
fn test_get_module_subdomain_nonexistent() {
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/subdomain/main.rs");
    let tree = module_tree.tree();
    // subdomain_names doesn't contain any actual subdomain from the tree
    let subdomain_names = hash_set!["nonexistent_subdomain".to_owned()];

    for node in tree.iter() {
        if node.parent_index().is_some() {
            let result = get_module_subdomain(node.index(), tree, &subdomain_names);
            assert!(
                result.is_none(),
                "Non-existent subdomain should return None"
            );
            break;
        }
    }
}

#[test]
fn test_get_module_subdomain_cross_subdomain() {
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/subdomain/main.rs");
    let tree = module_tree.tree();
    let subdomain_names = hash_set!["fixation_processing".to_owned(), "fixation_view".to_owned()];

    // Get subdomain for entity in fixation_processing
    let mut processing_entity_subdomain: Option<&str> = None;
    let mut view_model_subdomain: Option<&str> = None;

    for node in tree.iter() {
        if node.module_name() == "entity" {
            processing_entity_subdomain =
                get_module_subdomain(node.index(), tree, &subdomain_names);
        }
        if node.module_name() == "model" {
            view_model_subdomain = get_module_subdomain(node.index(), tree, &subdomain_names);
        }
    }

    assert!(processing_entity_subdomain.is_some());
    assert!(view_model_subdomain.is_some());
    assert_eq!(processing_entity_subdomain.unwrap(), "fixation_processing");
    assert_eq!(view_model_subdomain.unwrap(), "fixation_view");
}
