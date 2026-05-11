use std::fs::File;
use std::io::Read;
use std::path::Path;

use arch_validation_core::access_rules::{
    Available, MayNotAccess, MayNotBeAccessedBy, MayOnlyAccess, MayOnlyBeAccessedBy,
    NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess, Restricted, RuleScope,
};
use arch_validation_core::hash_set;
use arch_validation_core::Architecture;

use crate::domain_values::{AccessRule, Failure, Specification};

pub fn parse_specification(specification_path: &Path) -> Result<Architecture<'_>, Failure> {
    let specification: Specification =
        serde_json::from_str(&read_file_content(specification_path)?)
            .map_err(|_| Failure::SpecificationCouldNotBeParsed)?;

    let layer_names = specification.layer_names.clone();
    let subdomain_names: Vec<String> = specification.subdomain_names.unwrap_or_default();
    let excluded_modules: Vec<String> = specification.exclude_modules.unwrap_or_default();
    let mut architecture = Architecture::new(hash_set![..layer_names])
        .with_subdomain_names(hash_set![..subdomain_names])
        .with_excluded_modules(hash_set![..excluded_modules]);
    for access_rule in specification.access_rules {
        match access_rule {
            AccessRule::NoLayerCyclicDependencies => {
                architecture = architecture.with_access_rule(NoLayerCyclicDependencies)
            }
            AccessRule::NoModuleCyclicDependencies => {
                architecture = architecture.with_access_rule(NoModuleCyclicDependencies)
            }
            AccessRule::NoParentAccess => {
                architecture = architecture.with_access_rule(NoParentAccess)
            }
            AccessRule::MayOnlyAccess {
                accessor,
                accessed,
                scope,
                when_same_parent,
            } => {
                let effective_scope = resolve_rule_scope(scope, when_same_parent);
                architecture = architecture.with_access_rule(MayOnlyAccess::new(
                    accessor,
                    hash_set![..accessed],
                    effective_scope,
                ))
            }
            AccessRule::MayNotAccess {
                accessor,
                accessed,
                scope,
                when_same_parent,
            } => {
                let effective_scope = resolve_rule_scope(scope, when_same_parent);
                architecture = architecture.with_access_rule(MayNotAccess::new(
                    accessor,
                    hash_set![..accessed],
                    effective_scope,
                ))
            }
            AccessRule::MayOnlyBeAccessedBy {
                accessors,
                accessed,
                scope,
                when_same_parent,
            } => {
                let effective_scope = resolve_rule_scope(scope, when_same_parent);
                architecture = architecture.with_access_rule(MayOnlyBeAccessedBy::new(
                    accessed,
                    hash_set![..accessors],
                    effective_scope,
                ))
            }
            AccessRule::MayNotBeAccessedBy {
                accessors,
                accessed,
                scope,
                when_same_parent,
            } => {
                let effective_scope = resolve_rule_scope(scope, when_same_parent);
                architecture = architecture.with_access_rule(MayNotBeAccessedBy::new(
                    accessed,
                    hash_set![..accessors],
                    effective_scope,
                ))
            }
            AccessRule::Available {
                layer_names,
                allowed_crates,
            } => {
                architecture = architecture.with_access_rule(Available::new(
                    hash_set![..layer_names],
                    hash_set![..allowed_crates],
                ))
            }
            AccessRule::Restricted {
                layer_names,
                restricted_crates,
            } => {
                architecture = architecture.with_access_rule(Restricted::new(
                    hash_set![..layer_names],
                    hash_set![..restricted_crates],
                ))
            }
        }
    }
    Ok(architecture)
}

fn resolve_rule_scope(scope: Option<String>, when_same_parent: Option<bool>) -> RuleScope {
    if scope.is_some() && when_same_parent.is_some() {
        eprintln!("Warning: both 'scope' and 'when_same_parent' specified. Using 'scope'.");
    }
    if let Some(s) = scope {
        match s.as_str() {
            "Global" => RuleScope::Global,
            "Parent" => RuleScope::Parent,
            "Subdomain" => RuleScope::Subdomain,
            _ => RuleScope::Global,
        }
    } else if let Some(wsp) = when_same_parent {
        if wsp {
            RuleScope::Parent
        } else {
            RuleScope::Global
        }
    } else {
        RuleScope::Global
    }
}

fn read_file_content(file_path: &Path) -> Result<String, Failure> {
    let mut file = File::open(file_path).map_err(|_| Failure::SpecificationFileCantBeOpened)?;
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    Ok(content)
}
