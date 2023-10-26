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
use std::{convert::TryFrom, fmt};

use crate::errors::{Error, Result};
use crate::{util, Bbox, LineStringType, PointType, PolygonType};
use crate::{JsonObject, JsonValue};
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

/// The underlying value for a `Geometry`.
///
/// # Conversion from `geo_types`
///
/// A `Value` can be created by using the `From` impl which is available for both `geo_types`
/// primitives AND `geo_types::Geometry` enum members:
///
/// ```rust
/// # #[cfg(feature = "geo-types")]
/// # fn test() {
/// let point = geo_types::Point::new(2., 9.);
/// let genum = geo_types::Geometry::from(point);
/// assert_eq!(
///     geojson::Value::from(&point),
///     geojson::Value::Point(vec![2., 9.]),
/// );
/// assert_eq!(
///     geojson::Value::from(&genum),
///     geojson::Value::Point(vec![2., 9.]),
/// );
/// # }
/// # #[cfg(not(feature = "geo-types"))]
/// # fn test() {}
/// # test()
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Value<T = f64>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    /// Point
    ///
    /// [GeoJSON Format Specification § 3.1.2](https://tools.ietf.org/html/rfc7946#section-3.1.2)
    Point(PointType<T>),

    /// MultiPoint
    ///
    /// [GeoJSON Format Specification § 3.1.3](https://tools.ietf.org/html/rfc7946#section-3.1.3)
    MultiPoint(Vec<PointType<T>>),

    /// LineString
    ///
    /// [GeoJSON Format Specification § 3.1.4](https://tools.ietf.org/html/rfc7946#section-3.1.4)
    LineString(LineStringType<T>),

    /// MultiLineString
    ///
    /// [GeoJSON Format Specification § 3.1.5](https://tools.ietf.org/html/rfc7946#section-3.1.5)
    MultiLineString(Vec<LineStringType<T>>),

    /// Polygon
    ///
    /// [GeoJSON Format Specification § 3.1.6](https://tools.ietf.org/html/rfc7946#section-3.1.6)
    Polygon(PolygonType<T>),

    /// MultiPolygon
    ///
    /// [GeoJSON Format Specification § 3.1.7](https://tools.ietf.org/html/rfc7946#section-3.1.7)
    MultiPolygon(Vec<PolygonType<T>>),

    /// GeometryCollection
    ///
    /// [GeoJSON Format Specification § 3.1.8](https://tools.ietf.org/html/rfc7946#section-3.1.8)
    GeometryCollection(Vec<Geometry<T>>),
}

impl<T> Value<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Point(..) => "Point",
            Value::MultiPoint(..) => "MultiPoint",
            Value::LineString(..) => "LineString",
            Value::MultiLineString(..) => "MultiLineString",
            Value::Polygon(..) => "Polygon",
            Value::MultiPolygon(..) => "MultiPolygon",
            Value::GeometryCollection(..) => "GeometryCollection",
        }
    }
}

impl<'a, T> From<&'a Value<T>> for JsonObject
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn from(value: &'a Value<T>) -> JsonObject {
        let mut map = JsonObject::new();
        map.insert(
            String::from("type"),
            // The unwrap() should never panic, because &str always serializes to JSON
            ::serde_json::to_value(value.type_name()).unwrap(),
        );
        map.insert(
            String::from(match value {
                Value::GeometryCollection(..) => "geometries",
                _ => "coordinates",
            }),
            // The unwrap() should never panic, because Value contains only JSON-serializable types
            ::serde_json::to_value(value).unwrap(),
        );
        map
    }
}

impl<T> Value<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    pub fn from_json_object(object: JsonObject) -> Result<Self, T> {
        Self::try_from(object)
    }

    pub fn from_json_value(value: JsonValue) -> Result<Self, T> {
        Self::try_from(value)
    }

    fn serialize_to_map<SM: SerializeMap>(
        &self,
        map: &mut SM,
    ) -> std::result::Result<(), SM::Error> {
        map.serialize_entry("type", self.type_name())?;
        map.serialize_entry(
            match self {
                Value::GeometryCollection(..) => "geometries",
                _ => "coordinates",
            },
            self,
        )?;
        Ok(())
    }
}

