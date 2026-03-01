// Copyright 2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str::FromStr;

use crate::errors::{Error, Result};
use crate::{Bbox, Geometry, GeometryValue, feature};
use crate::{JsonObject, JsonValue};
use serde::{Deserialize, Serialize};

/// Feature Object
///
/// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
///
/// # Examples
///
/// Serializing a `Feature` to a GeoJSON string:
///
/// ```
/// use geojson::{Feature, Geometry};
///
/// let feature = Feature::from(Geometry::new_point([1.0, 2.0]));
///
/// let geojson_string = feature.to_string();
/// assert_eq!(
///     geojson_string,
///     r#"{"type":"Feature","geometry":{"type":"Point","coordinates":[1.0,2.0]},"properties":null}"#
/// );
/// ```
///
/// Deserializing a GeoJSON string into a `Feature`:
///
/// ```
/// use geojson::{Feature, Geometry};
///
/// let geojson_str = r#"
/// {
///   "type": "Feature",
///   "geometry": {
///     "type": "Point",
///     "coordinates": [1.0, 2.0]
///   }
/// }"#;
///
/// let feature = geojson_str
///     .parse::<Feature>()
///     .expect("valid Feature GeoJSON");
///
/// let expected = Feature::from(Geometry::new_point([1.0, 2.0]));
/// assert_eq!(feature, expected);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", from = "deserialize::DeserializeFeatureHelper")]
pub struct Feature {
    /// Bounding Box
    ///
    /// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Bbox>,
    /// Geometry
    ///
    /// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
    pub geometry: Option<Geometry>,
    /// Identifier
    ///
    /// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<feature::Id>,
    /// Properties
    ///
    /// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
    ///
    /// NOTE: This crate will permissively parse a Feature whose json is missing a `properties` key.
    /// Because the spec implies that the `properties` key must be present, we will always include
    /// the `properties` key when serializing.
    pub properties: Option<JsonObject>,
    /// Foreign Members
    ///
    /// [GeoJSON Format Specification § 6](https://tools.ietf.org/html/rfc7946#section-6.1)
    ///
    /// See the [crate-level foreign members documentation](crate#foreign-members) for details,
    /// including limitations on key names.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub foreign_members: Option<JsonObject>,
}

mod deserialize {
    use super::*;
    use crate::util::normalize_foreign_members;

    /// The purpose of this helper is to verify that `"type": "Feature"` during
    /// deserialization by explicitly encoding the type as an enum with one member.
    ///
    /// It's dumb, but otherwise serde will ignore the `#[serde(tag="type")]`, and happily
    /// deserialize (e.g.) `"type": "Point"` as a Feature.
    ///
    /// See: https://github.com/serde-rs/serde/issues/3028
    #[derive(Deserialize)]
    pub(crate) struct DeserializeFeatureHelper {
        #[allow(unused)]
        r#type: FeatureType,
        bbox: Option<Bbox>,
        geometry: Option<Geometry>,
        id: Option<feature::Id>,
        properties: Option<JsonObject>,
        #[serde(flatten)]
        foreign_members: Option<JsonObject>,
    }

    #[derive(Deserialize)]
    enum FeatureType {
        Feature,
    }

    impl From<DeserializeFeatureHelper> for Feature {
        fn from(mut value: DeserializeFeatureHelper) -> Self {
            normalize_foreign_members(&mut value.foreign_members);
            Self {
                bbox: value.bbox,
                geometry: value.geometry,
                id: value.id,
                properties: value.properties,
                foreign_members: value.foreign_members,
            }
        }
    }
}

impl From<Geometry> for Feature {
    fn from(geom: Geometry) -> Feature {
        Feature {
            bbox: geom.bbox.clone(),
            foreign_members: geom.foreign_members.clone(),
            geometry: Some(geom),
            id: None,
            properties: None,
        }
    }
}

impl From<GeometryValue> for Feature {
    fn from(val: GeometryValue) -> Feature {
        Feature {
            bbox: None,
            foreign_members: None,
            geometry: Some(Geometry::from(val)),
            id: None,
            properties: None,
        }
    }
}

