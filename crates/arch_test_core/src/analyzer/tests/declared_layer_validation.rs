use velcro::hash_set;

use crate::analyzer::domain_values::access_rules::{MayNotAccess, RuleScope};
use crate::analyzer::domain_values::RuleViolationType;
use crate::parser::materials::ModuleTree;
use crate::Architecture;

#[test]
fn root_level_layers_pass() {
    // Test layers at root level using layer_dependency_direction tree (application, infra)
    let architecture = Architecture::new(hash_set!["application".to_owned(), "infra".to_owned(),]);
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/layer_dependency_direction/main.rs");

    let result = architecture.validate_declared_layers_exist(&module_tree);
    assert!(result.is_ok(), "Layers at root level should pass");
}

/// Test that a subset of existing layers passes validation.
#[test]
fn subset_of_layers_passes() {
    // Only declare a subset of actual layers
    let architecture = Architecture::new(hash_set!["application".to_owned()]);
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/layer_dependency_direction/main.rs");

    let result = architecture.validate_declared_layers_exist(&module_tree);
    assert!(result.is_ok(), "Subset of layers should pass");
}

/// Test that a missing layer causes an error.
#[test]
fn missing_layer_fails() {
    // Declare a layer that doesn't exist
    let architecture = Architecture::new(hash_set!["nonexistent_layer".to_owned()]);
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/layer_dependency_direction/main.rs");

    let result = architecture.validate_declared_layers_exist(&module_tree);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(
        err.violation_type(),
        RuleViolationType::DeclaredLayerNotFound
    );
}

/// Test that a missing subdomain causes an error.
#[test]
fn missing_subdomain_fails() {
    // Declare a subdomain that doesn't exist
    let architecture = Architecture::new(hash_set!["domain".to_owned()])
        .with_subdomain_names(hash_set!["nonexistent_subdomain".to_owned()]);
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/layer_dependency_direction/main.rs");

    let result = architecture.validate_declared_layers_exist(&module_tree);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(
        err.violation_type(),
        RuleViolationType::DeclaredLayerNotFound
    );
}

/// Test that a layer nested under an undeclared subdomain causes an error.
#[test]
fn nested_under_undeclared_subdomain_fails() {
    // The subdomain/main.rs has modules like "domain" nested under "fixation_processing"
    // We declare "fixation_view" as subdomain but NOT "fixation_processing"
    // So "domain" under "fixation_processing" should still fail
    let architecture = Architecture::new(hash_set!["domain".to_owned()])
        .with_subdomain_names(hash_set!["fixation_view".to_owned()]);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/subdomain/main.rs");

    let result = architecture.validate_declared_layers_exist(&module_tree);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(
        err.violation_type(),
        RuleViolationType::DeclaredLayerNotFound
    );
}

#[test]
fn nested_under_declared_subdomain_passes() {
    // Declare both the layer and ALL subdomains it lives under (tree has both fixation_processing and fixation_view)
    let architecture =
        Architecture::new(hash_set!["domain".to_owned()]).with_subdomain_names(hash_set![
            "fixation_processing".to_owned(),
            "fixation_view".to_owned()
        ]);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/subdomain/main.rs");

    let result = architecture.validate_declared_layers_exist(&module_tree);
    assert!(result.is_ok(), "Layer under declared subdomain should pass");
}

#[test]
fn layer_under_layer_passes() {
    // Flat tree with root level layers - declaring them should pass (no undeclared nesting)
    let architecture = Architecture::new(hash_set!["application".to_owned(), "infra".to_owned(),]);
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/layer_dependency_direction/main.rs");

    let result = architecture.validate_declared_layers_exist(&module_tree);
    assert!(
        result.is_ok(),
        "Layers at root level should pass (no nesting issue)"
    );
}

/// Test that existing subdomain passes validation.
#[test]
fn existing_subdomain_passes() {
    // Using the subdomain test structure where fixation_processing exists
    let architecture =
        Architecture::new(hash_set!["domain".to_owned()]).with_subdomain_names(hash_set![
            "fixation_processing".to_owned(),
            "fixation_view".to_owned()
        ]);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/subdomain/main.rs");

    let result = architecture.validate_declared_layers_exist(&module_tree);
    assert!(result.is_ok(), "Existing subdomains should pass");
}

#[test]
fn nested_non_subdomain_layer_at_root_passes() {
    let architecture = Architecture::new(hash_set!["application".to_owned(), "infra".to_owned()]);
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/layer_dependency_direction/main.rs");

    let result = architecture.validate_declared_layers_exist(&module_tree);
    assert!(
        result.is_ok(),
        "Layers at root level should pass when subdomain_names is empty"
    );
}

#[test]
fn nested_non_subdomain_layer_not_at_root_fails() {
    let architecture = Architecture::new(hash_set!["domain".to_owned()]);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/subdomain/main.rs");

    let result = architecture.validate_declared_layers_exist(&module_tree);
    assert!(
        result.is_err(),
        "Layers nested under non-subdomain directories should fail when subdomain_names is empty"
    );

    let err = result.unwrap_err();
    assert_eq!(
        err.violation_type(),
        RuleViolationType::DeclaredLayerNotFound
    );
}

#[test]
fn nested_non_subdomain_layer_files_inside_are_found_via_parent() {
    let architecture = Architecture::new(hash_set!["application".to_owned(), "infra".to_owned()])
        .with_access_rule(MayNotAccess::new(
            "application".to_owned(),
            hash_set!["infra".to_owned()],
            RuleScope::Global,
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/layer_dependency_direction/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_err());
}
