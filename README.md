# ArchTest

[![crates.io](https://img.shields.io/crates/v/cargo-archtest-cli.svg)](https://crates.io/crates/cargo-archtest-cli)
[![crates.io](https://img.shields.io/crates/v/arch_validation_core.svg)](https://crates.io/crates/arch_validation_core)
[![license](https://img.shields.io/crates/l/arch_test_core.svg)](https://github.com/magistrser/arch_test/blob/master/LICENCE)
[![Crates.io](https://img.shields.io/crates/d/cargo-archtest-cli?label=cargo%20installs)](https://crates.io/crates/cargo-archtest-cli)
[![Crates.2io](https://img.shields.io/crates/d/arch_validation_core?label=cargo%20installs)](https://crates.io/crates/arch_validation_core)

<p align="center">
  <img src="https://github.com/Geigerkind/arch_test/blob/master/logo.png?raw=true" />
</p>
ArchTest is a rule based architecture testing tool.
It applies static analyses on the specified rust project to extract use relationships.

## Features

* Detect cyclic dependencies level wise or module wise
* Prohibit parent access
* Define layer relationships like `MayNotAccess`, `MayOnlyAccess`, `MayNotBeAccessedBy`, `MayOnlyBeAccessedBy`
* Restrict external crate usage with `Available` rule (white_list) — specify which external crates (including std) each layer is **allowed** to use
* **Forbid** external crate usage with `Restricted` rule (black_list) — specify which external crates each layer is **not allowed** to use
* **Conflict detection** — `Available` and `Restricted` rules on the same layer are automatically detected as conflicting
* **Subdomain scoping** — group modules into logical subdomains and enforce rules within subdomain boundaries (via `RuleScope::Subdomain`)
* **Rule scoping** — control whether rules apply globally, within the same parent module, or within the same subdomain (via [`RuleScope`](crates/arch_test_core/src/analyzer/domain_values/access_rules/mod.rs:22-31) enum)
* Exclude specific modules from architecture checks (supports exact match and prefix matching)
* Exclude entire workspace crates from checking via `exclude_crates` in `architecture.json` or `--exclude-crate` CLI flag
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
  "subdomain_names": ["subdomain_1", "subdomain_2"],
  "exclude_modules": ["crate::tests::integration", "crate::utils::helpers"],
  "exclude_crates": ["libs/checksum_validator"],
  "access_rules": [
    "NoLayerCyclicDependencies",
    "NoModuleCyclicDependencies",
    "NoParentAccess",
    {
      "MayNotAccess": {
        "accessor": "parser",
        "accessed": ["analyzer"],
        "scope": "Parent"
      }
    },
    {
      "MayOnlyAccess": {
        "accessor": "analyzer",
        "accessed": ["analyzer", "parser"],
        "scope": "Parent"
      }
    },
    {
      "MayOnlyBeAccessedBy": {
        "accessors": ["materials", "tests"],
        "accessed": "services",
        "scope": "Global"
      }
    },
    {
      "MayNotBeAccessedBy": {
        "accessors": ["services", "domain_values", "entities", "utils"],
        "accessed": "materials",
        "scope": "Parent"
      }
    },
    {
      "Available": {
        "layer_names": ["parser"],
        "allowed_crates": ["std", "serde"]
      }
    },
    {
      "Restricted": {
        "layer_names": ["domain"],
        "restricted_crates": ["serde"]
      }
    }
  ]
}
```

> **Note:**
>
> * The `exclude_modules` field is optional. It supports:
>   * Exact module names: `"crate::utils"` — excludes only that specific module
>   * Prefix matching: `"crate::utils::"` — excludes the module and all its submodules
> * The `subdomain_names` field is optional. It defines logical subdomains — groups of modules that form independent units within your architecture.
> * The `scope` field replaces the old `when_same_parent` boolean. It accepts `"Global"`, `"Parent"`, or `"Subdomain"`. For backward compatibility, `when_same_parent` is still accepted (where `true` → `"Parent"`, `false` → `"Global"`), but `scope` takes precedence when both are specified.
> * The `Available` rule (white_list) does **not** support `scope` — it always applies globally to all modules in the specified layers (including deeply nested submodules).
> * The `Restricted` rule (black_list) works similarly to `Available` but in reverse: it **forbids** the listed crates.

### Using a rust test

You can use the `Architecture` struct in order to define your architecture.
Afterwards you check it for failures.

```rust
use arch_validation_core::access_rules::{
    MayNotAccess, MayNotBeAccessedBy, MayOnlyAccess, MayOnlyBeAccessedBy,
    NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess,
    Available, Restricted, RuleScope,
};
use arch_validation_core::{hash_set, Architecture, ModuleTree};

let architecture = Architecture::new(hash_set![
        "analyzer".to_owned(),
        "parser".to_owned(),
        ...
    ])
    .with_subdomain_names(hash_set![
        "fixation_processing".to_owned(),
        "fixation_view".to_owned(),
    ])
    .with_excluded_modules(hash_set!["crate::tests::integration".to_owned()])
    .with_access_rule(NoParentAccess)
    .with_access_rule(NoModuleCyclicDependencies)
    .with_access_rule(NoLayerCyclicDependencies)
    ...
    .with_access_rule(MayNotAccess::new(
        "materials".to_owned(),
        hash_set!["tests".to_owned()],
        RuleScope::Parent,
    ))
    .with_access_rule(Available::new(
        hash_set!["parser".to_owned()],
        hash_set!["std".to_owned(), "serde".to_owned()],
    ))
    .with_access_rule(Restricted::new(
        hash_set!["domain".to_owned()],
        hash_set!["serde".to_owned()],
    ));
let module_tree = ModuleTree::new("src/lib.rs");
assert!(architecture.validate_access_rules().is_ok());
assert!(architecture.check_access_rules(&module_tree).is_ok());
```

> **Notes:**
>
> * Use `with_excluded_modules()` to skip specific modules from architecture checks.
>   Supports exact match (`"crate::utils"`) and prefix matching (`"crate::utils::"`).
> * Use `with_subdomain_names()` to define logical subdomains for subdomain-scoped rules.
> * Use `RuleScope` enum instead of the legacy boolean: `RuleScope::Parent` replaces `true`, `RuleScope::Global` replaces `false`.
> * `Available::new()` takes **2 parameters** — `layer_names` and `allowed_crates` (no `scope`).
> * `Restricted::new()` takes **2 parameters** — `layer_names` and `restricted_crates`.

If you are interested in the failure you can pretty print it like this:

```rust
architecture.check_access_rules(&module_tree).err().unwrap().print(module_tree.tree());
```

## Rule Scoping

The `scope` field on every relational access rule (`MayNotAccess`, `MayOnlyAccess`, etc.) controls **when** the rule fires. Three values:

| Scope       | Enforced when…                                                                       |
|-------------|--------------------------------------------------------------------------------------|
| `Global`    | …anywhere in the project (default).                                                  |
| `Parent`    | …source and target modules share the **same direct parent** (same directory).        |
| `Subdomain` | …both modules belong to the **same subdomain** (defined via `subdomain_names`).      |

**Examples:**

```json
// Only check this rule inside each subdomain independently
{ "MayNotAccess": { "accessor": "domain", "accessed": ["infrastructure"], "scope": "Subdomain" } }
// Enforce across the whole project — ignore subdomain boundaries
{ "MayNotAccess": { "accessor": "domain", "accessed": ["infrastructure"], "scope": "Global" } }
// Check only when both modules sit in the same directory
{ "MayNotAccess": { "accessor": "parser", "accessed": ["analyzer"], "scope": "Parent" } }
```

> **`Available` and `Restricted` do NOT support `scope`** — these rules are unary (check a single module against a list of allowed/forbidden crates), not binary (source→target). They always apply globally to every module in the target layers, including deeply nested submodules.

A module's subdomain is the nearest ancestor whose name matches `subdomain_names`. If no subdomains are defined, `Subdomain` is a no-op (never triggers).

> Legacy `when_same_parent` (boolean) still works: `true` → `Parent`, `false` → `Global`. `scope` takes precedence when both are set.

## Conflict detection

`Available` and `Restricted` rules on the **same layer** are automatically detected as **conflicting** during `validate_access_rules()`:

```json
{
  "Available": {
    "layer_names": ["domain"],
    "allowed_crates": ["std"]
  },
  "Restricted": {
    "layer_names": ["domain"],
    "restricted_crates": ["serde"]
  }
}
```

The validation will return a `ConflictingRules` error because the same layer (`domain`) has both a white-list and a black-list rule. This works for any layer overlap between `Available` and `Restricted` rule sets.

## Examples

The project includes several examples demonstrating different use cases:

| Example | Description |
|---------|-------------|
| [`layered_project`](examples/layered_project) | Classic layered architecture with `domain`, `application`, `infrastructure` layers and `scope: Global` rules |
| [`domain_decomposition`](examples/domain_decomposition) | Domain-driven decomposition with `billing`, `ordering`, `shared` subdomains and `scope: Subdomain` rules |
| [`restricted_example`](examples/restricted_example) | Demonstrates the `Restricted` (black_list) rule to forbid specific external crates |

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
```
