//! # ArchTest
//! ArchTest is a rule based architecture testing tool. It applies static analyses on the specified rust project to extract use relationships.
//!
//! For a through documentation on how to use it for tests, please consult the [arch_test_core](https://docs.rs/arch_test_core/0.1.2/arch_test_core/) crate.
//!
//! ## Install
//! ```sh
//! cargo install cargo-archtest-cli --force
//! ```
//!
//! ## How to use it
//! Define in the cargo root path a file called `architecture.json`. Fill it according to the `Specification` struct.
//!
//! Example
//! ```json
//! let architecture = Architecture::new(hash_set!["analyzer".to_owned(), "parser".to_owned(), ...])
//! {
//!   "layer_names": ["analyzer", "parser", "domain_values", "entities", "materials", "services", "tests", "utils"],
//!   "access_rules": [
//!     "NoLayerCyclicDependencies",
//!     "NoModuleCyclicDependencies",
//!     "NoParentAccess",
//!     {
//!       "MayNotAccess": {
//!         "accessor": "parser",
//!         "accessed": ["analyzer"],
//!         "when_same_parent": true
//!       }
//!     },
//!     {
//!       "MayOnlyBeAccessedBy": {
//!         "accessors": ["materials", "tests"],
//!         "accessed": "services",
//!         "when_same_parent": false
//!       }
//!     },
//!     {
//!       "MayNotBeAccessedBy": {
//!         "accessors": ["services", "domain_values", "entities", "utils"],
//!         "accessed": "materials",
//!         "when_same_parent": true
//!       }
//!     }
//!   ]
//! }
//! ```
//! Then execute `cargo archtest` in your project directory.
//!
//! ## Continuous integration
//! You can use it in continuous integration by using either methods. If you decide to use the Cargo sub command on GitHub, the following snippet will allow you to test your project.
//! ```yml
//! arch_test:
//!    name: ArchTest
//!    runs-on: ubuntu-latest
//!    steps:
//!      - uses: actions/checkout@v2
//!      - uses: actions-rs/install@v0.1
//!        with:
//!          crate: cargo-archtest-cli
//!          version: latest
//!      - run: cargo archtest
//! ```

extern crate cargo_toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;

use std::fs;
use std::path::Path;

use structopt::StructOpt;

use crate::domain_values::Command;
use crate::services::{check_architecture, parse_raw_specification};

mod domain_values;
mod services;

#[cfg(test)]
mod tests;

fn main() {
    let Command::Archtest {
        check_for_complete_layer_specification,
        toml_path,
        exclude_crates: cli_exclude_crates,
    } = Command::from_args();
    let toml_path = Path::new(&toml_path);
    if toml_path.exists() && toml_path.is_file() {
        // Get the directory containing the Cargo.toml file
        // Canonicalize to get the absolute path, then get parent
        let cargo_dir = match toml_path.canonicalize() {
            Ok(canonical_path) => canonical_path
                .parent()
                .and_then(|p| p.to_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| ".".to_string()),
            Err(_) => {
                // If canonicalization fails, fall back to parent of the path as-is
                toml_path
                    .parent()
                    .and_then(|p| p.to_str())
                    .unwrap_or(".")
                    .to_string()
            }
        };

        // Build the combined exclude list (CLI + specification)
        let mut exclude_crates: Vec<String> = cli_exclude_crates;

        // Try to read workspace-root architecture.json for exclude_crates
        let root_arch_path = Path::new(&cargo_dir).join("architecture.json");
        if root_arch_path.exists() {
            if let Ok(spec) = parse_raw_specification(&root_arch_path) {
                if let Some(ec) = spec.exclude_crates {
                    for crate_path in ec {
                        if !exclude_crates.contains(&crate_path) {
                            exclude_crates.push(crate_path);
                        }
                    }
                }
            }
        }

        // Read the file content first, then parse it. This avoids workspace resolution
        // which can fail when workspace.metadata is present but no workspace root exists.
        let toml_content = match fs::read_to_string(toml_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Cargo.toml could not be read!");
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };

        // Try parsing with from_path first (for workspace support)
        // If that fails, fall back to from_str (without workspace resolution)
        let toml = match cargo_toml::Manifest::from_path(toml_path) {
            Ok(manifest) => manifest,
            Err(_) => {
                // Fall back to parsing from string, which doesn't try to resolve workspace
                match cargo_toml::Manifest::from_str(&toml_content) {
                    Ok(manifest) => manifest,
                    Err(e) => {
                        eprintln!("Cargo.toml could not be parsed!");
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        };

        if let Some(workspace) = toml.workspace {
            if workspace.members.is_empty() {
                // This is likely a package with workspace.metadata but not actually a workspace
                check_architecture(&cargo_dir, check_for_complete_layer_specification);
            } else {
                for member in workspace.members {
                    if member.contains('*') {
                        println!("Can not interpret paths with '*'");
                        std::process::exit(1);
                    } else if is_crate_excluded(&member, &exclude_crates) {
                        println!("[Skip]: '{}' is excluded from architecture check", member);
                        continue;
                    } else {
                        check_architecture(&member, check_for_complete_layer_specification);
                    }
                }
            }
        } else {
            // Use the directory containing Cargo.toml as the base directory
            check_architecture(&cargo_dir, check_for_complete_layer_specification);
        }
    } else {
        println!("Cargo.toml not found in the specified path!");
        std::process::exit(1);
    }

    println!("[Ok]: No architecture rules were violated!");
}

/// Checks if a workspace member path matches any excluded crate pattern.
/// Supports exact match and prefix match.
fn is_crate_excluded(member_path: &str, exclude_crates: &[String]) -> bool {
    exclude_crates
        .iter()
        .any(|ec| member_path == ec.as_str() || member_path.starts_with(ec.as_str()))
}