impl<T> TryFrom<JsonObject> for Value<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    type Error = Error<T>;

    fn try_from(mut object: JsonObject) -> Result<Self, T> {
        util::get_value(&mut object)
    }
}

impl<T> TryFrom<JsonValue> for Value<T>
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

impl<T> fmt::Display for Value<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ::serde_json::to_string(&JsonObject::from(self))
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

impl<'a, T> From<&'a Value<T>> for JsonValue
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn from(value: &'a Value<T>) -> JsonValue {
        ::serde_json::to_value(value).unwrap()
    }
}

impl<T> Serialize for Value<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Point(x) => x.serialize(serializer),
            Value::MultiPoint(x) => x.serialize(serializer),
            Value::LineString(x) => x.serialize(serializer),
            Value::MultiLineString(x) => x.serialize(serializer),
            Value::Polygon(x) => x.serialize(serializer),
            Value::MultiPolygon(x) => x.serialize(serializer),
            Value::GeometryCollection(x) => x.serialize(serializer),
        }
    }
}

/// Geometry Objects
///
/// [GeoJSON Format Specification § 3.1](https://tools.ietf.org/html/rfc7946#section-3.1)
///
/// ## Examples
///
/// Constructing a `Geometry`:
///
/// ```
/// use geojson::{Geometry, Value};
///
/// let geometry = Geometry::new(Value::Point(vec![7.428959, 1.513394]));
/// ```
///
/// Geometries can be created from `Value`s.
/// ```
/// # use geojson::{Geometry, Value};
/// let geometry1: Geometry = Value::Point(vec![7.428959, 1.513394]).into();
/// ```
///
/// Serializing a `Geometry` to a GeoJSON string:
///
/// ```
/// use geojson::{GeoJson, Geometry, Value};
/// use serde_json;
///
/// let geometry = Geometry::new(Value::Point(vec![7.428959, 1.513394]));
///
/// let geojson_string = geometry.to_string();
///
/// assert_eq!(
///     "{\"type\":\"Point\",\"coordinates\":[7.428959,1.513394]}",
///     geojson_string,
/// );
/// ```
///
/// Deserializing a GeoJSON string into a `Geometry`:
///
/// ```
/// use geojson::{GeoJson, Geometry, Value};
///
/// let geojson_str = "{\"coordinates\":[7.428959,1.513394],\"type\":\"Point\"}";
///
/// let geometry = match geojson_str.parse::<GeoJson>() {
///     Ok(GeoJson::Geometry(g)) => g,
///     _ => return,
/// };
///
/// assert_eq!(
///     Geometry::new(Value::Point(vec![7.428959, 1.513394]),),
///     geometry,
/// );
/// ```
///
/// Transforming a `Geometry` into a `geo_types::Geometry<f64>` (which requires the `geo-types`
/// feature):
///
/// ```
/// use geojson::{Geometry, Value};
/// use std::convert::TryInto;
///
/// let geometry = Geometry::new(Value::Point(vec![7.428959, 1.513394]));
/// # #[cfg(feature = "geo-types")]
/// let geom: geo_types::Geometry<f64> = geometry.try_into().unwrap();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Geometry<T = f64>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    /// Bounding Box
    ///
    /// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
    pub bbox: Option<Bbox<T>>,
    pub value: Value<T>,
    /// Foreign Members
    ///
    /// [GeoJSON Format Specification § 6](https://tools.ietf.org/html/rfc7946#section-6)
    pub foreign_members: Option<JsonObject>,
}

impl<T> Geometry<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    /// Returns a new `Geometry` with the specified `value`. `bbox` and `foreign_members` will be
    /// set to `None`.
    pub fn new(value: Value<T>) -> Self {
        Geometry {
            bbox: None,
            value,
            foreign_members: None,
        }
    }
}

