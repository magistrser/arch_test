// This file uses an external crate that is NOT in the allowed list
// Expected: should trigger a violation when Available rule is checked
use serde::Serialize;

fn fun_1() {
    let _a = serde::json::Value;
}
