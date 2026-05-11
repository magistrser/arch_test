use crate::restricted_layer::MyStruct;

mod restricted_layer;

fn main() {
    let data = MyStruct {
        id: 1,
        name: "test".to_string(),
    };
    // Using serde_json - this should be allowed
    let json = serde_json::to_string(&data).unwrap();
    println!("JSON: {}", json);
}
