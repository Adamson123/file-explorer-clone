use std::time::SystemTime;

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

pub fn get_field_as_int(value: &Value, field: &str) -> i64 {
    let def = json!({});
    value.get(field).unwrap_or(&def).as_i64().unwrap_or(0)
}

pub fn get_field_as_bool(value: &Value, field: &str) -> bool {
    let def = json!({});
    value.get(field).unwrap_or(&def).as_bool().unwrap_or(false)
}

pub fn get_time(f: &SystemTime, t: &SystemTime) -> i64 {
    f.clone().duration_since(t.clone()).unwrap().as_millis() as i64
}
