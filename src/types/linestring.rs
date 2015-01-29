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
use Ring;

/// LineString
///
/// [GeoJSON Format Specification § 2.1.4](http://geojson.org/geojson-spec.html#linestring)
#[derive(RustcEncodable, Clone, Debug)]
pub struct LineString {
    pub coordinates: Ring,
}

impl ToJson for LineString {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "LineString".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}

impl LineString {
    pub fn from_json(json_geometry: &Object) -> LineString {
        let json_point = json_geometry.get("coordinates").unwrap();
        let coordinates = Ring::from_json(json_point.as_array().unwrap());
        return LineString{coordinates: coordinates};
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json::{ToJson, Json};
    use {Pos, LineString, Ring};

    #[test]
    fn test_line_string_to_json() {
        let line_string = LineString{coordinates: Ring(vec![Pos(vec![1., 2., 3.]), Pos(vec![2., 4., 3.])])};
        let json_string = format!("{}", line_string.to_json());
        assert_eq!("{\"coordinates\":[[1.0,2.0,3.0],[2.0,4.0,3.0]],\"type\":\"LineString\"}", json_string);
    }

    #[test]
    fn test_line_string_from_json() {
        let json_string = "{\"coordinates\":[[1.0,2.0,3.0],[2.0,4.0,3.0]],\"type\":\"LineString\"}";
        let json_doc = Json::from_str(json_string).unwrap();
        let line_string = LineString::from_json(json_doc.as_object().unwrap());
        assert_eq!(json_string, format!("{}", line_string.to_json()));
    }
}
