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

use rustc_serialize::json::{Json, ToJson, Object};
use std::collections::HashMap;
use {Ring, GeoJsonResult};

/// MultiLineString
///
/// [GeoJSON Format Specification § 2.1.5](http://geojson.org/geojson-spec.html#multilinestring)
#[derive(RustcEncodable, Clone, Debug)]
pub struct MultiLineString {
    pub coordinates: Vec<Ring>,
}

impl ToJson for MultiLineString {
    fn to_json(&self) -> Json {
        let mut d = HashMap::new();
        d.insert(format!("type"), "MultiLineString".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}

impl MultiLineString {
    pub fn from_json(json_geometry: &Object) -> GeoJsonResult<MultiLineString> {
        let mut coordinates = vec![];
        for json_ring in expect_array!(expect_property!(json_geometry, "coordinates", "missing 'coordinates' field")) {
            coordinates.push(try!(Ring::from_json(expect_array!(json_ring))));
        }
        return Ok(MultiLineString{coordinates: coordinates});
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json::{ToJson, Json};
    use {MultiLineString, Pos, Ring};

    #[test]
    fn test_multi_line_string_to_json() {
        let multi_line_string = MultiLineString{coordinates: vec![
            Ring(vec![Pos(vec![1., 2., 3.]), Pos(vec![2., 4., 3.])]),
            Ring(vec![Pos(vec![3., 2., 3.]), Pos(vec![2., 4., 3.])])
            ]};
        let json_string = format!("{}", multi_line_string.to_json());
        assert_eq!("{\"coordinates\":[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]],\"type\":\"MultiLineString\"}", json_string);
    }

    #[test]
    fn test_multi_line_string_from_json() {
        let json_string = "{\"coordinates\":[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]],\"type\":\"MultiLineString\"}";
        let json_doc = Json::from_str(json_string).unwrap();
        let multi_line_string = MultiLineString::from_json(json_doc.as_object().unwrap()).ok().unwrap();
        assert_eq!(json_string, format!("{}", multi_line_string.to_json()));
    }
}
