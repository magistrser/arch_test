mod access_rule;
mod cyclic_dependency;

#[cfg(test)]
// TODO this placed here because of legacy test structure when you have to get access to the function
// outside of the module
pub use access_rule::get_module_subdomain;

pub use self::access_rule::{AccessRule, RuleCategory};
