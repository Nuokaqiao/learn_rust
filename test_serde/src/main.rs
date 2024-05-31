use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    #[serde(default)]
    age: u8,
    #[serde(default)]
    email: String,
}
impl Default for Person {
    fn default() -> Self {
        Person {
            name: String::new(),
            age: 0,
            email: String::new(),
        }
    }
}
fn main() {
    let person = Person {
        name: String::from("shi"),
        age: 18,
        email: String::from("test@gmail.com"),
    };
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized:{}", serialized);

    let json_data = r#"{"name":"hi_json"}"#;

    let deserialized: Person = serde_json::from_str(&json_data).unwrap();
    println!("Deserilized:{:?}", deserialized);
}