impl FromStr for Feature {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl<'a> From<&'a Feature> for JsonObject {
    fn from(feature: &'a Feature) -> JsonObject {
        // The unwrap() should never panic, because Feature contains only JSON-serializable types
        match serde_json::to_value(feature).unwrap() {
            serde_json::Value::Object(obj) => obj,
            value => {
                // Panic should never happen, because `impl Serialize for Feature` always produces an
                // Object
                panic!(
                    "serializing Feature should result in an Object, but got something {:?}",
                    value
                )
            }
        }
    }
}

impl Feature {
    /// Return the value of this property, if it's set
    pub fn property(&self, key: impl AsRef<str>) -> Option<&JsonValue> {
        self.properties
            .as_ref()
            .and_then(|props| props.get(key.as_ref()))
    }

    /// Return true iff this key is set
    pub fn contains_property(&self, key: impl AsRef<str>) -> bool {
        match &self.properties {
            None => false,
            Some(props) => props.contains_key(key.as_ref()),
        }
    }

    /// Set a property to this value, overwriting any possible older value
    pub fn set_property(&mut self, key: impl Into<String>, value: impl Into<JsonValue>) {
        let key: String = key.into();
        let value: JsonValue = value.into();
        if self.properties.is_none() {
            self.properties = Some(JsonObject::new());
        }

        self.properties.as_mut().unwrap().insert(key, value);
    }

    /// Removes a key from the `properties` map, returning the value at the key if the key
    /// was previously in the `properties` map.
    pub fn remove_property(&mut self, key: impl AsRef<str>) -> Option<JsonValue> {
        self.properties
            .as_mut()
            .and_then(|props| props.remove(key.as_ref()))
    }

    /// The number of properties
    pub fn len_properties(&self) -> usize {
        match &self.properties {
            None => 0,
            Some(props) => props.len(),
        }
    }

    /// Returns an iterator over all the properties
    pub fn properties_iter(&self) -> Box<dyn ExactSizeIterator<Item = (&String, &JsonValue)> + '_> {
        match self.properties.as_ref() {
            None => Box::new(std::iter::empty()),
            Some(props) => Box::new(props.iter()),
        }
    }
}

/// Feature identifier
///
/// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged, expecting = "Feature 'id' must be a string or a number")]
pub enum Id {
    String(String),
    Number(serde_json::Number),
}

#[cfg(test)]
mod tests {
    use crate::{Error, Feature, GeoJson, Geometry, JsonObject, feature};
    use serde_json::json;

    use std::str::FromStr;

