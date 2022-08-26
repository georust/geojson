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
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
/// use tinyvec::tiny_vec;
/// let point = geo_types::Point::new(2., 9.);
/// let genum = geo_types::Geometry::from(point);
/// assert_eq!(
///     geojson::Value::from(&point),
///     geojson::Value::Point(tiny_vec![2., 9.]),
/// );
/// assert_eq!(
///     geojson::Value::from(&genum),
///     geojson::Value::Point(tiny_vec![2., 9.]),
/// );
/// # }
/// # #[cfg(not(feature = "geo-types"))]
/// # fn test() {}
/// # test()
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Point
    ///
    /// [GeoJSON Format Specification § 3.1.2](https://tools.ietf.org/html/rfc7946#section-3.1.2)
    Point(PointType),

    /// MultiPoint
    ///
    /// [GeoJSON Format Specification § 3.1.3](https://tools.ietf.org/html/rfc7946#section-3.1.3)
    MultiPoint(Vec<PointType>),

    /// LineString
    ///
    /// [GeoJSON Format Specification § 3.1.4](https://tools.ietf.org/html/rfc7946#section-3.1.4)
    LineString(LineStringType),

    /// MultiLineString
    ///
    /// [GeoJSON Format Specification § 3.1.5](https://tools.ietf.org/html/rfc7946#section-3.1.5)
    MultiLineString(Vec<LineStringType>),

    /// Polygon
    ///
    /// [GeoJSON Format Specification § 3.1.6](https://tools.ietf.org/html/rfc7946#section-3.1.6)
    Polygon(PolygonType),

    /// MultiPolygon
    ///
    /// [GeoJSON Format Specification § 3.1.7](https://tools.ietf.org/html/rfc7946#section-3.1.7)
    MultiPolygon(Vec<PolygonType>),

    /// GeometryCollection
    ///
    /// [GeoJSON Format Specification § 3.1.8](https://tools.ietf.org/html/rfc7946#section-3.1.8)
    GeometryCollection(Vec<Geometry>),
}

impl Value {
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

impl<'a> From<&'a Value> for JsonObject {
    fn from(value: &'a Value) -> JsonObject {
        let mut map = JsonObject::new();
        map.insert(
            String::from("type"),
            ::serde_json::to_value(value.type_name()).unwrap(),
        );
        map.insert(
            String::from(match value {
                Value::GeometryCollection(..) => "geometries",
                _ => "coordinates",
            }),
            ::serde_json::to_value(value).unwrap(),
        );
        map
    }
}

impl Value {
    pub fn from_json_object(object: JsonObject) -> Result<Self> {
        Self::try_from(object)
    }

    pub fn from_json_value(value: JsonValue) -> Result<Self> {
        Self::try_from(value)
    }
}

impl TryFrom<JsonObject> for Value {
    type Error = Error;

