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

use crate::errors::{Error, Result};
use crate::{Feature, FeatureCollection, Geometry};
use crate::{JsonObject, JsonValue};
use serde::{Deserialize, Serialize, Serializer};
use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;

/// GeoJSON Objects
///
/// ```
/// use std::convert::TryInto;
/// use geojson::{Feature, GeoJson, Geometry, Value};
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
/// // Easily convert a feature to a GeoJson
/// let geojson: GeoJson = feature.into();
/// // and back again
/// let feature2: Feature = geojson.try_into().unwrap();
/// ```
/// [GeoJSON Format Specification § 3](https://tools.ietf.org/html/rfc7946#section-3)
#[derive(Clone, Debug, PartialEq, Deserialize)]
// Tagging is a pickle... we have a "type" field which works like a tag, and FeatureCollection.type and Feature.type are fine,
// but for a Geometry, the type is not "Geometry" rather it's one of the inner variants.
// We could use serdes `untagged` attribute, but if the geojson is invalid, we get an obtuse error like
// "did not match any variant of untagged enum GeoJson" when we really want something more specific like "`id` field had invalid value"
#[serde(from = "TaggedGeoJson")]
pub enum GeoJson {
    // this "tag" is probably wrong, because Geometry.type is not "Geometry", rather it's the value
    // of one of it's Value enum members.
    Geometry(Geometry),
    Feature(Feature),
    FeatureCollection(FeatureCollection),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum TaggedGeoJson {
    #[serde(deserialize_with = "crate::geometry::deserialize_point")]
    Point(Geometry),
    #[serde(deserialize_with = "crate::geometry::deserialize_line_string")]
    Linestring(Geometry),
    #[serde(deserialize_with = "crate::geometry::deserialize_polygon")]
    Polygon(Geometry),
    #[serde(deserialize_with = "crate::geometry::deserialize_multi_point")]
    MultiPoint(Geometry),
    #[serde(deserialize_with = "crate::geometry::deserialize_multi_line_string")]
    MultiLineString(Geometry),
    #[serde(deserialize_with = "crate::geometry::deserialize_multi_polygon")]
    MultiPolygon(Geometry),
    #[serde(deserialize_with = "crate::geometry::deserialize_geometry_collection")]
    GeometryCollection(Geometry),

    Feature(Feature),
    FeatureCollection(FeatureCollection),
}

impl From<TaggedGeoJson> for GeoJson {
    fn from(value: TaggedGeoJson) -> Self {
        use TaggedGeoJson::*;
        match value {
            Point(g)
            | Linestring(g)
            | Polygon(g)
            | MultiPoint(g)
            | MultiLineString(g)
            | MultiPolygon(g)
            | GeometryCollection(g) => GeoJson::Geometry(g),
            Feature(f) => GeoJson::Feature(f),
            FeatureCollection(fc) => GeoJson::FeatureCollection(fc),
        }
    }
}

impl<'a> From<&'a GeoJson> for JsonObject {
    fn from(geojson: &'a GeoJson) -> JsonObject {
        match *geojson {
            GeoJson::Geometry(ref geometry) => geometry.into(),
            GeoJson::Feature(ref feature) => feature.into(),
            GeoJson::FeatureCollection(ref fc) => fc.into(),
        }
    }
}

impl From<GeoJson> for JsonValue {
    fn from(geojson: GeoJson) -> JsonValue {
        match geojson {
            GeoJson::Geometry(geometry) => JsonValue::Object(JsonObject::from(&geometry)),
            GeoJson::Feature(feature) => JsonValue::Object(JsonObject::from(&feature)),
            GeoJson::FeatureCollection(fc) => JsonValue::Object(JsonObject::from(&fc)),
        }
    }
}

impl<G: Into<Geometry>> From<G> for GeoJson {
    fn from(geometry: G) -> Self {
        GeoJson::Geometry(geometry.into())
    }
}

impl<G: Into<Geometry>> FromIterator<G> for GeoJson {
    fn from_iter<I: IntoIterator<Item = G>>(iter: I) -> Self {
        use crate::Value;
        let geometries = iter.into_iter().map(|g| g.into()).collect();
        let collection = Value::GeometryCollection(geometries);
        GeoJson::Geometry(Geometry::new(collection))
    }
}

impl From<Feature> for GeoJson {
    fn from(feature: Feature) -> Self {
        GeoJson::Feature(feature)
    }
}

impl From<FeatureCollection> for GeoJson {
    fn from(feature_collection: FeatureCollection) -> GeoJson {
        GeoJson::FeatureCollection(feature_collection)
    }
}

impl TryFrom<GeoJson> for Geometry {
    type Error = Error;
    fn try_from(value: GeoJson) -> Result<Self> {
        match value {
            GeoJson::Geometry(g) => Ok(g),
            GeoJson::Feature(_) => Err(Error::ExpectedType {
                expected: "Geometry".to_string(),
                actual: "Feature".to_string(),
            }),
            GeoJson::FeatureCollection(_) => Err(Error::ExpectedType {
                expected: "Geometry".to_string(),
                actual: "FeatureCollection".to_string(),
            }),
        }
    }
}

impl TryFrom<GeoJson> for Feature {
    type Error = Error;
    fn try_from(value: GeoJson) -> Result<Self> {
        match value {
            GeoJson::Geometry(_) => Err(Error::ExpectedType {
                expected: "Feature".to_string(),
                actual: "Geometry".to_string(),
            }),
            GeoJson::Feature(f) => Ok(f),
            GeoJson::FeatureCollection(_) => Err(Error::ExpectedType {
                expected: "Feature".to_string(),
                actual: "FeatureCollection".to_string(),
            }),
        }
    }
}

impl TryFrom<GeoJson> for FeatureCollection {
    type Error = Error;
    fn try_from(value: GeoJson) -> Result<Self> {
        match value {
            GeoJson::Geometry(_) => Err(Error::ExpectedType {
                expected: "FeatureCollection".to_string(),
                actual: "Geometry".to_string(),
            }),
            GeoJson::Feature(_) => Err(Error::ExpectedType {
                expected: "FeatureCollection".to_string(),
                actual: "Feature".to_string(),
            }),
            GeoJson::FeatureCollection(f) => Ok(f),
        }
    }
}

impl GeoJson {
    pub fn from_json_object(object: JsonObject) -> Result<Self> {
        Self::try_from(object)
    }

