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
    pub fn from_json(json_pos: &Array) -> Pos {
        let vec = json_pos.iter()
            .map(|json_f64| json_f64.as_f64().unwrap())
            .collect();
        return Pos(vec);
    }
}