impl<'a, T> From<&'a Geometry<T>> for JsonObject
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn from(geometry: &'a Geometry<T>) -> JsonObject {
        let mut map = JsonObject::from(&geometry.value);
        if let Some(ref bbox) = geometry.bbox {
            map.insert(String::from("bbox"), ::serde_json::to_value(bbox).unwrap());
        }

        if let Some(ref foreign_members) = geometry.foreign_members {
            for (key, value) in foreign_members {
                map.insert(key.to_owned(), value.to_owned());
            }
        }
        map
    }
}

impl<T> Geometry<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    pub fn from_json_object(object: JsonObject) -> Result<Self, T> {
        Self::try_from(object)
    }

    pub fn from_json_value(value: JsonValue) -> Result<Self, T> {
        Self::try_from(value)
    }

    fn serialize_to_map<SM: SerializeMap>(
        &self,
        map: &mut SM,
    ) -> std::result::Result<(), SM::Error> {
        self.value.serialize_to_map(map)?;
        if let Some(ref bbox) = self.bbox {
            map.serialize_entry("bbox", bbox)?;
        }

        if let Some(ref foreign_members) = self.foreign_members {
            for (key, value) in foreign_members {
                map.serialize_entry(key, value)?
            }
        }
        Ok(())
    }
}

impl<T> TryFrom<JsonObject> for Geometry<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    type Error = Error<T>;

    fn try_from(mut object: JsonObject) -> Result<Geometry<T>, T> {
        let bbox = util::get_bbox(&mut object)?;
        let value = util::get_value(&mut object)?;
        let foreign_members = util::get_foreign_members(object)?;
        Ok(Geometry {
            bbox,
            value,
            foreign_members,
        })
    }
}

impl<T> TryFrom<JsonValue> for Geometry<T>
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

impl<T> FromStr for Geometry<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    type Err = Error<T>;

    fn from_str(s: &str) -> Result<Self, T> {
        Self::try_from(crate::GeoJson::from_str(s)?)
    }
}

impl<T> Serialize for Geometry<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        self.serialize_to_map(&mut map)?;
        map.end()
    }
}

impl<'de, T> Deserialize<'de> for Geometry<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Geometry<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as SerdeError;

        let val = JsonObject::deserialize(deserializer)?;

        Geometry::from_json_object(val).map_err(|e| D::Error::custom(e.to_string()))
    }
}

