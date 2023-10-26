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

use std::convert::TryFrom;
use std::str::FromStr;

use crate::errors::{Error, Result};
use crate::{util, Feature, Geometry, Value};
use crate::{JsonObject, JsonValue};
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

impl<T> From<Geometry<T>> for Feature<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn from(geom: Geometry<T>) -> Feature<T> {
        Feature {
            bbox: geom.bbox.clone(),
            foreign_members: geom.foreign_members.clone(),
            geometry: Some(geom),
            id: None,
            properties: None,
        }
    }
}

impl<T> From<Value<T>> for Feature<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn from(val: Value<T>) -> Feature<T> {
        Feature {
            bbox: None,
            foreign_members: None,
            geometry: Some(Geometry::from(val)),
            id: None,
            properties: None,
        }
    }
}

impl<T> FromStr for Feature<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    type Err = Error<T>;

    fn from_str(s: &str) -> Result<Self, T> {
        Self::try_from(crate::GeoJson::from_str(s)?)
    }
}

impl<'a, T> From<&'a Feature<T>> for JsonObject
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn from(feature: &'a Feature<T>) -> JsonObject {
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

impl<T> Feature<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    pub fn from_json_object(object: JsonObject) -> Result<Self, T> {
        Self::try_from(object)
    }

    pub fn from_json_value(value: JsonValue) -> Result<Self, T> {
        Self::try_from(value)
    }

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

impl<T> TryFrom<JsonObject> for Feature<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    type Error = Error<T>;

    fn try_from(mut object: JsonObject) -> Result<Self, T> {
        let res = &*util::expect_type(&mut object)?;
        match res {
            "Feature" => Ok(Self {
                geometry: util::get_geometry(&mut object)?,
                properties: util::get_properties(&mut object)?,
                id: util::get_id(&mut object)?,
                bbox: util::get_bbox(&mut object)?,
                foreign_members: util::get_foreign_members(object)?,
            }),
            _ => Err(Error::NotAFeature(res.to_string())),
        }
    }
}

impl<T> TryFrom<JsonValue> for Feature<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    type Error = Error<T>;

    fn try_from(value: JsonValue) -> Result<Self, T> {
        if let JsonValue::Object(obj) = value {
            Self::try_from(obj)
        } else {
            Err(Error::GeoJsonExpectedObject(value))
        }
    }
}

impl<T> Serialize for Feature<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", "Feature")?;
        map.serialize_entry("geometry", &self.geometry)?;
        map.serialize_entry("properties", &self.properties)?;
        if let Some(ref bbox) = self.bbox {
            map.serialize_entry("bbox", bbox)?;
        }
        if let Some(ref id) = self.id {
            map.serialize_entry("id", id)?;
        }
        if let Some(ref foreign_members) = self.foreign_members {
            for (key, value) in foreign_members {
                map.serialize_entry(key, value)?;
            }
        }
        map.end()
    }
}

impl<'de, T> Deserialize<'de> for Feature<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Feature<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as SerdeError;

        let val = JsonObject::deserialize(deserializer)?;

        Feature::from_json_object(val).map_err(|e| D::Error::custom(e.to_string()))
    }
}

/// Feature identifier
///
/// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
#[derive(Clone, Debug, PartialEq)]
pub enum Id {
    String(String),
    Number(serde_json::Number),
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Id::String(ref s) => s.serialize(serializer),
            Id::Number(ref n) => n.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::JsonObject;
    use crate::{feature, Error, Feature, GeoJson, Geometry, Value};
    use serde_json::json;

    use std::str::FromStr;

