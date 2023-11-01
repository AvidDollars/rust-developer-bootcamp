// cargo add serde --features derive
use serde::{Serialize, Deserialize};

#[derive(Default, Debug)]
#[derive(Serialize, Deserialize)] // Serde
struct Pt {
    x: i32,
    y: i32,
}

pub fn hello_serde() {
    let pt = Pt::default();
    let point = serde_json::to_string(&pt).unwrap();
    println!("{point}");
    let deser: Pt = serde_json::from_str(r#"{"x":1,"y":2}"#).unwrap();
    println!("{:?}", deser);
}