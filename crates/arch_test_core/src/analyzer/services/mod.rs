pub use self::access_rule::AccessRule;
// TODO
#[allow(unused_imports)]
pub(crate) use self::access_rule::get_module_subdomain;

mod access_rule;
mod cyclic_dependency;