impl<V, T> From<V> for Geometry<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
    V: Into<Value<T>>,
{
    fn from(v: V) -> Geometry<T> {
        Geometry::new(v.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, GeoJson, Geometry, JsonObject, Value};
    use serde_json::json;
    use std::str::FromStr;

    fn encode(geometry: &Geometry) -> String {
        serde_json::to_string(&geometry).unwrap()
    }
    fn decode(json_string: String) -> GeoJson {
        json_string.parse().unwrap()
    }

    #[test]
    fn encode_decode_geometry() {
        let geometry_json_str = "{\"type\":\"Point\",\"coordinates\":[1.1,2.1]}";
        let geometry = Geometry {
            value: Value::Point(vec![1.1, 2.1]),
            bbox: None,
            foreign_members: None,
        };

        // Test encode
        let json_string = encode(&geometry);
        assert_eq!(json_string, geometry_json_str);

        // Test decode
        let decoded_geometry = match decode(json_string) {
            GeoJson::Geometry(g) => g,
            _ => unreachable!(),
        };
        assert_eq!(decoded_geometry, geometry);
    }

    #[test]
    fn test_geometry_from_value() {
        use serde_json::json;
        use std::convert::TryInto;

        let json_value = json!({
            "type": "Point",
            "coordinates": [
                0.0, 0.1
            ],
        });
        assert!(json_value.is_object());

        let geometry: Geometry = json_value.try_into().unwrap();
        assert_eq!(
            geometry,
            Geometry {
                value: Value::Point(vec![0.0, 0.1]),
                bbox: None,
                foreign_members: None,
            }
        )
    }

    #[test]
    fn test_geometry_display() {
        let v = Value::LineString(vec![vec![0.0, 0.1], vec![0.1, 0.2], vec![0.2, 0.3]]);
        let geometry = Geometry::new(v);
        assert_eq!(
            geometry.to_string(),
            "{\"type\":\"LineString\",\"coordinates\":[[0.0,0.1],[0.1,0.2],[0.2,0.3]]}"
        );
    }

    #[test]
    fn test_value_display() {
        let v = Value::LineString(vec![vec![0.0, 0.1], vec![0.1, 0.2], vec![0.2, 0.3]]);
        assert_eq!(
            "{\"coordinates\":[[0.0,0.1],[0.1,0.2],[0.2,0.3]],\"type\":\"LineString\"}",
            v.to_string()
        );
    }

    #[test]
    fn encode_decode_geometry_with_foreign_member() {
        let geometry_json_str =
            "{\"type\":\"Point\",\"coordinates\":[1.1,2.1],\"other_member\":true}";
        let mut foreign_members = JsonObject::new();
        foreign_members.insert(
            String::from("other_member"),
            serde_json::to_value(true).unwrap(),
        );
        let geometry = Geometry {
            value: Value::Point(vec![1.1, 2.1]),
            bbox: None,
            foreign_members: Some(foreign_members),
        };

        // Test encode
        let json_string = encode(&geometry);
        assert_eq!(json_string, geometry_json_str);

        // Test decode
        let decoded_geometry = match decode(geometry_json_str.into()) {
            GeoJson::Geometry(g) => g,
            _ => unreachable!(),
        };
        assert_eq!(decoded_geometry, geometry);
    }

    #[test]
    fn encode_decode_geometry_collection() {
        let geometry_collection = Geometry {
            bbox: None,
            value: Value::GeometryCollection(vec![
                Geometry {
                    bbox: None,
                    value: Value::Point(vec![100.0, 0.0]),
                    foreign_members: None,
                },
                Geometry {
                    bbox: None,
                    value: Value::LineString(vec![vec![101.0, 0.0], vec![102.0, 1.0]]),
                    foreign_members: None,
                },
            ]),
            foreign_members: None,
        };

        let geometry_collection_string = "{\"type\":\"GeometryCollection\",\"geometries\":[{\"type\":\"Point\",\"coordinates\":[100.0,0.0]},{\"type\":\"LineString\",\"coordinates\":[[101.0,0.0],[102.0,1.0]]}]}";
        // Test encode
        let json_string = encode(&geometry_collection);
        assert_eq!(json_string, geometry_collection_string);

        // Test decode
        let decoded_geometry = match decode(geometry_collection_string.into()) {
            GeoJson::Geometry(g) => g,
            _ => unreachable!(),
        };
        assert_eq!(decoded_geometry, geometry_collection);
    }

    #[test]
    fn test_from_str_ok() {
        let geometry_json = json!({
            "type": "Point",
            "coordinates": [125.6f64, 10.1]
        })
        .to_string();

        let geometry = Geometry::<f64>::from_str(&geometry_json).unwrap();
        assert!(matches!(geometry.value, Value::Point(_)));
    }

    #[test]
    fn test_from_str_with_unexpected_type() {
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

        let actual_failure = Geometry::<f64>::from_str(&feature_json).unwrap_err();
        match actual_failure {
            Error::ExpectedType { actual, expected } => {
                assert_eq!(actual, "Feature");
                assert_eq!(expected, "Geometry");
            }
            e => panic!("unexpected error: {}", e),
        };
    }

    #[test]
    fn test_reject_too_few_coordinates() {
        let err = Geometry::<f64>::from_str(r#"{"type": "Point", "coordinates": []}"#).unwrap_err();
        assert_eq!(
            err.to_string(),
            "A position must contain two or more elements, but got `0`"
        );

        let err =
            Geometry::<f64>::from_str(r#"{"type": "Point", "coordinates": [23.42]}"#).unwrap_err();
        assert_eq!(
            err.to_string(),
            "A position must contain two or more elements, but got `1`"
        );
    }
}
