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
use Pos;

/// Ring
#[derive(RustcEncodable, Clone)]
pub struct Ring(pub Vec<Pos>);

impl ToJson for Ring {
    fn to_json(&self) -> Json {
       let &Ring(ref points) = self;
        points.to_json()
    }
}

impl Ring {
    pub fn from_json(json_ring: &Array) -> Ring {
        let vec = json_ring.iter()
            .map(|json_pos| Pos::from_json(json_pos.as_array().unwrap()).ok().unwrap())
            .collect();
        return Ring(vec);
    }
}
