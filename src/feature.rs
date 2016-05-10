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

#[cfg(not(feature = "with-serde"))]
use ::json::ToJson;
#[cfg(feature = "with-serde")]
use ::json::{Serialize, Deserialize, Serializer, Deserializer, SerdeError};

use ::json::{JsonValue, JsonObject, json_val};
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
        let mut map = BTreeMap::new();
        map.insert(String::from("type"), json_val(&String::from("Feature")));
        map.insert(String::from("geometry"), json_val(&feature.geometry));
        if let Some(ref properties) = feature.properties {
            map.insert(String::from("properties"), json_val(properties));
        }
        if let Some(ref crs) = feature.crs {
            map.insert(String::from("crs"), json_val(crs));
        }
        if let Some(ref bbox) = feature.bbox {
            map.insert(String::from("bbox"), json_val(bbox));
        }
        if let Some(ref id) = feature.id {
            map.insert(String::from("id"), json_val(id));
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

#[cfg(not(feature = "with-serde"))]
impl ToJson for Feature {
    fn to_json(&self) -> ::rustc_serialize::json::Json {
        return ::rustc_serialize::json::Json::Object(self.into());
    }
}

#[cfg(feature = "with-serde")]
impl Serialize for Feature {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where S: Serializer {
        JsonObject::from(self).serialize(serializer)
    }
}

#[cfg(feature = "with-serde")]
impl Deserialize for Feature {
    fn deserialize<D>(deserializer: &mut D) -> Result<Feature, D::Error>
    where D: Deserializer {
        use std::error::Error as StdError;

        let val = try!(JsonValue::deserialize(deserializer));

        if let Some(feature) = val.as_object() {
            Feature::from_object(feature).map_err(|e| D::Error::custom(e.description()))
        }
        else {
            Err(D::Error::custom("expected json object"))
        }
    }
}


#[cfg(test)]
mod tests {
    use ::{Error, Feature, Geometry, Value, GeoJson};

    fn feature_json_str() -> &'static str {
        "{\"geometry\":{\"coordinates\":[1.1,2.1],\"type\":\"Point\"},\"properties\":{},\"type\":\"Feature\"}"
    }

    #[cfg(not(feature = "with-serde"))]
    fn properties() -> Option<::json::JsonObject> {
        Some(::rustc_serialize::json::Object::new())
    }
    #[cfg(feature = "with-serde")]
    fn properties() -> Option<::json::JsonObject> {
        use std::collections::BTreeMap;

        Some(BTreeMap::new())
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

    #[cfg(not(feature = "with-serde"))]
    fn encode(feature: &Feature) -> String {
        use rustc_serialize::json::{self, ToJson};

        json::encode(&feature.to_json()).unwrap()
    }
    #[cfg(feature = "with-serde")]
    fn encode(feature: &Feature) -> String {
        use serde_json;

        serde_json::to_string(&feature).unwrap()
    }

    #[cfg(not(feature = "with-serde"))]
    fn decode(json_string: String) -> GeoJson {
        json_string.parse().unwrap()
    }
    #[cfg(feature = "with-serde")]
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
