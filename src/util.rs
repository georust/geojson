use std::collections::HashMap;
use rustc_serialize::json::{Json, ToJson};


pub fn new_geometry_object(type_: &'static str, coords: Json) -> Json {
    let mut d = HashMap::new();
    d.insert("type".to_string(), type_.to_json());
    d.insert("coordinates".to_string(), coords.to_json());
    d.to_json()
}
