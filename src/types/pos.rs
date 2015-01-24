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

use rustc_serialize::json::{Json, ToJson, Array};
use {GeoJsonResult, GeoJsonError};

/// Pos (alias for Positions)
///
/// [GeoJSON Format Specification § 2.1.1](http://geojson.org/geojson-spec.html#positions)
#[derive(RustcEncodable, Clone)]
pub struct Pos(pub Vec<f64>);

impl ToJson for Pos {
    fn to_json(&self) -> Json {
        let &Pos(ref nums) = self;
        nums.to_json()
    }
}

impl Pos {
    pub fn from_json(json_pos: &Array) -> GeoJsonResult<Pos> {
        let mut vec = vec![];
        for json_f64 in json_pos.iter() {
            vec.push(try!(
                json_f64.as_f64()
                .ok_or(GeoJsonError::new("Expected f64 value"))
            ));
        }
        return Ok(Pos(vec));
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json::Json;
    use Pos;

    #[test]
    fn test_from_json_ok() {
        let json_str = "[1.0, 2.0, 3.0]";
        let json_pos = Json::from_str(json_str).unwrap();
        match Pos::from_json(json_pos.as_array().unwrap()) {
            Ok(Pos(v)) => assert_eq!(vec![1., 2., 3.], v),
            Err(_) => panic!(),
        };
    }

    #[test]
    fn test_from_json_err() {
        let json_str = "[null]";
        let json_pos = Json::from_str(json_str).unwrap();
        match Pos::from_json(json_pos.as_array().unwrap()) {
            Ok(_) => panic!(),
            Err(e) => assert_eq!(e.desc, "Expected f64 value"),
        };
    }
}