    fn try_from(mut object: JsonObject) -> Result<Self> {
        util::get_value(&mut object)
    }
}

impl TryFrom<JsonValue> for Value {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self> {
        if let JsonValue::Object(obj) = value {
            Self::try_from(obj)
        } else {
            Err(Error::GeoJsonExpectedObject(value))
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ::serde_json::to_string(&JsonObject::from(self))
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

impl<'a> From<&'a Value> for JsonValue {
    fn from(value: &'a Value) -> JsonValue {
        match *value {
            Value::Point(ref x) => ::serde_json::to_value(x),
            Value::MultiPoint(ref x) => ::serde_json::to_value(x),
            Value::LineString(ref x) => ::serde_json::to_value(x),
            Value::MultiLineString(ref x) => ::serde_json::to_value(x),
            Value::Polygon(ref x) => ::serde_json::to_value(x),
            Value::MultiPolygon(ref x) => ::serde_json::to_value(x),
            Value::GeometryCollection(ref x) => ::serde_json::to_value(x),
        }
        .unwrap()
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        JsonValue::from(self).serialize(serializer)
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
/// use tinyvec::tiny_vec;
/// use geojson::{Geometry, Value};
///
/// let geometry = Geometry::new(Value::Point(tiny_vec![7.428959, 1.513394]));
/// ```
///
/// Geometries can be created from `Value`s.
/// ```
/// # use tinyvec::tiny_vec;
/// # use geojson::{Geometry, Value};
/// let geometry1: Geometry = Value::Point(tiny_vec![7.428959, 1.513394]).into();
/// ```
///
/// Serializing a `Geometry` to a GeoJSON string:
///
/// ```
/// use tinyvec::tiny_vec;
/// use geojson::{GeoJson, Geometry, Value};
/// use serde_json;
///
/// let geometry = Geometry::new(Value::Point(tiny_vec![7.428959, 1.513394]));
///
/// let geojson_string = geometry.to_string();
///
/// assert_eq!(
///     "{\"coordinates\":[7.428959,1.513394],\"type\":\"Point\"}",
///     geojson_string,
/// );
/// ```
///
/// Deserializing a GeoJSON string into a `Geometry`:
///
/// ```
/// use geojson::{GeoJson, Geometry, Value};
/// use tinyvec::tiny_vec;
///
/// let geojson_str = "{\"coordinates\":[7.428959,1.513394],\"type\":\"Point\"}";
///
/// let geometry = match geojson_str.parse::<GeoJson>() {
///     Ok(GeoJson::Geometry(g)) => g,
///     _ => return,
/// };
///
/// assert_eq!(
///     Geometry::new(Value::Point(tiny_vec![7.428959, 1.513394]),),
///     geometry,
/// );
/// ```
///
/// Transforming a `Geometry` into a `geo_types::Geometry<f64>` (which requires the `geo-types`
/// feature):
///
/// ```
/// use geojson::{Geometry, Value};
/// use tinyvec::tiny_vec;
/// use std::convert::TryInto;
///
/// let geometry = Geometry::new(Value::Point(tiny_vec![7.428959, 1.513394]));
/// # #[cfg(feature = "geo-types")]
/// let geom: geo_types::Geometry<f64> = geometry.try_into().unwrap();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Geometry {
    /// Bounding Box
    ///
    /// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
    pub bbox: Option<Bbox>,
    pub value: Value,
    /// Foreign Members
    ///
    /// [GeoJSON Format Specification § 6](https://tools.ietf.org/html/rfc7946#section-6)
    pub foreign_members: Option<JsonObject>,
}

impl Geometry {
    /// Returns a new `Geometry` with the specified `value`. `bbox` and `foreign_members` will be
    /// set to `None`.
    pub fn new(value: Value) -> Self {
        Geometry {
            bbox: None,
            value,
            foreign_members: None,
        }
    }
}

impl<'a> From<&'a Geometry> for JsonObject {
    fn from(geometry: &'a Geometry) -> JsonObject {
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

impl Geometry {
    pub fn from_json_object(object: JsonObject) -> Result<Self> {
        Self::try_from(object)
    }

    pub fn from_json_value(value: JsonValue) -> Result<Self> {
        Self::try_from(value)
    }
}

impl TryFrom<JsonObject> for Geometry {
    type Error = Error;

    fn try_from(mut object: JsonObject) -> Result<Self> {
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

impl TryFrom<JsonValue> for Geometry {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self> {
        if let JsonValue::Object(obj) = value {
            Self::try_from(obj)
        } else {
            Err(Error::GeoJsonExpectedObject(value))
        }
    }
}

impl FromStr for Geometry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::try_from(crate::GeoJson::from_str(s)?)
    }
}

impl Serialize for Geometry {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        JsonObject::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Geometry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Geometry, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as SerdeError;

        let val = JsonObject::deserialize(deserializer)?;

        Geometry::from_json_object(val).map_err(|e| D::Error::custom(e.to_string()))
    }
}

impl<V> From<V> for Geometry
where
    V: Into<Value>,
{
    fn from(v: V) -> Geometry {
        Geometry::new(v.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, GeoJson, Geometry, JsonObject, Value};
    use serde_json::json;
    use std::str::FromStr;
    use tinyvec::tiny_vec;

    fn encode(geometry: &Geometry) -> String {
        serde_json::to_string(&geometry).unwrap()
    }
    fn decode(json_string: String) -> GeoJson {
        json_string.parse().unwrap()
    }

    #[test]
    fn encode_decode_geometry() {
        let geometry_json_str = "{\"coordinates\":[1.1,2.1],\"type\":\"Point\"}";
        let geometry = Geometry {
            value: Value::Point(tiny_vec![1.1, 2.1]),
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
                value: Value::Point(tiny_vec![0.0, 0.1]),
                bbox: None,
                foreign_members: None,
            }
        )
    }

    #[test]
    fn test_geometry_display() {
        let v = Value::LineString(vec![
            tiny_vec![0.0, 0.1],
            tiny_vec![0.1, 0.2],
            tiny_vec![0.2, 0.3],
        ]);
        let geometry = Geometry::new(v);
        assert_eq!(
            "{\"coordinates\":[[0.0,0.1],[0.1,0.2],[0.2,0.3]],\"type\":\"LineString\"}",
            geometry.to_string()
        );
    }

    #[test]
    fn test_value_display() {
        let v = Value::LineString(vec![
            tiny_vec![0.0, 0.1],
            tiny_vec![0.1, 0.2],
            tiny_vec![0.2, 0.3],
        ]);
        assert_eq!(
            "{\"coordinates\":[[0.0,0.1],[0.1,0.2],[0.2,0.3]],\"type\":\"LineString\"}",
            v.to_string()
        );
    }

    #[test]
    fn encode_decode_geometry_with_foreign_member() {
        let geometry_json_str =
            "{\"coordinates\":[1.1,2.1],\"other_member\":true,\"type\":\"Point\"}";
        let mut foreign_members = JsonObject::new();
        foreign_members.insert(
            String::from("other_member"),
            serde_json::to_value(true).unwrap(),
        );
        let geometry = Geometry {
            value: Value::Point(tiny_vec![1.1, 2.1]),
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
                    value: Value::Point(tiny_vec![100.0, 0.0]),
                    foreign_members: None,
                },
                Geometry {
                    bbox: None,
                    value: Value::LineString(vec![tiny_vec![101.0, 0.0], tiny_vec![102.0, 1.0]]),
                    foreign_members: None,
                },
            ]),
            foreign_members: None,
        };

        let geometry_collection_string = "{\"geometries\":[{\"coordinates\":[100.0,0.0],\"type\":\"Point\"},{\"coordinates\":[[101.0,0.0],[102.0,1.0]],\"type\":\"LineString\"}],\"type\":\"GeometryCollection\"}";
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

        let geometry = Geometry::from_str(&geometry_json).unwrap();
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

        let actual_failure = Geometry::from_str(&feature_json).unwrap_err();
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
        let err = Geometry::from_str(r#"{"type": "Point", "coordinates": []}"#).unwrap_err();
        assert_eq!(
            err.to_string(),
            "A position must contain two or more elements, but got `0`"
        );

        let err = Geometry::from_str(r#"{"type": "Point", "coordinates": [23.42]}"#).unwrap_err();
        assert_eq!(
            err.to_string(),
            "A position must contain two or more elements, but got `1`"
        );
    }
}
