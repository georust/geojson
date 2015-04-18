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
use {Feature, GeoJsonResult, GeoJsonError};

/// FeatureCollection
///
/// [GeoJSON Format Specification § 2.3](http://geojson.org/geojson-spec.html#feature-collection-objects)
#[derive(Debug)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
}

impl ToJson for FeatureCollection {
    fn to_json(&self) -> Json {
        let mut d = HashMap::new();
        d.insert("type".to_string(), "FeatureCollection".to_json());
        d.insert("features".to_string(), self.features.to_json());
        d.to_json()
    }
}

impl FeatureCollection {
    pub fn from_json(json_doc: &Object) -> GeoJsonResult<FeatureCollection> {
        let mut features = vec![];
        for feature_json in expect_array!(expect_property!(json_doc, "features", "Missing 'features' field")) {
            features.push(try!(Feature::from_json(expect_object!(feature_json))));
        }
        Ok(FeatureCollection{features: features})
    }

    pub fn from_str(json_str: &str) -> GeoJsonResult<FeatureCollection> {
        let json_doc = match Json::from_str(json_str) {
            Ok(v) => v,
            Err(_) => return Err(GeoJsonError::new("Error parsing JSON document")),
        };
        FeatureCollection::from_json(expect_object!(json_doc))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use rustc_serialize::json::ToJson;
    use {FeatureCollection, Feature, MultiPolygon, Geometry, Poly, Pos, Ring};

    #[test]
    fn test_feature_collection_to_json() {
        let mut map = BTreeMap::new();
        map.insert("hi".to_string(), "there".to_json());
        let point = FeatureCollection {
            features:vec![
              Feature {
                geometry: Geometry::MultiPolygon(MultiPolygon {
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
                }),
                properties: Some(map)
            }
        ]};
        let json_string = point.to_json().to_string();
        assert_eq!("{\"features\":[{\"geometry\":{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"},\"properties\":{\"hi\":\"there\"},\"type\":\"Feature\"}],\"type\":\"FeatureCollection\"}", json_string);
    }

    #[test]
    fn test_json_string_to_feature_collection() {
        let json_string = "{\"features\":[{\"geometry\":{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"},\"properties\":{\"hi\":\"there\"},\"type\":\"Feature\"}],\"type\":\"FeatureCollection\"}";
        let fc = FeatureCollection::from_str(json_string).ok().unwrap();
        assert_eq!(json_string, fc.to_json().to_string());
    }

    #[test]
    fn test_invalid_json() {
        match FeatureCollection::from_str("---") {
            Ok(_) => panic!(),
            Err(_) => ()
        }
    }
}