    fn feature_json_str() -> &'static str {
        "{\"type\":\"Feature\",\"geometry\":{\"type\":\"Point\",\"coordinates\":[1.1,2.1]},\"properties\":{}}"
    }

    fn properties() -> Option<JsonObject> {
        Some(JsonObject::new())
    }

    fn feature() -> Feature {
        crate::Feature {
            geometry: Some(Geometry {
                value: value(),
                bbox: None,
                foreign_members: None,
            }),
            properties: properties(),
            bbox: None,
            id: None,
            foreign_members: None,
        }
    }

    fn value() -> Value {
        Value::Point(vec![1.1, 2.1])
    }

    fn geometry() -> Geometry {
        Geometry::new(value())
    }

    fn encode(feature: &Feature) -> String {
        serde_json::to_string(&feature).unwrap()
    }

    fn decode(json_string: String) -> GeoJson<f64> {
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
    fn try_from_value() {
        use serde_json::json;
        use std::convert::TryInto;

        let json_value = json!({
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [1.1, 2.1]
            },
            "properties": null,
        });
        assert!(json_value.is_object());

        let feature: Feature = json_value.try_into().unwrap();
        assert_eq!(
            feature,
            Feature {
                bbox: None,
                geometry: Some(geometry()),
                id: None,
                properties: None,
                foreign_members: None,
            }
        )
    }

    #[test]
    fn test_display_feature() {
        let f = feature().to_string();
        assert_eq!(f, "{\"type\":\"Feature\",\"geometry\":{\"type\":\"Point\",\"coordinates\":[1.1,2.1]},\"properties\":{}}");
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
            Error::FeatureInvalidGeometryValue(_) => (),
            _ => unreachable!(),
        }
    }

    #[test]
    fn encode_decode_feature_with_id_number() {
        let feature_json_str = "{\"type\":\"Feature\",\"geometry\":{\"type\":\"Point\",\"coordinates\":[1.1,2.1]},\"properties\":{},\"id\":0}";
        let feature = crate::Feature {
            geometry: Some(Geometry {
                value: Value::Point(vec![1.1, 2.1]),
                bbox: None,
                foreign_members: None,
            }),
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
        let feature_json_str = "{\"type\":\"Feature\",\"geometry\":{\"type\":\"Point\",\"coordinates\":[1.1,2.1]},\"properties\":{},\"id\":\"foo\"}";
        let feature = crate::Feature {
            geometry: Some(Geometry {
                value: Value::Point(vec![1.1, 2.1]),
                bbox: None,
                foreign_members: None,
            }),
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
        let feature_json_str = "{\"geometry\":{\"coordinates\":[1.1,2.1],\"type\":\"Point\"},\"id\":{},\"properties\":{},\"type\":\"Feature\"}";
        assert!(matches!(
            feature_json_str.parse::<GeoJson>(),
            Err(Error::FeatureInvalidIdentifierType(_))
        ));
    }

    #[test]
    fn decode_feature_with_invalid_id_type_null() {
        let feature_json_str = "{\"geometry\":{\"coordinates\":[1.1,2.1],\"type\":\"Point\"},\"id\":null,\"properties\":{},\"type\":\"Feature\"}";
        assert!(matches!(
            feature_json_str.parse::<GeoJson>(),
            Err(Error::FeatureInvalidIdentifierType(_))
        ));
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
        let feature = crate::Feature {
            geometry: Some(Geometry {
                value: Value::Point(vec![1.1, 2.1]),
                bbox: None,
                foreign_members: None,
            }),
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

        let feature = crate::Feature {
            geometry: Some(Value::Point(vec![1.1, 2.1]).into()),
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

        let feature = Feature::<f64>::from_str(&feature_json).unwrap();
        assert_eq!("Dinagat Islands", feature.property("name").unwrap());
    }

    #[test]
    fn test_from_str_with_unexpected_type() {
        let geometry_json = json!({
            "type": "Point",
            "coordinates": [125.6, 10.1]
        })
        .to_string();

        let actual_failure = Feature::<f64>::from_str(&geometry_json).unwrap_err();
        match actual_failure {
            Error::ExpectedType { actual, expected } => {
                assert_eq!(actual, "Geometry");
                assert_eq!(expected, "Feature");
            }
            e => panic!("unexpected error: {}", e),
        };
    }
}
