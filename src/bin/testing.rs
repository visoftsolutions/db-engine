use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "jeden")]
struct One {
    id: usize,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "dwa")]
struct Two {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum MyEnum {
    #[serde(rename = "jeden")]
    Variant1(One),
    #[serde(rename = "dwa")]
    Variant2(Two),
}

fn main() {
    let val = serde_json::to_string(&One { id: 15 }).unwrap();
    dbg!(&val);
    let result: Result<MyEnum, serde_json::Error> = serde_json::from_str(&val);

    match result {
        Ok(value) => println!("Deserialized: {:?}", value),
        Err(err) => println!("Error: {}", err),
    }
}
