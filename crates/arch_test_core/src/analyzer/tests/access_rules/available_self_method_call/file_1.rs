// Test file: Self::method() calls should NOT be flagged by Available rule
// even when the crate is not in the allowed list

pub struct MyStruct {
    pub value: i32,
}

impl MyStruct {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn method_a(&self) -> i32 {
        Self::helper_method(self.value)
    }

    pub fn method_b(&self) -> Self {
        Self::new(self.value * 2)
    }

    fn helper_method(val: i32) -> i32 {
        val + 1
    }
}

// Standalone function using Self:: for clarity
fn standalone_function() -> i32 {
    Self::helper_standalone()
}

fn helper_standalone() -> i32 {
    42
}

mod inner {
    use super::MyStruct;
    
    pub fn test_inner() {
        let _ = MyStruct::new(42);
        let s = MyStruct { value: 10 };
        let _ = s.method_a();
        let _ = s.method_b();
    }
}
