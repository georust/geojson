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

use crate::json::{self, Deserialize, Deserializer, JsonObject, JsonValue, Serialize, Serializer};
use crate::serde;
use crate::{
    Error, FeatureBase, FeatureCollectionBase, GeometryBase,
    Position,
};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

/// GeoJSON Objects
///
/// [GeoJSON Format Specification § 3](https://tools.ietf.org/html/rfc7946#section-3)
#[derive(Clone, Debug, PartialEq)]
pub enum GeoJsonBase<Pos> {
    Geometry(GeometryBase<Pos>),
    Feature(FeatureBase<Pos>),
    FeatureCollection(FeatureCollectionBase<Pos>),
}

impl<'a, P: Position> From<&'a GeoJsonBase<P>> for JsonObject {
    fn from(geojson: &'a GeoJsonBase<P>) -> JsonObject {
        match *geojson {
            GeoJsonBase::Geometry(ref geometry) => geometry.into(),
            GeoJsonBase::Feature(ref feature) => feature.into(),
            GeoJsonBase::FeatureCollection(ref fc) => fc.into(),
        }
    }
}

impl<P: Position> From<GeometryBase<P>> for GeoJsonBase<P> {
    fn from(geometry: GeometryBase<P>) -> Self {
        GeoJsonBase::Geometry(geometry)
    }
}

impl<P: Position> From<FeatureBase<P>> for GeoJsonBase<P> {
    fn from(feature: FeatureBase<P>) -> Self {
        GeoJsonBase::Feature(feature)
    }
}

impl<P: Position> From<FeatureCollectionBase<P>> for GeoJsonBase<P> {
    fn from(feature_collection: FeatureCollectionBase<P>) -> GeoJsonBase<P> {
        GeoJsonBase::FeatureCollection(feature_collection)
    }
}

impl<P: Position> GeoJsonBase<P> {
    pub fn from_json_object(object: JsonObject) -> Result<Self, Error> {
        let type_ = match object.get("type") {
            Some(json::JsonValue::String(t)) => Type::from_str(t),
            _ => return Err(Error::ExpectedProperty("type".to_owned())),
        };
        let type_ = type_.ok_or(Error::GeoJsonUnknownType)?;
        match type_ {
            Type::Feature => FeatureBase::try_from(object).map(GeoJsonBase::Feature),
            Type::FeatureCollection => {
                FeatureCollectionBase::try_from(object).map(GeoJsonBase::FeatureCollection)
            }
            _ => GeometryBase::try_from(object).map(GeoJsonBase::Geometry),
        }
    }
}

impl<P: Position> GeoJsonBase<P> {
    /// Converts a JSON Value into a GeoJson object.
    ///
    /// # Example
    /// ```
    /// use std::convert::TryInto;
    /// use geojson::{Feature, GeoJson, Geometry, Value};
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
    ///         geometry: Some(Geometry::new(ValueBase::Point(vec![102.0, 0.5]))),
    ///         id: None,
    ///         properties: None,
    ///         foreign_members: None,
    ///     })
    /// );
    /// ```
    pub fn from_json_value(value: JsonValue) -> Result<Self, Error> {
        Self::try_from(value)
    }
}

impl<P: Position> TryFrom<JsonObject> for GeoJsonBase<P> {
    type Error = Error;

    fn try_from(object: JsonObject) -> Result<Self, Self::Error> {
        let type_ = match object.get("type") {
            Some(json::JsonValue::String(t)) => Type::from_str(t),
            _ => return Err(Error::ExpectedProperty("type".to_owned())),
        };
        let type_ = type_.ok_or(Error::GeoJsonUnknownType)?;
        match type_ {
            Type::Feature => FeatureBase::try_from(object).map(GeoJsonBase::Feature),
            Type::FeatureCollection => {
                FeatureCollectionBase::try_from(object).map(GeoJsonBase::FeatureCollection)
            }
            _ => GeometryBase::try_from(object).map(GeoJsonBase::Geometry),
        }
    }
}

impl<P: Position> TryFrom<JsonValue> for GeoJsonBase<P> {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        if let JsonValue::Object(obj) = value {
            Self::try_from(obj)
        } else {
            Err(Error::GeoJsonExpectedObject)
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

impl<P: Position> Serialize for GeoJsonBase<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        JsonObject::from(self).serialize(serializer)
    }
}

impl<'de, Pos: Position> Deserialize<'de> for GeoJsonBase<Pos> {
    fn deserialize<D>(deserializer: D) -> Result<GeoJsonBase<Pos>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as SerdeError;

        let val = JsonObject::deserialize(deserializer)?;

        GeoJsonBase::from_json_object(val).map_err(|e| D::Error::custom(e.to_string()))
    }
}

impl<P: Position> FromStr for GeoJsonBase<P> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let object = get_object(s)?;

        GeoJsonBase::from_json_object(object)
    }
}

fn get_object(s: &str) -> Result<json::JsonObject, Error> {
    ::serde_json::from_str(s)
        .ok()
        .and_then(json_value_into_json_object)
        .ok_or(Error::MalformedJson)
}

fn json_value_into_json_object(json_value: json::JsonValue) -> Option<json::JsonObject> {
    if let json::JsonValue::Object(geo) = json_value {
        Some(geo)
    } else {
        None
    }
}

impl<P: Position> fmt::Display for GeoJsonBase<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ::serde_json::to_string(self)
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

impl<P: Position> fmt::Display for FeatureBase<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ::serde_json::to_string(self)
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

impl<P: Position> fmt::Display for GeometryBase<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ::serde_json::to_string(self)
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

impl<P: Position> fmt::Display for FeatureCollectionBase<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ::serde_json::to_string(self)
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

#[cfg(test)]
mod tests {
    use crate::{FeatureBase, GeoJsonBase, GeometryBase, ValueBase};
    use serde_json::json;
    use std::convert::TryInto;

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

        let geojson: GeoJsonBase<(f64, f64)> = json_value.try_into().unwrap();

        assert_eq!(
            geojson,
            GeoJsonBase::Feature(FeatureBase {
                bbox: None,
                geometry: Some(GeometryBase::new(ValueBase::Point((102.0, 0.5)))),
                id: None,
                properties: None,
                foreign_members: None,
            })
        );
    }
}
