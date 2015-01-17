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
use rustc_serialize::json::{Json, ToJson};
use Geometry;

/// GeometryCollection
///
/// [GeoJSON Format Specification § 2.1.8](http://geojson.org/geojson-spec.html#geometry-collection)
#[derive(RustcEncodable, Clone)]
pub struct GeometryCollection {
    geometries: Vec<Geometry>,
}

impl ToJson for GeometryCollection {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "GeometryCollection".to_json());
        d.insert(format!("geometries"), self.geometries.to_json());
        d.to_json()
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json::ToJson;
    use {GeometryCollection, MultiPolygon, Geometry, Poly, Pos, Ring};

    #[test]
    fn test_geometry_collection_string_tojson() {
        let point = GeometryCollection {
            geometries: vec![Geometry::MultiPolygon(MultiPolygon {
                        coordinates: vec![
                            Poly(vec![
                                Ring(vec![
                                    Pos(vec![1., 2., 3.]),
                                    Pos(vec![2., 4., 3.])
                                ]),
                                Ring(vec![
                                    Pos(vec![3., 2., 3.]),
                                    Pos(vec![2., 4., 3.])
                                ])
                            ])
                        ]
                    })
            ]
        };
        let json_string = format!("{}",point.to_json());
        assert_eq!("{\"geometries\":[{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"}],\"type\":\"GeometryCollection\"}", json_string);
    }
}
