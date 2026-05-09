mod file_1;
mod my_module;

mod sibling_module {
    pub struct SiblingType {
        pub id: u32,
    }
}

pub fn test() {
    // Empty function - test only checks imports
}
