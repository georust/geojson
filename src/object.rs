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

use crate::errors::Error;
use crate::json::{self, Deserialize, Deserializer, JsonObject, JsonValue, Serialize, Serializer};
use crate::serde;
use crate::{Feature, FeatureCollection, Geometry};
use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;

/// GeoJSON Objects
///
/// ```
/// use std::convert::TryInto;
/// use geojson::{Feature, Object, Geometry, Value};
/// use serde_json::json;
/// let json_value = json!({
///     "type": "Feature",
///     "geometry": {
///         "type": "Point",
///         "coordinates": [102.0, 0.5]
///     },
///     "properties": null,
/// });
/// let feature: Feature = json_value.try_into().unwrap();
///
/// // Easily convert a feature to a Object
/// let geojson: Object = feature.into();
/// // and back again
/// let feature2: Feature = geojson.try_into().unwrap();
/// ```
/// [GeoJSON Format Specification § 3](https://tools.ietf.org/html/rfc7946#section-3)
#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Geometry(Geometry),
    Feature(Feature),
    FeatureCollection(FeatureCollection),
}

impl<'a> From<&'a Object> for JsonObject {
    fn from(geojson: &'a Object) -> JsonObject {
        match *geojson {
            Object::Geometry(ref geometry) => geometry.into(),
            Object::Feature(ref feature) => feature.into(),
            Object::FeatureCollection(ref fc) => fc.into(),
        }
    }
}

impl From<Object> for JsonValue {
    fn from(geojson: Object) -> JsonValue {
        match geojson {
            Object::Geometry(geometry) => JsonValue::Object(JsonObject::from(&geometry)),
            Object::Feature(feature) => JsonValue::Object(JsonObject::from(&feature)),
            Object::FeatureCollection(fc) => JsonValue::Object(JsonObject::from(&fc)),
        }
    }
}

impl<G: Into<Geometry>> From<G> for Object {
    fn from(geometry: G) -> Self {
        Object::Geometry(geometry.into())
    }
}

impl<G: Into<Geometry>> FromIterator<G> for Object {
    fn from_iter<I: IntoIterator<Item = G>>(iter: I) -> Self {
        use crate::Value;
        let geometries = iter.into_iter().map(|g| g.into()).collect();
        let collection = Value::GeometryCollection(geometries);
        Object::Geometry(Geometry::new(collection))
    }
}

impl From<Feature> for Object {
    fn from(feature: Feature) -> Self {
        Object::Feature(feature)
    }
}

impl From<FeatureCollection> for Object {
    fn from(feature_collection: FeatureCollection) -> Object {
        Object::FeatureCollection(feature_collection)
    }
}

impl TryFrom<Object> for Geometry {
    type Error = Error;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Geometry(g) => Ok(g),
            Object::Feature(_) => Err(Error::ExpectedType {
                expected: "Geometry".to_string(),
                actual: "Feature".to_string(),
            }),
            Object::FeatureCollection(_) => Err(Error::ExpectedType {
                expected: "Geometry".to_string(),
                actual: "FeatureCollection".to_string(),
            }),
        }
    }
}

impl TryFrom<Object> for Feature {
    type Error = Error;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Geometry(_) => Err(Error::ExpectedType {
                expected: "Feature".to_string(),
                actual: "Geometry".to_string(),
            }),
            Object::Feature(f) => Ok(f),
            Object::FeatureCollection(_) => Err(Error::ExpectedType {
                expected: "Feature".to_string(),
                actual: "FeatureCollection".to_string(),
            }),
        }
    }
}

impl TryFrom<Object> for FeatureCollection {
    type Error = Error;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Geometry(_) => Err(Error::ExpectedType {
                expected: "FeatureCollection".to_string(),
                actual: "Geometry".to_string(),
            }),
            Object::Feature(_) => Err(Error::ExpectedType {
                expected: "FeatureCollection".to_string(),
                actual: "Feature".to_string(),
            }),
            Object::FeatureCollection(f) => Ok(f),
        }
    }
}

impl Object {
    pub fn from_json_object(object: JsonObject) -> Result<Self, Error> {
        Self::try_from(object)
    }

    /// Converts a JSON Value into a Object object.
    ///
    /// # Example
    /// ```
    /// use std::convert::TryInto;
    /// use geojson::{Feature, Object, Geometry, Value};
    /// use serde_json::json;
    ///
    /// let json_value = json!({
    ///     "type": "Feature",
    ///     "geometry": {
    ///         "type": "Point",
    ///         "coordinates": [102.0, 0.5]
    ///     },
    ///     "properties": null,
    /// });
    ///
    /// assert!(json_value.is_object());
    ///
    /// let geojson: Object = json_value.try_into().unwrap();
    ///
    /// assert_eq!(
    ///     geojson,
    ///     Object::Feature(Feature {
    ///         bbox: None,
    ///         geometry: Some(Geometry::new(Value::Point(vec![102.0, 0.5]))),
    ///         id: None,
    ///         properties: None,
    ///         foreign_members: None,
    ///     })
    /// );
    /// ```
    pub fn from_json_value(value: JsonValue) -> Result<Self, Error> {
        Self::try_from(value)
    }

