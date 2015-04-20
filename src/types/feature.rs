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
use rustc_serialize::json::{Json, Object, ToJson};
use {Geometry, GeoJsonResult, FromJson};

/// Feature
///
/// [GeoJSON Format Specification § 2.2](http://geojson.org/geojson-spec.html#feature-objects)
#[derive(Debug)]
pub struct Feature {
    pub geometry: Geometry,
    pub properties: Option<Object>,
    pub id: Option<String>,  // todo: change this to &str?
}

impl ToJson for Feature {
    fn to_json(&self) -> Json {
        let mut d = HashMap::new();
        d.insert("type".to_string(), "Feature".to_json());
        d.insert("geometry".to_string(), self.geometry.to_json());
        d.insert("properties".to_string(), self.properties.to_json());
        if let Some(ref id) = self.id {
            d.insert("id".to_string(), id.to_json());
        }
        d.to_json()
    }
}

impl FromJson for Feature {
    fn from_json(json_feature: &Object) -> GeoJsonResult<Self> {
        let geometry_json = expect_object!(expect_property!(json_feature, "geometry", "Missing 'geometry' field"));
        let properties_json = expect_property!(json_feature, "properties", "missing 'properties' field");
        let properties = match *properties_json {
            Json::Object(ref x) => Some(x.clone()),
            Json::Null => None,
            _ => panic!("expected an Object or Null value for feature properties"),
        };
        let id = json_feature.get("id").and_then(|id| id.as_string()).map(|id| id.to_string());
        Ok(Feature{
            geometry: try!(Geometry::from_json(geometry_json)),
            properties: properties,
            id: id,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use rustc_serialize::json::{Json, ToJson};
    use {Geometry, Feature, Poly, MultiPolygon, Pos, Ring, Point, FromJson};

    #[test]
    fn test_feature_to_json() {
        let mut map = BTreeMap::new();
        map.insert("hi".to_string(), "there".to_json());
        let point = Feature {
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
        properties: Some(map),
        id: None

        };
        let json_string = point.to_json().to_string();
        assert_eq!("{\"geometry\":{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"},\"properties\":{\"hi\":\"there\"},\"type\":\"Feature\"}", json_string);
    }

    #[test]
    fn test_feature_with_id() {
        let id = Some("1".to_string());
        let feature = Feature {
            id: id.clone(),
            geometry: Geometry::Point(Point{coordinates: Pos(vec![1., 2., 3.])}),
            properties: None,
        };

        let string = feature.to_json().to_string();
        assert_eq!("{\"geometry\":{\"coordinates\":[1.0,2.0,3.0],\"type\":\"Point\"},\"id\":\"1\",\"properties\":null,\"type\":\"Feature\"}", string);

        let json = Json::from_str(&string).unwrap();
        let object = match json {
            Json::Object(object) => object,
            _ => unreachable!(),
        };
        let feature = Feature::from_json(&object).ok().unwrap();
        assert_eq!(feature.id, id);
    }
}
