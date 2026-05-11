// This file has various imports to test restricted rule comprehensively
// Uses std (allowed), serde_json (restricted), and local crate imports

use std::collections::HashMap;
use serde_json::Value;

mod inner {
    pub struct InnerType {
        pub id: u32,
    }
}

mod parent {
    pub struct ParentType {
        pub id: u32,
    }
}

// Local module imports - should be ignored by Restricted rule
use crate::analyzer::tests::access_rules::restricted_comprehensive::inner::InnerType;
use self::inner::InnerType as SelfInnerType;
use super::parent::ParentType;

// This uses a restricted crate - should trigger violation
fn fun_1() {
    let _a = serde_json::Value;
    let _b = Value;
}

// This uses only std - should NOT trigger violation
fn fun_2() {
    let _map: HashMap<String, u32> = HashMap::new();
}
