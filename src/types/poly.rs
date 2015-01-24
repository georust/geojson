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
use {Ring, GeoJsonResult};

/// Poly  (alias for Polygon)
#[derive(RustcEncodable, Clone)]
pub struct Poly(pub Vec<Ring>);

impl ToJson for Poly {
    fn to_json(&self) -> Json {
        let &Poly(ref rings) = self;
        rings.to_json()
    }
}

impl Poly {
    pub fn from_json(json_poly: &Array) -> GeoJsonResult<Poly> {
        let mut vec = vec![];
        for json_ring in json_poly.iter() {
            vec.push(try!(Ring::from_json(expect_array!(json_ring))));
        }
        return Ok(Poly(vec));
    }
}
