mod available;
mod may_not_access;
mod may_not_be_accessed_by;
mod may_only_access;
mod may_only_be_accessed_by;
mod no_layer_cyclic_dependencies;
mod no_module_cyclic_dependencies;
mod no_parent_access;
mod restricted;

pub use self::available::Available;
pub use self::may_not_access::MayNotAccess;
pub use self::may_not_be_accessed_by::MayNotBeAccessedBy;
pub use self::may_only_access::MayOnlyAccess;
pub use self::may_only_be_accessed_by::MayOnlyBeAccessedBy;
pub use self::no_layer_cyclic_dependencies::NoLayerCyclicDependencies;
pub use self::no_module_cyclic_dependencies::NoModuleCyclicDependencies;
pub use self::no_parent_access::NoParentAccess;
pub use self::restricted::Restricted;

/// Defines the scope within which an access rule is enforced.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RuleScope {
    /// Rule applies across all modules regardless of parent/subdomain
    #[default]
    Global,
    /// Rule applies only when source and target share the same direct parent
    Parent,
    /// Rule applies only when source and target are within the same subdomain
    Subdomain,
}
