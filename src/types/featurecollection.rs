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
use rustc_serialize::json::{Json, ToJson, Builder, Object};
use Feature;

/// FeatureCollection
///
/// [GeoJSON Format Specification § 2.3](http://geojson.org/geojson-spec.html#feature-collection-objects)
pub struct FeatureCollection {
    features: Vec<Feature>,
}

impl ToJson for FeatureCollection {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "FeatureCollection".to_json());
        d.insert(format!("features"), self.features.to_json());
        d.to_json()
    }
}

impl FeatureCollection {
    pub fn from_json(json_doc: &Object) -> FeatureCollection {
        assert_eq!(json_doc.get("type").unwrap().as_string().unwrap(), "FeatureCollection");
        let feature_array = json_doc
            .get("features").unwrap()
            .as_array().unwrap();
        let fs: Vec<Feature> = feature_array.iter().map(|f| Feature::from_json(f.as_object().unwrap())).collect();
        return FeatureCollection{features: fs};
    }
}

pub fn from_str(json_str: &str) -> FeatureCollection {
    let json_doc = Builder::new(json_str.chars()).build().unwrap();
    return FeatureCollection::from_json(json_doc.as_object().unwrap());
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use rustc_serialize::json::ToJson;
    use {FeatureCollection, Feature, MultiPolygon, Geometry, Poly, Pos, Ring, from_str};

    #[test]
    fn test_feature_collection_string_tojson() {
        let mut map = BTreeMap::new();
        map.insert(format!("hi"), "there".to_json());
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
                properties: map.to_json()
            }
        ]};
        let json_string = format!("{}",point.to_json());
        assert_eq!("{\"features\":[{\"geometry\":{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"},\"properties\":{\"hi\":\"there\"},\"type\":\"Feature\"}],\"type\":\"FeatureCollection\"}", json_string);
    }

    #[test]
    fn test_json_string_to_feature_collection() {
        let json_string = "{\"features\":[{\"geometry\":{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"},\"properties\":{\"hi\":\"there\"},\"type\":\"Feature\"}],\"type\":\"FeatureCollection\"}";
        let fc = from_str(json_string);
        assert_eq!(json_string, format!("{}", fc.to_json()));
    }
}
