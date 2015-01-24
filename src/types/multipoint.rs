// Copyright 2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson, Object};
use Pos;

/// MultiPoint
///
/// [GeoJSON Format Specification § 2.1.3](http://geojson.org/geojson-spec.html#multipoint)
#[derive(RustcEncodable, Clone)]
pub struct MultiPoint {
    pub coordinates: Vec<Pos>,
}

impl ToJson for MultiPoint {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "MultiPoint".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}

impl MultiPoint {
    pub fn from_json(json_geometry: &Object) -> MultiPoint {
        let coordinates = json_geometry.get("coordinates").unwrap()
            .as_array().unwrap()
            .iter()
            .map(|json_pos| Pos::from_json(json_pos.as_array().unwrap()))
            .collect();
        return MultiPoint{coordinates: coordinates};
    }
}

#[test]
fn test_multi_point_tojson() {
    let point = MultiPoint {coordinates: vec![Pos(vec![1., 2., 3.])]};
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"coordinates\":[[1.0,2.0,3.0]],\"type\":\"MultiPoint\"}", json_string);
}

#[test]
fn test_multi_point_from_json() {
    let json_string = "{\"coordinates\":[[1.0,2.0,3.0]],\"type\":\"MultiPoint\"}";
    let json_doc = Json::from_str(json_string).unwrap();
    let multi_point = MultiPoint::from_json(json_doc.as_object().unwrap());
    assert_eq!(json_string, format!("{}", multi_point.to_json()));
}
