# ArchTest
[![crates.io](https://img.shields.io/crates/v/cargo-archtest.svg)](https://crates.io/crates/cargo-archtest)
[![crates.io](https://img.shields.io/crates/v/arch_test_core.svg)](https://crates.io/crates/arch_test_core)
[![codecov](https://codecov.io/gh/Geigerkind/arch_test/branch/master/graph/badge.svg)](https://codecov.io/gh/Geigerkind/arch_test)
[![license](https://img.shields.io/crates/l/arch_test_core.svg)](https://github.com/Geigerkind/arch_test/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/cargo-archtest?label=cargo%20installs)](https://crates.io/crates/cargo-archtest)
[![Crates.2io](https://img.shields.io/crates/d/arch_test_core?label=cargo%20installs)](https://crates.io/crates/arch_test_core)

<p align="center">
  <img src="https://github.com/Geigerkind/arch_test/blob/master/logo.png?raw=true" />
</p>
ArchTest is a rule based architecture testing tool. 
It applies static analyses on the specified rust project to extract use relationships.

## Features
* Detect cyclic dependencies level wise or module wise
* Prohibit parent access
* Define layer relationships like `MayNotAccess`, `MayOnlyAccess`, `MayNotBeAccessedBy`, `MayOnlyBeAccessedBy`
* Restrict external crate usage with `Available` rule (white_list) — specify which external crates (including std) each layer is allowed to use
* Exclude specific modules from architecture checks (supports exact match and prefix matching)
* And more, please consult the documentation.

## Install
You can install it either as sub command of Cargo or as a package in your developer dependencies.
```
# Sub command
cargo install cargo-archtest-cli --force

# Package
[dev-dependencies]
arch_test_core = "*"
```

## How to use it
### Using the Cargo sub command
Define in the cargo root path a file called `architecture.json`. Fill it according to the `Specification` struct.
Example:
```json
{
  "layer_names": ["analyzer", "parser", "domain_values", "entities", "materials", "services", "tests", "utils"],
  "exclude_modules": ["crate::tests::integration", "crate::utils::helpers"],
  "access_rules": [
    "NoLayerCyclicDependencies",
    "NoModuleCyclicDependencies",
    "NoParentAccess",
    {
      "MayNotAccess": {
        "accessor": "parser",
        "accessed": ["analyzer"],
        "when_same_parent": true
      }
    },
    {
      "MayOnlyAccess": {
        "accessor": "analyzer",
        "accessed": ["analyzer", "parser"],
        "when_same_parent": true
      }
    },
    {
      "MayOnlyBeAccessedBy": {
        "accessors": ["materials", "tests"],
        "accessed": "services",
        "when_same_parent": false
      }
    },
    {
      "MayNotBeAccessedBy": {
        "accessors": ["services", "domain_values", "entities", "utils"],
        "accessed": "materials",
        "when_same_parent": true
      }
    },
    {
      "Available": {
        "layer_names": ["parser"],
        "allowed_crates": ["std", "serde"],
        "when_same_parent": false
      }
    }
  ]
}
```

> **Note:** The `exclude_modules` field is optional. It supports:
> - Exact module names: `"crate::utils"` - excludes only that specific module
> - Prefix matching: `"crate::utils::"` - excludes the module and all its submodules

### Using a rust test
You can use the `Architecture` struct in order to define your architecture.
Afterwards you check it for failures.
```rust
let architecture = Architecture::new(hash_set!["analyzer".to_owned(), "parser".to_owned(), ...])
.with_excluded_modules(hash_set!["crate::tests::integration".to_owned()])
.with_access_rule(NoParentAccess)
.with_access_rule(NoModuleCyclicDependencies)
.with_access_rule(NoLayerCyclicDependencies)
...
.with_access_rule(MayNotAccess::new(
    "materials".to_owned(),
    hash_set!["tests".to_owned()],
    true,
))
.with_access_rule(Available::new(
    hash_set!["parser".to_owned()],
    hash_set!["std".to_owned(), "serde".to_owned()],
    false,
));
let module_tree = ModuleTree::new("src/lib.rs");
assert!(architecture.validate_access_rules().is_ok());
assert!(architecture.check_access_rules(&module_tree).is_ok());
```

> **Note:** Use `with_excluded_modules()` to skip specific modules from architecture checks.
> Supports exact match (`"crate::utils"`) and prefix matching (`"crate::utils::"`).
If you are interested in the failure you can pretty print it like this:
```rust
architecture.check_access_rules(&module_tree).err().unwrap().print(module_tree.tree());
```

## Continuous integration
You can use it in continuous integration by using either methods.
If you decide to use the Cargo sub command on GitHub, the following snippet will allow you to test your project.
```yml
arch_test:
  name: ArchTest
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo install cargo-archtest-cli --locked
    - run: cargo archtest
