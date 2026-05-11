// This file uses serde which is in the restricted list
// Expected: should trigger a violation when Restricted rule is checked
use serde::Serialize;

fn fun_1() {
    let _a = serde::json::Value;
}
