// Simple import
use serde_json::Value;
// Import with alias
use serde_json::Value as JsonValue;
// Multiple imports
use serde_json::{Map, Number};
// Conditional import (only for test configuration)
#[cfg(test)]
use serde_json::Serde;

// ============================================================================
// Standard library imports (requires white_list: std)
// ============================================================================

// Simple import
use std::collections::HashMap;
// Import with alias
use std::collections::HashMap as StdHashMap;
// Multiple imports
use std::collections::{BTreeMap, HashSet};
// Import trait
use std::fmt::Debug;
// Import with full path
use std::io::Read;

// ============================================================================
// 3. Local module imports (should be allowed by default - no white_list needed)
// ============================================================================

// Import via crate:: (explicit absolute path)
use crate::my_module::my_function;
use crate::my_module::MyType;
use crate::my_module::{MyType as LocalType, MY_CONST};

// Note: Relative imports without crate:: prefix (like `use my_module::...`)
// are treated as external crates by the current implementation
// They would need to be added to white_list or use crate:: prefix
// TODO fix this
// use my_module::my_another_function;
// use my_module::{MyAnotherType, ANOTHER_CONST};

// Self path (current module)
use self::inner_module::InnerType;
// Super path (parent module)
use super::sibling_module::SiblingType;

// ============================================================================
// Nested module for self:: testing
// ============================================================================

mod inner_module {
    pub struct InnerType {
        pub data: i32,
    }
}

// ============================================================================
// Usage of imported items in code
// ============================================================================

fn example_function() -> HashMap<String, Value> {
    let mut map = HashMap::new();
    map.insert("key".to_string(), Value::Null);

    let local = MyType { value: 42 };
    let _ = my_function();
    let _ = MY_CONST;

    let hashset: HashSet<i32> = HashSet::new();
    let _ = hashset;

    let json_value: Value = Value::Null;
    let _ = json_value;

    map
}

fn example_with_debug() {
    let debuggable = DebugStruct { value: 1 };
    let _ = format!("{:?}", debuggable);
}

fn example_with_io() {
    use std::io::Write;
    let _ = std::io::stdout();
}

struct DebugStruct {
    value: i32,
}

impl Debug for DebugStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DebugStruct {{ value: {} }}", self.value)
    }
}

type JsonMap = Map<String, Value>;
type StdStringHashMap = HashMap<String, String>;

struct ExampleStruct {
    json_field: JsonValue,
    std_map: StdHashMap<i32, i32>,
    local_type: LocalType,
}

impl ExampleStruct {
    fn new() -> Self {
        ExampleStruct {
            json_field: Value::Null,
            std_map: StdHashMap::new(),
            local_type: LocalType { value: 0 },
        }
    }
}
