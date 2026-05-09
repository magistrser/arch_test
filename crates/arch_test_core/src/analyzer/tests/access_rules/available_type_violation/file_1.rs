// This file uses an external crate in a type position (return type)
// that is NOT in the allowed list
// Expected: should trigger a violation when Available rule is checked
fn fun_1() -> serde_json::Value {
    serde_json::Value::Null
}

struct MyStruct {
    field: serde_json::Value,
}
