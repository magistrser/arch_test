// This file uses an external crate `serde` that is NOT in the allowed list for `domain` layer.
// Expected: should trigger a violation when Available rule is checked with allowed_crates: ["std"]
use serde::Serialize;

#[derive(Serialize)]
pub struct DeepNestedStruct {
    pub id: u32,
    pub name: String,
}
