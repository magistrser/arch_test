use std::path::Path;

use arch_validation_core::ModuleTree;

use crate::services::parse_specification;

pub fn check_architecture(directory_path: &str, check_for_complete_layer_specification: bool) {
    // Ensure we have a proper path (handle both relative and absolute)
    let base_dir = Path::new(directory_path);
    let base_dir = if base_dir.is_absolute() {
        base_dir.to_path_buf()
    } else {
        // For relative paths, resolve them relative to current working directory
        std::env::current_dir()
            .unwrap_or_else(|_| Path::new(".").to_path_buf())
            .join(base_dir)
    };

    // Prioritize lib.rs over main.rs for architecture checking
    // since lib.rs typically contains the module structure
    let lib_path = base_dir.join("src/lib.rs");
    let main_path = base_dir.join("src/main.rs");
    let root_path = if lib_path.exists() && lib_path.is_file() {
        lib_path.to_string_lossy().to_string()
    } else if main_path.exists() && main_path.is_file() {
        main_path.to_string_lossy().to_string()
    } else {
        eprintln!(
            "Error: Neither src/main.rs nor src/lib.rs found in '{}'",
            directory_path
        );
        eprintln!("Checked paths:");
        eprintln!("  - {}", main_path.display());
        eprintln!("  - {}", lib_path.display());
        std::process::exit(1);
    };

    let specification_path = base_dir.join("architecture.json");

    // Verify the specification file exists before trying to parse
    if !specification_path.exists() || !specification_path.is_file() {
        eprintln!(
            "Specification file not found at '{}'.",
            specification_path.display()
        );
        eprintln!("Base directory: {}", base_dir.display());
        std::process::exit(1);
    }

    let specification = parse_specification(&specification_path);

    if let Ok(architecture) = specification {
        // Verify the source file exists before trying to parse
        if !Path::new(&root_path).exists() {
            eprintln!("Source file not found: {}", root_path);
            std::process::exit(1);
        }

        let module_tree = ModuleTree::new(&root_path);

        // Validate access rules first
        if let Err(err) = architecture.validate_access_rules() {
            err.print(module_tree.tree());
            std::process::exit(1);
        }

        if let Err(err) = architecture.validate_declared_layers_exist(&module_tree) {
            err.print(module_tree.tree());
            std::process::exit(1);
        }

        // Check access rules
        if let Err(err) = architecture.check_access_rules(&module_tree) {
            err.print(module_tree.tree());
            std::process::exit(1);
        }

        // Check complete layer specification if requested
        if check_for_complete_layer_specification {
            if let Err(err) = architecture.check_complete_layer_specification(&module_tree) {
                err.print(module_tree.tree());
                std::process::exit(1);
            }
        }
    } else {
        eprintln!(
            "Specification file cant be opened for '{}'.",
            directory_path
        );
        eprintln!(
            "Expected architecture.json at: {}",
            specification_path.display()
        );
        std::process::exit(1);
    }
}
