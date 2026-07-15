use serde_json::{json, Value};

pub fn put_value_in_result(value: &Value) -> Result<String, String> {
    Ok(serde_json::to_string(value).unwrap_or(String::new()))
}

pub fn get_field_as_string(value: &Value, field: &str) -> String {
    let def = json!({});
    value
        .get(field)
        .unwrap_or(&def)
        .as_str()
        .unwrap_or("")
        .to_string()
}
