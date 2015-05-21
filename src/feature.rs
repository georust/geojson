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

use rustc_serialize::json::{self, Json, ToJson};

use ::{Bbox, Crs, Error, FromObject, Geometry};
use ::util::ObjectUtils;


#[derive(Clone, Debug, PartialEq)]
pub struct Feature {
    pub bbox: Option<Bbox>,
    pub crs: Option<Crs>,
    pub geometry: Geometry,
    pub id: Option<json::Json>,
    pub properties: Option<json::Object>,
}

impl<'a> From<&'a Feature> for json::Object {
    fn from(feature: &'a Feature) -> json::Object {
        let mut map = BTreeMap::new();
        map.insert(String::from("type"), "Feature".to_json());
        map.insert(String::from("geometry"), feature.geometry.to_json());
        if let Some(ref properties) = feature.properties {
            map.insert(String::from("properties"), properties.to_json());
        }
        if let Some(ref crs) = feature.crs {
            map.insert(String::from("crs"), crs.to_json());
        }
        if let Some(ref bbox) = feature.bbox {
            map.insert(String::from("bbox"), bbox.to_json());
        }
        if let Some(ref id) = feature.id {
            map.insert(String::from("id"), id.to_json());
        }
        return map;
    }
}

impl FromObject for Feature {
    fn from_object(object: &json::Object) -> Result<Self, Error> {
        return Ok(Feature{
            geometry: try!(object.get_geometry()),
            properties: try!(object.get_properties()),
            id: try!(object.get_id()),
            crs: try!(object.get_crs()),
            bbox: try!(object.get_bbox()),
        });
    }
}

impl ToJson for Feature {
    fn to_json(&self) -> json::Json {
        return json::Json::Object(self.into());
    }
}


#[cfg(test)]
mod tests {
    use rustc_serialize::json::{self, ToJson};
    use super::super::{Feature, GeoJson, Geometry, Value};


    #[test]
    fn encode_decode_feature() {
        let feature_json_str = "{\"geometry\":{\"coordinates\":[1.0,2.0],\"type\":\"Point\"},\"properties\":{},\"type\":\"Feature\"}";
        let feature = Feature {
            geometry: Geometry {
                value: Value::Point(vec![1., 2.]),
                crs: None,
                bbox: None,
            },
            properties: Some(json::Object::new()),
            crs: None,
            bbox: None,
            id: None,
        };

        // Test encoding
        let json_string = json::encode(&feature.to_json()).unwrap();
        assert_eq!(json_string, feature_json_str);

        // Test decoding
        let decoded_feature = match json_string.parse() {
            Ok(GeoJson::Feature(f)) => f,
            _ => unreachable!(),
        };
        assert_eq!(decoded_feature, feature);
    }
}
