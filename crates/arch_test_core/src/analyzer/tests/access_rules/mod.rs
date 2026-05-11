#[cfg(test)]
use std::collections::HashSet;
#[cfg(test)]
mod subdomain;

use rstest::rstest;
use velcro::hash_set;

use crate::analyzer::domain_values::access_rules::{
    Available, MayNotAccess, MayNotBeAccessedBy, MayOnlyAccess, MayOnlyBeAccessedBy,
    NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess, Restricted, RuleScope,
};
use crate::{Architecture, ModuleTree};

#[test]
fn layer_dependency_direction_violation() {
    let architecture = Architecture::new(hash_set!["application".to_owned(), "infra".to_owned()])
        .with_access_rule(MayNotAccess::new(
            "application".to_owned(),
            hash_set!["infra".to_owned()],
            RuleScope::Global,
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/layer_dependency_direction/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn no_parent_access() {
    let architecture = Architecture::new(hash_set![]).with_access_rule(NoParentAccess);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/no_parent_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn no_module_cyclic_dependencies() {
    let architecture = Architecture::new(hash_set![]).with_access_rule(NoModuleCyclicDependencies);
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/no_module_cyclic_dependencies/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn cyclic_dependency_over_several_modules() {
    let architecture = Architecture::new(hash_set![]).with_access_rule(NoModuleCyclicDependencies);
    let module_tree = ModuleTree::new(
        "src/analyzer/tests/access_rules/cyclic_dependency_over_several_modules/main.rs",
    );
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn no_layer_cyclic_dependencies() {
    let architecture = Architecture::new(hash_set![]).with_access_rule(NoLayerCyclicDependencies);
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/no_layer_cyclic_dependencies/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn cyclic_dependency_over_several_layers() {
    let architecture = Architecture::new(hash_set![]).with_access_rule(NoLayerCyclicDependencies);
    let module_tree = ModuleTree::new(
        "src/analyzer/tests/access_rules/cyclic_dependency_over_several_layers/main.rs",
    );
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn may_only_access_positive() {
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_access_rule(MayOnlyAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn may_only_access_negative() {
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()]).with_access_rule(
            MayOnlyAccess::new("file_1".to_owned(), hash_set![], RuleScope::Global),
        );
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn may_only_access_when_same_parent_positive() {
    let architecture =
        Architecture::new(hash_set!["layer_1".to_owned(), "layer_2".to_owned()]).with_access_rule(
            MayOnlyAccess::new("file_1".to_owned(), hash_set![], RuleScope::Parent),
        );
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/may_access_same_parent/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn may_only_access_when_same_parent_negative() {
    let architecture =
        Architecture::new(hash_set!["layer_1".to_owned(), "layer_2".to_owned()]).with_access_rule(
            MayOnlyAccess::new("file_1".to_owned(), hash_set![], RuleScope::Global),
        );
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/may_access_same_parent/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn may_not_access() {
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_access_rule(MayNotAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn may_not_access_when_same_parent_positive() {
    let architecture = Architecture::new(hash_set!["layer_1".to_owned(), "layer_2".to_owned()])
        .with_access_rule(MayNotAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Parent,
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/may_access_same_parent/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn may_not_access_when_same_parent_negative() {
    let architecture = Architecture::new(hash_set!["layer_1".to_owned(), "layer_2".to_owned()])
        .with_access_rule(MayNotAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Global,
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/may_access_same_parent/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn may_only_be_accessed_by() {
    let architecture = Architecture::new(hash_set![
        "file_1".to_owned(),
        "file_2".to_owned(),
        "file_3".to_owned()
    ])
    .with_access_rule(MayOnlyBeAccessedBy::new(
        "file_2".to_owned(),
        hash_set!["file_1".to_owned()],
        RuleScope::Global,
    ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn may_only_be_accessed_by_when_same_parent_positive() {
    let architecture =
        Architecture::new(hash_set!["layer_1".to_owned(), "layer_2".to_owned()]).with_access_rule(
            MayOnlyBeAccessedBy::new("file_2".to_owned(), hash_set![], RuleScope::Parent),
        );
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/may_access_same_parent/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn may_only_be_accessed_by_when_same_parent_negative() {
    let architecture =
        Architecture::new(hash_set!["layer_1".to_owned(), "layer_2".to_owned()]).with_access_rule(
            MayOnlyBeAccessedBy::new("file_2".to_owned(), hash_set![], RuleScope::Global),
        );
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/may_access_same_parent/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn may_not_be_accessed_by() {
    let architecture = Architecture::new(hash_set![
        "file_1".to_owned(),
        "file_2".to_owned(),
        "file_3".to_owned()
    ])
    .with_access_rule(MayNotBeAccessedBy::new(
        "file_2".to_owned(),
        hash_set!["file_3".to_owned()],
        RuleScope::Global,
    ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn may_not_be_accessed_by_when_same_parent_positive() {
    let architecture = Architecture::new(hash_set!["layer_1".to_owned(), "layer_2".to_owned()])
        .with_access_rule(MayOnlyBeAccessedBy::new(
            "file_2".to_owned(),
            hash_set!["file_1".to_owned()],
            RuleScope::Parent,
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/may_access_same_parent/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn may_not_be_accessed_by_when_same_parent_negative() {
    let architecture = Architecture::new(hash_set!["layer_1".to_owned(), "layer_2".to_owned()])
        .with_access_rule(MayNotBeAccessedBy::new(
            "file_2".to_owned(),
            hash_set!["file_1".to_owned()],
            RuleScope::Global,
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/may_access_same_parent/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn myself() {
    let architecture = Architecture::new(hash_set![
        "analyzer".to_owned(),
        "parser".to_owned(),
        "domain_values".to_owned(),
        "entities".to_owned(),
        "materials".to_owned(),
        "services".to_owned(),
        "tests".to_owned(),
        "utils".to_owned()
    ])
    .with_access_rule(NoParentAccess)
    .with_access_rule(NoModuleCyclicDependencies)
    .with_access_rule(NoLayerCyclicDependencies)
    .with_access_rule(MayNotAccess::new(
        "parser".to_owned(),
        hash_set!["analyzer".to_owned()],
        RuleScope::Parent,
    ))
    .with_access_rule(MayOnlyAccess::new(
        "analyzer".to_owned(),
        hash_set!["analyzer".to_owned(), "parser".to_owned()],
        RuleScope::Parent,
    ))
    .with_access_rule(MayOnlyAccess::new(
        "domain_values".to_owned(),
        hash_set!["domain_values".to_owned(), "utils".to_owned()],
        RuleScope::Global,
    ))
    .with_access_rule(MayOnlyAccess::new(
        "entities".to_owned(),
        hash_set!["entities".to_owned(), "domain_values".to_owned()],
        RuleScope::Global,
    ))
    .with_access_rule(MayOnlyAccess::new(
        "utils".to_owned(),
        hash_set!["utils".to_owned()],
        RuleScope::Parent,
    ))
    .with_access_rule(MayNotAccess::new(
        "services".to_owned(),
        hash_set!["materials".to_owned()],
        RuleScope::Parent,
    ))
    .with_access_rule(MayNotAccess::new(
        "materials".to_owned(),
        hash_set!["tests".to_owned()],
        RuleScope::Parent,
    ));
    let module_tree = ModuleTree::new("src/lib.rs");
    assert!(architecture.validate_access_rules().is_ok());
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

// ============================================================================
// Module Exclusion Tests
// ============================================================================

#[test]
fn exclude_modules_empty_list() {
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_excluded_modules(hash_set![])
        .with_access_rule(MayOnlyAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_invalid_identifiers() {
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_excluded_modules(hash_set![
            "non_existent_module".to_owned(),
            "another_invalid::module".to_owned(),
            "file_999".to_owned(),
        ])
        .with_access_rule(MayOnlyAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_exact_match() {
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_excluded_modules(hash_set!["file_2".to_owned()])
        .with_access_rule(MayOnlyAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_prefix_match() {
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_excluded_modules(hash_set!["file_2::".to_owned()])
        .with_access_rule(MayOnlyAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_cyclic_dependency_integrity() {
    // file_1 <-> file_2 has cyclic dependency
    let architecture = Architecture::new(hash_set![])
        .with_excluded_modules(hash_set!["crate::file_1".to_owned()])
        .with_access_rule(NoModuleCyclicDependencies);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_accessor_excluded() {
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_excluded_modules(hash_set!["file_1".to_owned()])
        .with_access_rule(MayOnlyAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_accessed_excluded() {
    // Exclude file_2 (the accessed) - no violations should be detected
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_excluded_modules(hash_set!["file_2".to_owned()])
        .with_access_rule(MayOnlyAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_may_not_access() {
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_excluded_modules(hash_set!["crate::file_2".to_owned()])
        .with_access_rule(MayNotAccess::new(
            "file_1".to_owned(),
            hash_set!["file_2".to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_may_only_be_accessed_by() {
    let architecture = Architecture::new(hash_set![
        "file_1".to_owned(),
        "file_2".to_owned(),
        "file_3".to_owned()
    ])
    .with_excluded_modules(hash_set!["file_3".to_owned()])
    .with_access_rule(MayOnlyBeAccessedBy::new(
        "file_2".to_owned(),
        hash_set!["file_1".to_owned()],
        RuleScope::Global,
    ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_may_not_be_accessed_by() {
    let architecture = Architecture::new(hash_set![
        "file_1".to_owned(),
        "file_2".to_owned(),
        "file_3".to_owned()
    ])
    .with_excluded_modules(hash_set!["file_3".to_owned()])
    .with_access_rule(MayNotBeAccessedBy::new(
        "file_2".to_owned(),
        hash_set!["file_3".to_owned()],
        RuleScope::Global,
    ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_no_parent_access() {
    let architecture = Architecture::new(hash_set![])
        .with_excluded_modules(hash_set!["crate::child".to_owned()])
        .with_access_rule(NoParentAccess);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/no_parent_access/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn is_module_excluded_returns_false_for_non_excluded() {
    let architecture =
        Architecture::new(hash_set![]).with_excluded_modules(hash_set!["file_1".to_owned()]);

    assert!(architecture.is_module_excluded("file_1"));
    assert!(!architecture.is_module_excluded("file_2"));
    assert!(!architecture.is_module_excluded("file_3"));
}

#[test]
fn is_module_excluded_prefix_matching() {
    let architecture =
        Architecture::new(hash_set![]).with_excluded_modules(hash_set!["parent::".to_owned()]);

    assert!(architecture.is_module_excluded("parent::"));
    assert!(architecture.is_module_excluded("parent::child"));
    assert!(architecture.is_module_excluded("parent::child::grandchild"));
    assert!(!architecture.is_module_excluded("other"));
    assert!(!architecture.is_module_excluded("other::child"));
}

#[test]
fn exclude_modules_multiple_exclusions() {
    let architecture = Architecture::new(hash_set![
        "file_1".to_owned(),
        "file_2".to_owned(),
        "file_3".to_owned()
    ])
    .with_excluded_modules(hash_set!["file_1".to_owned(), "file_2".to_owned(),])
    .with_access_rule(MayOnlyAccess::new(
        "file_1".to_owned(),
        hash_set!["file_2".to_owned(), "file_3".to_owned()],
        RuleScope::Global,
    ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_both_cyclic_modules_excluded() {
    let architecture = Architecture::new(hash_set![])
        .with_excluded_modules(hash_set![
            "crate::file_1".to_owned(),
            "crate::file_2".to_owned()
        ])
        .with_access_rule(NoModuleCyclicDependencies);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn exclude_modules_complete_layer_specification() {
    // file_3 is not in any layer, but if we exclude it, check should pass
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_excluded_modules(hash_set!["crate::file_3".to_owned()]);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/exclude_modules/main.rs");

    assert!(architecture
        .check_complete_layer_specification(&module_tree)
        .is_ok());
}

const INVALID_ACCESS_PATTERN: (&str, &[&str]) = ("file_1", &["std"]);
const DEEP_NESTING_VALID_PATTERN: (&str, &[&str]) = ("domain", &["std", "serde"]);
const DEEP_NESTING_INVALID_PATTERN: (&str, &[&str]) = ("domain", &["std"]);

#[rstest]
#[case(
    "std",
    "src/analyzer/tests/access_rules/available_use_violation/main.rs"
)]
#[case(
    "std",
    "src/analyzer/tests/access_rules/available_type_violation/main.rs"
)]
fn test_checks_using_unavailable(#[case] crate_name: &str, #[case] module_path: &str) {
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned()]).with_access_rule(Available::new(
            hash_set!["file_1".to_owned()],
            hash_set![crate_name.to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new(module_path);
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[rstest]
#[case(
    "serde",
    "src/analyzer/tests/access_rules/available_use_violation/main.rs"
)]
#[case(
    "serde_json",
    "src/analyzer/tests/access_rules/available_type_violation/main.rs"
)]
fn test_checks_available_libs(#[case] crate_name: &str, #[case] module_path: &str) {
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned()]).with_access_rule(Available::new(
            hash_set!["file_1".to_owned()],
            hash_set![crate_name.to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new(module_path);

    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn available_white_list_comprehensive_positive() {
    // Test: All external imports in white_list - should pass
    // Local modules (crate::, my_module, self::, super::) should be allowed by default
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned()]).with_access_rule(Available::new(
            hash_set!["file_1".to_owned()],
            hash_set!["std".to_owned(), "serde_json".to_owned()], // Only external crates
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new(
        "src/analyzer/tests/access_rules/available_white_list_comprehensive/main.rs",
    );
    dbg!(&module_tree);
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[rstest]
#[case(hash_set!["std".to_owned()])]
#[case(hash_set!["serde_json".to_owned()])]
#[case(hash_set![])]
fn available_white_list_comprehensive_missing_external_crate(#[case] white_list: HashSet<String>) {
    // Test: Missing external crate (serde_json) - should fail
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned()]).with_access_rule(Available::new(
            hash_set!["file_1".to_owned()],
            white_list, // Missing serde_json
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new(
        "src/analyzer/tests/access_rules/available_white_list_comprehensive/main.rs",
    );
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[rstest]
#[case(DEEP_NESTING_INVALID_PATTERN.0, DEEP_NESTING_INVALID_PATTERN.1, false)]
#[case(DEEP_NESTING_VALID_PATTERN.0, DEEP_NESTING_VALID_PATTERN.1, true)]
fn available_deep_nesting(
    #[case] layer: &str,
    #[case] white_list: &[&str],
    #[case] expect_ok: bool,
) {
    let architecture =
        Architecture::new(hash_set![layer.to_owned()]).with_access_rule(Available::new(
            hash_set![layer.to_owned()],
            white_list.iter().map(|s| (*s).to_owned()).collect(),
            RuleScope::Global,
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/available_deep_nesting/main.rs");
    if expect_ok {
        assert!(architecture.check_access_rules(&module_tree).is_ok());
    } else {
        assert!(architecture.check_access_rules(&module_tree).is_err());
        architecture
            .check_access_rules(&module_tree)
            .err()
            .unwrap()
            .print(module_tree.tree());
    }
}

#[rstest]
#[case(
    "src/analyzer/tests/access_rules/available_local_module_reference/main.rs",
    INVALID_ACCESS_PATTERN.0,
    INVALID_ACCESS_PATTERN.1,
)]
#[case(
    "src/analyzer/tests/access_rules/available_self_method_call/main.rs",
    INVALID_ACCESS_PATTERN.0,
    INVALID_ACCESS_PATTERN.1,
)]
fn available_does_not_flag_local_references(
    #[case] module_path: &str,
    #[case] layer: &str,
    #[case] white_list: &[&str],
) {
    let architecture =
        Architecture::new(hash_set![layer.to_owned()]).with_access_rule(Available::new(
            hash_set![layer.to_owned()],
            white_list.iter().map(|s| (*s).to_owned()).collect(),
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new(module_path);
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

// ============================================================================
// Restricted rule tests
// ============================================================================

#[test]
fn restricted_simple_violation() {
    // Layer "file_1" has serde restricted
    // file_1.rs uses serde::Serialize - should trigger violation
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned()]).with_access_rule(Restricted::new(
            hash_set!["file_1".to_owned()],
            hash_set!["serde".to_owned()],
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/restricted_simple_violation/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
    architecture
        .check_access_rules(&module_tree)
        .err()
        .unwrap()
        .print(module_tree.tree());
}

#[test]
fn restricted_comprehensive_positive() {
    // Layer "file_1" has tokio restricted (not used in the file)
    // file_1.rs uses std::collections::HashMap and serde_json - should NOT trigger violation
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned()]).with_access_rule(Restricted::new(
            hash_set!["file_1".to_owned()],
            hash_set!["tokio".to_owned()],
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/restricted_comprehensive/main.rs");
    // tokio is not used, so this should pass
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn restricted_comprehensive_negative() {
    // Layer "file_1" has serde_json restricted
    // file_1.rs uses serde_json::Value - should trigger violation
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned()]).with_access_rule(Restricted::new(
            hash_set!["file_1".to_owned()],
            hash_set!["serde_json".to_owned()],
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/restricted_comprehensive/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn restricted_multiple_crates() {
    // Layer "file_1" has both serde and serde_json restricted
    // file_1.rs uses serde_json::Value - should trigger violation
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned()]).with_access_rule(Restricted::new(
            hash_set!["file_1".to_owned()],
            hash_set!["serde".to_owned(), "serde_json".to_owned()],
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/restricted_comprehensive/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn restricted_ignores_local_modules() {
    // Layer "file_1" has serde restricted
    // file_1.rs uses crate::, self::, super:: imports - should NOT trigger violation
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned()]).with_access_rule(Restricted::new(
            hash_set!["file_1".to_owned()],
            hash_set!["serde".to_owned()],
        ));
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/restricted_comprehensive/main.rs");
    // Local imports should be ignored by Restricted rule
    // This file also uses serde_json which is NOT restricted here, so it passes
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn conflicting_rules_detected() {
    // Layer "file_1" has both Available (allowing only std) and Restricted (blocking serde)
    // This should be detected as conflicting rules during validation
    let architecture = Architecture::new(hash_set!["file_1".to_owned()])
        .with_access_rule(Available::new(
            hash_set!["file_1".to_owned()],
            hash_set!["std".to_owned()],
            RuleScope::Global,
        ))
        .with_access_rule(Restricted::new(
            hash_set!["file_1".to_owned()],
            hash_set!["serde".to_owned()],
        ));

    // validate_access_rules should detect the conflict
    assert!(architecture.validate_access_rules().is_err());
    let err = architecture.validate_access_rules().err().unwrap();
    // Verify it's the ConflictingRules error
    let violation_type = err.violation_type();
    assert!(matches!(
        violation_type,
        crate::analyzer::domain_values::RuleViolationType::ConflictingRules
    ));
}

#[test]
fn available_deep_nesting_flat_structure_still_works() {
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned()]).with_access_rule(Available::new(
            hash_set!["file_1".to_owned()],
            hash_set!["std".to_owned(), "serde_json".to_owned()],
            RuleScope::Global,
        ));
    let module_tree = ModuleTree::new(
        "src/analyzer/tests/access_rules/available_white_list_comprehensive/main.rs",
    );
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}