    /// Convience method to convert to a JSON Value. Uses `From`.
    /// ```
    /// use std::convert::TryFrom;
    /// use geojson::Object;
    /// use serde_json::json;
    ///
    /// let geojson = Object::try_from( json!({
    ///        "type": "Feature",
    ///        "geometry": {
    ///            "type": "Point",
    ///            "coordinates": [102.0, 0.5]
    ///        },
    ///        "properties": {},
    ///     })).unwrap();
    ///
    /// let json_value = geojson.to_json_value();
    /// assert_eq!(json_value,
    ///     json!({
    ///        "type": "Feature",
    ///        "geometry": {
    ///            "type": "Point",
    ///            "coordinates": [102.0, 0.5]
    ///        },
    ///        "properties": {},
    ///     })
    ///    );
    /// ```
    pub fn to_json_value(self) -> JsonValue {
        JsonValue::from(self)
    }

    // Deserialize a Object object from an IO stream of JSON
    pub fn from_reader<R>(rdr: R) -> Result<Self, serde_json::Error>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(rdr)
    }
}

impl TryFrom<JsonObject> for Object {
    type Error = Error;

    fn try_from(object: JsonObject) -> Result<Self, Self::Error> {
        let type_ = match object.get("type") {
            Some(json::JsonValue::String(t)) => Type::from_str(t),
            _ => return Err(Error::GeometryUnknownType("type".to_owned())),
        };
        let type_ = type_.ok_or(Error::EmptyType)?;
        match type_ {
            Type::Feature => Feature::try_from(object).map(Object::Feature),
            Type::FeatureCollection => {
                FeatureCollection::try_from(object).map(Object::FeatureCollection)
            }
            _ => Geometry::try_from(object).map(Object::Geometry),
        }
    }
}

impl TryFrom<JsonValue> for Object {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        if let JsonValue::Object(obj) = value {
            Self::try_from(obj)
        } else {
            Err(Error::ExpectedObject(value))
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Type {
    Point,
    MultiPoint,
    LineString,
    MultiLineString,
    Polygon,
    MultiPolygon,
    GeometryCollection,
    Feature,
    FeatureCollection,
}

impl Type {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Point" => Some(Type::Point),
            "MultiPoint" => Some(Type::MultiPoint),
            "LineString" => Some(Type::LineString),
            "MultiLineString" => Some(Type::MultiLineString),
            "Polygon" => Some(Type::Polygon),
            "MultiPolygon" => Some(Type::MultiPolygon),
            "GeometryCollection" => Some(Type::GeometryCollection),
            "Feature" => Some(Type::Feature),
            "FeatureCollection" => Some(Type::FeatureCollection),
            _ => None,
        }
    }
}

impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        JsonObject::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Object {
    fn deserialize<D>(deserializer: D) -> Result<Object, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as SerdeError;

        let val = JsonObject::deserialize(deserializer)?;

        Object::from_json_object(val).map_err(|e| D::Error::custom(e.to_string()))
    }
}

/// # Example
///```
/// use geojson::Object;
/// use std::str::FromStr;
///
/// let geojson_str = r#"{
///   "type": "FeatureCollection",
///   "features": [
///     {
///       "type": "Feature",
///       "properties": {},
///       "geometry": {
///         "type": "Point",
///         "coordinates": [
///           -0.13583511114120483,
///           51.5218870403801
///         ]
///       }
///     }
///   ]
/// }
/// "#;
/// let geo_json = Object::from_str(&geojson_str).unwrap();
/// if let Object::FeatureCollection(collection) = geo_json {
///     assert_eq!(1, collection.features.len());
/// } else {
///     panic!("expected feature collection");
/// }
/// ```
impl FromStr for Object {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let object = get_object(s)?;

        Object::from_json_object(object)
    }
}

fn get_object(s: &str) -> Result<json::JsonObject, Error> {
    match ::serde_json::from_str(s) {
        Ok(json::JsonValue::Object(object)) => Ok(object),
        Ok(other) => Err(Error::ExpectedObjectValue(other)),
        Err(serde_error) => Err(Error::MalformedJson(serde_error)),
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ::serde_json::to_string(self)
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ::serde_json::to_string(self)
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

impl fmt::Display for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ::serde_json::to_string(self)
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

impl fmt::Display for FeatureCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ::serde_json::to_string(self)
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Feature, Geometry, Object, Value};
    use serde_json::json;
    use std::convert::TryInto;
    use std::str::FromStr;

    #[test]
    fn test_geojson_from_reader() {
        let json_str = r#"{
            "type": "Feature",
            "geometry": {
                    "type": "Point",
                    "coordinates": [102.0, 0.5]
            },
            "properties": null
        }"#;

        let g1 = Object::from_reader(json_str.as_bytes()).unwrap();

        let json_value = json!({
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [102.0, 0.5]
            },
            "properties": null,
        });

        let g2: Object = json_value.try_into().unwrap();

        assert_eq!(g1, g2);
    }

    #[test]
    fn test_geojson_from_value() {
        let json_value = json!({
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [102.0, 0.5]
            },
            "properties": null,
        });

        assert!(json_value.is_object());

        let geojson: Object = json_value.try_into().unwrap();

        assert_eq!(
            geojson,
            Object::Feature(Feature {
                bbox: None,
                geometry: Some(Geometry::new(Value::Point(vec![102.0, 0.5]))),
                id: None,
                properties: None,
                foreign_members: None,
            })
        );
    }

    #[test]
    fn test_invalid_json() {
        let geojson_str = r#"{
           "type": "FeatureCollection",
           "features": [
             !INTENTIONAL_TYPO! {
               "type": "Feature",
               "properties": {},
               "geometry": {
                 "type": "Point",
                 "coordinates": [
                   -0.13583511114120483,
                   51.5218870403801
                 ]
               }
             }
           ]
        }"#;
        assert!(matches!(
            Object::from_str(geojson_str),
            Err(Error::MalformedJson(_))
        ))
    }
}
