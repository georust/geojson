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

use std::collections::HashMap;
use rustc_serialize::json::{Json, ToJson, Object};
use {Geometry, GeoJsonResult};

/// GeometryCollection
///
/// [GeoJSON Format Specification § 2.1.8](http://geojson.org/geojson-spec.html#geometry-collection)
#[derive(RustcEncodable, Clone, Debug)]
pub struct GeometryCollection {
    pub geometries: Vec<Geometry>,
}

impl ToJson for GeometryCollection {
    fn to_json(&self) -> Json {
        let mut d = HashMap::new();
        d.insert(format!("type"), "GeometryCollection".to_json());
        d.insert(format!("geometries"), self.geometries.to_json());
        d.to_json()
    }
}

impl GeometryCollection {
    pub fn from_json(json_geometry: &Object) -> GeoJsonResult<GeometryCollection> {
        let mut geometries = vec![];
        for json_geom in expect_array!(expect_property!(json_geometry, "geometries", "Missing 'geometries' field")) {
            geometries.push(try!(Geometry::from_json(expect_object!(json_geom))));
        }
        Ok(GeometryCollection{geometries: geometries})
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json::{ToJson, Json};
    use {GeometryCollection, MultiPolygon, Geometry, Poly, Pos, Ring};

    #[test]
    fn test_geometry_collection_to_json() {
        let geometry_collection = GeometryCollection{
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
        let json_string = format!("{}", geometry_collection.to_json());
        assert_eq!("{\"geometries\":[{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"}],\"type\":\"GeometryCollection\"}", json_string);
    }

    #[test]
    fn test_geometry_collection_from_json() {
        let json_string = "{\"geometries\":[{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"}],\"type\":\"GeometryCollection\"}";
        let json_doc = Json::from_str(json_string).unwrap();
        let geometry_collection = GeometryCollection::from_json(json_doc.as_object().unwrap()).ok().unwrap();
        assert_eq!(json_string, format!("{}", geometry_collection.to_json()));
    }
}