    /// Converts a JSON Value into a GeoJson object.
    ///
    /// # Example
    /// ```
    /// use std::convert::TryInto;
    /// use geojson::{Feature, GeoJson, Geometry, Position, Value};
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
    /// let geojson: GeoJson = json_value.try_into().unwrap();
    ///
    /// assert_eq!(
    ///     geojson,
    ///     GeoJson::Feature(Feature {
    ///         bbox: None,
    ///         geometry: Some(Geometry::new(Value::Point(Position::from([102.0, 0.5])))),
    ///         id: None,
    ///         properties: None,
    ///         foreign_members: None,
    ///     })
    /// );
    /// ```
    pub fn from_json_value(value: JsonValue) -> Result<Self> {
        Self::try_from(value)
    }

    /// Convenience method to convert to a JSON Value. Uses `From`.
    /// ```
    /// use std::convert::TryFrom;
    /// use geojson::GeoJson;
    /// use serde_json::json;
    ///
    /// let geojson = GeoJson::try_from( json!({
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

    // Deserialize a GeoJson object from an IO stream of JSON
    pub fn from_reader<R>(rdr: R) -> serde_json::Result<Self>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(rdr)
    }
}

impl TryFrom<JsonObject> for GeoJson {
    type Error = Error;

    fn try_from(object: JsonObject) -> Result<Self> {
        let type_ = match object.get("type") {
            Some(JsonValue::String(t)) => Type::from_str(t),
            _ => return Err(Error::GeometryUnknownType("type".to_owned())),
        };
        let type_ = type_.ok_or(Error::EmptyType)?;
        match type_ {
            Type::Feature => Feature::try_from(object).map(GeoJson::Feature),
            Type::FeatureCollection => {
                FeatureCollection::try_from(object).map(GeoJson::FeatureCollection)
            }
            _ => Geometry::try_from(object).map(GeoJson::Geometry),
        }
    }
}

impl TryFrom<JsonValue> for GeoJson {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self> {
        if let JsonValue::Object(obj) = value {
            Self::try_from(obj)
        } else {
            Err(Error::GeoJsonExpectedObject(value))
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

impl Serialize for GeoJson {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        JsonObject::from(self).serialize(serializer)
    }
}

// impl<'de> Deserialize<'de> for GeoJson {
//     fn deserialize<D>(deserializer: D) -> std::result::Result<GeoJson, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         use serde::de::Error as SerdeError;
//
//         let val = JsonObject::deserialize(deserializer)?;
//
//         GeoJson::from_json_object(val).map_err(|e| D::Error::custom(e.to_string()))
//     }
// }

/// # Example
///```
/// use geojson::GeoJson;
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
/// let geo_json = GeoJson::from_str(&geojson_str).unwrap();
/// if let GeoJson::FeatureCollection(collection) = geo_json {
///     assert_eq!(1, collection.features.len());
/// } else {
///     panic!("expected feature collection");
/// }
/// ```
impl FromStr for GeoJson {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_reader(s.as_bytes())?)
    }
}

impl fmt::Display for GeoJson {
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
    use crate::{Error, Feature, FeatureCollection, GeoJson, Geometry, Position, Value};
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

        let g1 = GeoJson::from_reader(json_str.as_bytes()).unwrap();

        let json_value = json!({
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [102.0, 0.5]
            },
            "properties": null,
        });

        let g2: GeoJson = json_value.try_into().unwrap();

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

        let geojson: GeoJson = json_value.try_into().unwrap();

        assert_eq!(
            geojson,
            GeoJson::Feature(Feature {
                bbox: None,
                geometry: Some(Geometry::new(Value::Point(Position::from([102.0, 0.5])))),
                id: None,
                properties: None,
                foreign_members: None,
            })
        );
    }

    #[test]
    fn test_missing_properties_key() {
        let json_value = json!({
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [102.0, 0.5]
            },
        });

        assert!(json_value.is_object());

        let geojson: GeoJson = json_value.try_into().unwrap();
        assert_eq!(
            geojson,
            GeoJson::Feature(Feature {
                bbox: None,
                geometry: Some(Geometry::new(Value::Point(Position::from([102.0, 0.5])))),
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
            GeoJson::from_str(geojson_str),
            Err(Error::MalformedJson(_))
        ))
    }

    #[test]
    fn countries() {
        let geojson_str = include_str!("../tests/fixtures/countries.geojson");
        let fc = geojson_str.parse::<FeatureCollection>().unwrap();
        assert_eq!(fc.features.len(), 180);
    }
}