    fn feature_json_str() -> &'static str {
        "{\"type\":\"Feature\",\"geometry\":{\"type\":\"Point\",\"coordinates\":[1.1,2.1]},\"properties\":{}}"
    }

    fn properties() -> Option<JsonObject> {
        Some(JsonObject::new())
    }

    fn feature() -> Feature {
        Feature {
            geometry: Some(Geometry::new_point([1.1, 2.1])),
            properties: properties(),
            bbox: None,
            id: None,
            foreign_members: None,
        }
    }

    fn encode(feature: &Feature) -> String {
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
    fn parsing() {
        let geojson_str = json!({
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [1.1, 2.1]
            }
        })
        .to_string();
        let feature_1: Feature = geojson_str.parse().unwrap();
        let feature_2: Feature = serde_json::from_str(&geojson_str).unwrap();
        assert_eq!(feature_1, feature_2);

        let GeoJson::Feature(feature_3): GeoJson = geojson_str.parse().unwrap() else {
            panic!("unexpected GeoJSON type");
        };
        let GeoJson::Feature(feature_4): GeoJson = serde_json::from_str(&geojson_str).unwrap()
        else {
            panic!("unexpected GeoJSON type");
        };
        assert_eq!(feature_3, feature_4);

        assert_eq!(feature_1, feature_4);
    }

    #[test]
    fn wrong_type() {
        let geojson_str = json!({
            "type": "Point",
            "coordinates": [1.1, 2.1]
        })
        .to_string();
        Geometry::from_str(&geojson_str).unwrap();
        Feature::from_str(&geojson_str).unwrap_err();
        serde_json::from_str::<Feature>(&geojson_str).unwrap_err();
    }

    #[test]
    fn null_bbox() {
        let geojson_str = r#"{
            "geometry": null,
            "bbox": null,
            "properties":{},
            "type":"Feature"
        }"#;
        let geojson = geojson_str.parse::<GeoJson>().unwrap();
        let feature = match geojson {
            GeoJson::Feature(feature) => feature,
            _ => unimplemented!(),
        };
        assert!(feature.bbox.is_none());
    }

    #[test]
    fn test_display_feature() {
        let f = feature().to_string();
        assert_eq!(
            f,
            "{\"type\":\"Feature\",\"geometry\":{\"type\":\"Point\",\"coordinates\":[1.1,2.1]},\"properties\":{}}"
        );
    }

    #[test]
    fn feature_json_null_geometry() {
        let geojson_str = json!({
            "type":"Feature",
            "geometry": null,
            "properties":{}
        })
        .to_string();
        let geojson = geojson_str.parse::<GeoJson>().unwrap();
        let feature = match geojson {
            GeoJson::Feature(feature) => feature,
            _ => unimplemented!(),
        };
        assert!(feature.geometry.is_none());
    }

    #[test]
    fn feature_json_invalid_geometry() {
        let geojson_str = json!({
            "type": "Feature",
            "geometry": 3.15,
            "properties": {}
        })
        .to_string();
        let err = geojson_str.parse::<GeoJson>().unwrap_err();
        let Error::MalformedGeoJson(serde_err) = err else {
            panic!("expected serde error");
        };
        assert!(serde_err.to_string().contains("expected Geometry object"));
    }

    #[test]
    fn encode_decode_feature_with_id_number() {
        let feature_json_str = r#"{"type":"Feature","geometry":{"type":"Point","coordinates":[1.1,2.1]},"id":0,"properties":{}}"#;
        let feature = Feature {
            geometry: Some(Geometry::new_point([1.1, 2.1])),
            properties: properties(),
            bbox: None,
            id: Some(feature::Id::Number(0.into())),
            foreign_members: None,
        };
        // Test encode
        let json_string = encode(&feature);
        assert_eq!(json_string, feature_json_str);

        // Test decode
        let decoded_feature = match decode(feature_json_str.into()) {
            GeoJson::Feature(f) => f,
            _ => unreachable!(),
        };
        assert_eq!(decoded_feature, feature);
    }

    #[test]
    fn encode_decode_feature_with_id_string() {
        let feature_json_str = r#"{"type":"Feature","geometry":{"type":"Point","coordinates":[1.1,2.1]},"id":"foo","properties":{}}"#;
        let feature = Feature {
            geometry: Some(Geometry::new_point([1.1, 2.1])),
            properties: properties(),
            bbox: None,
            id: Some(feature::Id::String("foo".into())),
            foreign_members: None,
        };
        // Test encode
        let json_string = encode(&feature);
        assert_eq!(json_string, feature_json_str);

        // Test decode
        let decoded_feature = match decode(feature_json_str.into()) {
            GeoJson::Feature(f) => f,
            _ => unreachable!(),
        };
        assert_eq!(decoded_feature, feature);
    }

    #[test]
    fn decode_feature_with_invalid_id_type_object() {
        let geojson_str = json!({
            "type":"Feature",
            "id":{},
            "geometry":{"coordinates":[1.1,2.1],"type":"Point"},
            "properties":{}
        })
        .to_string();
        let err = geojson_str.parse::<GeoJson>().unwrap_err();
        let Error::MalformedGeoJson(serde_err) = err else {
            panic!("expected serde error");
        };
        assert!(
            serde_err
                .to_string()
                .contains("Feature 'id' must be a string or a number")
        );
    }

    #[test]
    fn decode_feature_with_id_null() {
        // The spec states that feature id is optional, and that "is either a JSON string or number".
        // So the spec doesn't explicitly allow "null" but we treat `"id": null` as if no id were set.
        let geojson_str = json!({
            "type": "Feature",
            "id": null,
            "geometry": { "type": "Point", "coordinates": [1.1,2.1] },
            "properties":{},
        })
        .to_string();
        let GeoJson::Feature(feature) = geojson_str.parse::<GeoJson>().unwrap() else {
            panic!("expected Feature");
        };
        assert!(feature.id.is_none(), "Expected id to be None for null id");
    }

    #[test]
    fn encode_decode_feature_with_foreign_member() {
        use crate::JsonObject;
        use serde_json;
        let feature_json_str = "{\"type\":\"Feature\",\"geometry\":{\"type\":\"Point\",\"coordinates\":[1.1,2.1]},\"properties\":{},\"other_member\":\"some_value\"}";

        let mut foreign_members = JsonObject::new();
        foreign_members.insert(
            String::from("other_member"),
            serde_json::to_value("some_value").unwrap(),
        );
        let feature = Feature {
            geometry: Some(Geometry::new_point([1.1, 2.1])),
            properties: properties(),
            bbox: None,
            id: None,
            foreign_members: Some(foreign_members),
        };
        // Test encode
        let json_string = encode(&feature);
        assert_eq!(json_string, feature_json_str);

        // Test decode
        let decoded_feature = match decode(feature_json_str.into()) {
            GeoJson::Feature(f) => f,
            _ => unreachable!(),
        };
        assert_eq!(decoded_feature, feature);
    }

    #[test]
    fn encode_decode_feature_with_null_properties() {
        let feature_json_str = r#"{"type":"Feature","geometry":{"type":"Point","coordinates":[1.1,2.1]},"properties":null}"#;

        let feature = Feature {
            geometry: Some(Geometry::new_point([1.1, 2.1])),
            properties: None,
            bbox: None,
            id: None,
            foreign_members: None,
        };
        // Test encode
        let json_string = encode(&feature);
        assert_eq!(json_string, feature_json_str);

        // Test decode
        let decoded_feature = match decode(feature_json_str.into()) {
            GeoJson::Feature(f) => f,
            _ => unreachable!(),
        };
        assert_eq!(decoded_feature, feature);
    }

    #[test]
    fn feature_ergonomic_property_access() {
        use serde_json::json;

        let mut feature = feature();

        assert_eq!(feature.len_properties(), 0);
        assert_eq!(feature.property("foo"), None);
        assert!(!feature.contains_property("foo"));
        assert_eq!(feature.properties_iter().collect::<Vec<_>>(), vec![]);

        feature.set_property("foo", 12);
        assert_eq!(feature.property("foo"), Some(&json!(12)));
        assert_eq!(feature.len_properties(), 1);
        assert!(feature.contains_property("foo"));
        assert_eq!(
            feature.properties_iter().collect::<Vec<_>>(),
            vec![(&"foo".to_string(), &json!(12))]
        );

        assert_eq!(Some(json!(12)), feature.remove_property("foo"));
        assert_eq!(feature.property("foo"), None);
        assert_eq!(feature.len_properties(), 0);
        assert!(!feature.contains_property("foo"));
        assert_eq!(feature.properties_iter().collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_from_str_ok() {
        let feature_json = json!({
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [125.6, 10.1]
            },
            "properties": {
                "name": "Dinagat Islands"
            }
        })
        .to_string();

        let feature = Feature::from_str(&feature_json).unwrap();
        assert_eq!("Dinagat Islands", feature.property("name").unwrap());
    }

    #[test]
    fn test_id_serialize_string() {
        let id = feature::Id::String("foo".to_string());
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"foo\"");
    }

    #[test]
    fn test_id_serialize_number_integer() {
        let id = feature::Id::Number(42.into());
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "42");
    }
}
