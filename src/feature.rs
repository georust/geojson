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

use ::json::{Serialize, Deserialize, Serializer, Deserializer, JsonValue, JsonObject};
use serde_json;
use ::{Bbox, Crs, Error, FromObject, Geometry, util};


/// Feature Objects
///
/// [GeoJSON Format Specification § 2.2]
/// (http://geojson.org/geojson-spec.html#feature-objects)
#[derive(Clone, Debug, PartialEq)]
pub struct Feature {
    pub bbox: Option<Bbox>,
    pub crs: Option<Crs>,
    pub geometry: Option<Geometry>,
    pub id: Option<JsonValue>,
    pub properties: Option<JsonObject>,
}

impl<'a> From<&'a Feature> for JsonObject {
    fn from(feature: &'a Feature) -> JsonObject {
        let mut map = JsonObject::new();
        map.insert(String::from("type"), json!("Feature"));
        map.insert(String::from("geometry"), serde_json::value::to_value(&feature.geometry).unwrap());
        if let Some(ref properties) = feature.properties {
            map.insert(String::from("properties"), serde_json::value::to_value(properties).unwrap());
        }
        if let Some(ref crs) = feature.crs {
            map.insert(String::from("crs"), serde_json::value::to_value(crs).unwrap());
        }
        if let Some(ref bbox) = feature.bbox {
            map.insert(String::from("bbox"), serde_json::value::to_value(bbox).unwrap());
        }
        if let Some(ref id) = feature.id {
            map.insert(String::from("id"), serde_json::value::to_value(id).unwrap());
        }

        return map;
    }
}

impl FromObject for Feature {
    fn from_object(object: &JsonObject) -> Result<Self, Error> {
        return Ok(Feature{
            geometry: try!(util::get_geometry(object)),
            properties: try!(util::get_properties(object)),
            id: try!(util::get_id(object)),
            crs: try!(util::get_crs(object)),
            bbox: try!(util::get_bbox(object)),
        });
    }
}

impl Serialize for Feature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        JsonObject::from(self).serialize(serializer)
    }
}

impl Deserialize for Feature {
    fn deserialize<D>(deserializer: D) -> Result<Feature, D::Error>
    where D: Deserializer {
        use std::error::Error as StdError;
        use serde::de::Error as SerdeError;

        let val = try!(JsonObject::deserialize(deserializer));

        Feature::from_object(&val).map_err(|e| D::Error::custom(e.description()))
    }
}


#[cfg(test)]
mod tests {
    use ::{Error, Feature, Geometry, Value, GeoJson};

    fn feature_json_str() -> &'static str {
        "{\"geometry\":{\"coordinates\":[1.1,2.1],\"type\":\"Point\"},\"properties\":{},\"type\":\"Feature\"}"
    }

    fn properties() -> Option<::json::JsonObject> {
        Some(::json::JsonObject::new())
    }

    fn feature() -> Feature {
        ::Feature {
            geometry: Some(Geometry {
                value: Value::Point(vec![1.1, 2.1]),
                crs: None,
                bbox: None,
            }),
            properties: properties(),
            crs: None,
            bbox: None,
            id: None,
        }
    }

    fn encode(feature: &Feature) -> String {
        use serde_json;

        serde_json::to_string(&feature).unwrap()
    }

    fn decode(json_string: String) -> GeoJson {
        json_string.parse().unwrap()
    }

    #[test]
    fn encode_decode_feature() {
        let feature = feature();

        // Test encoding
        let json_string = encode(&feature);
        assert_eq!(json_string, feature_json_str());

        // Test decoding
        let decoded_feature = match decode(json_string) {
            GeoJson::Feature(f) => f,
            _ => unreachable!(),
        };
        assert_eq!(decoded_feature, feature);
    }

    #[test]
    fn feature_json_null_geometry() {
        let geojson_str = r#"{
            "geometry": null,
            "properties":{},
            "type":"Feature"
        }"#;
        let geojson = geojson_str.parse::<GeoJson>().unwrap();
        let feature = match geojson {
            GeoJson::Feature(feature) => feature,
            _ => unimplemented!(),
        };
        assert!(feature.geometry.is_none());
    }

    #[test]
    fn feature_json_invalid_geometry() {
        let geojson_str = r#"{"geometry":3.14,"properties":{},"type":"Feature"}"#;
        match geojson_str.parse::<GeoJson>().unwrap_err() {
            Error::FeatureInvalidGeometryValue => (),
            _ => unreachable!(),
        }
    }
}
