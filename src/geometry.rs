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
use crate::{util, Bbox, LineStringType, PointType, PolygonType, Position};
use crate::{JsonObject, JsonValue};
use serde::{Deserialize, Serialize};

#[deprecated(note = "Renamed to GeometryValue")]
pub type Value = GeometryValue;
/// The underlying value for a `Geometry`.
///
/// # Conversion from `geo_types`
///
/// A `GeometryValue` can be created by using the `From` impl which is available for both `geo_types`
/// primitives AND `geo_types::Geometry` enum members:
///
/// ```rust
/// # #[cfg(feature = "geo-types")]
/// # fn test() {
/// let point = geo_types::Point::new(2., 9.);
/// let genum = geo_types::Geometry::from(point);
/// assert_eq!(
///     geojson::GeometryValue::from(&point),
///     geojson::GeometryValue::new_point([2., 9.]),
/// );
/// assert_eq!(
///     geojson::GeometryValue::from(&genum),
///     geojson::GeometryValue::new_point([2., 9.]),
/// );
/// # }
/// # #[cfg(not(feature = "geo-types"))]
/// # fn test() {}
/// # test()
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GeometryValue {
    /// Point
    ///
    /// [GeoJSON Format Specification § 3.1.2](https://tools.ietf.org/html/rfc7946#section-3.1.2)
    Point { coordinates: PointType },

    /// MultiPoint
    ///
    /// [GeoJSON Format Specification § 3.1.3](https://tools.ietf.org/html/rfc7946#section-3.1.3)
    MultiPoint { coordinates: Vec<PointType> },

    /// LineString
    ///
    /// [GeoJSON Format Specification § 3.1.4](https://tools.ietf.org/html/rfc7946#section-3.1.4)
    LineString { coordinates: LineStringType },

    /// MultiLineString
    ///
    /// [GeoJSON Format Specification § 3.1.5](https://tools.ietf.org/html/rfc7946#section-3.1.5)
    MultiLineString { coordinates: Vec<LineStringType> },

    /// Polygon
    ///
    /// [GeoJSON Format Specification § 3.1.6](https://tools.ietf.org/html/rfc7946#section-3.1.6)
    Polygon { coordinates: PolygonType },

    /// MultiPolygon
    ///
    /// [GeoJSON Format Specification § 3.1.7](https://tools.ietf.org/html/rfc7946#section-3.1.7)
    MultiPolygon { coordinates: Vec<PolygonType> },

    /// GeometryCollection
    ///
    /// [GeoJSON Format Specification § 3.1.8](https://tools.ietf.org/html/rfc7946#section-3.1.8)
    GeometryCollection { geometries: Vec<Geometry> },
}

impl GeometryValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            GeometryValue::Point { .. } => "Point",
            GeometryValue::MultiPoint { .. } => "MultiPoint",
            GeometryValue::LineString { .. } => "LineString",
            GeometryValue::MultiLineString { .. } => "MultiLineString",
            GeometryValue::Polygon { .. } => "Polygon",
            GeometryValue::MultiPolygon { .. } => "MultiPolygon",
            GeometryValue::GeometryCollection { .. } => "GeometryCollection",
        }
    }
    pub fn new_point(value: impl Into<Position>) -> GeometryValue {
        GeometryValue::Point {
            coordinates: value.into(),
        }
    }
    pub fn new_line_string(value: impl IntoIterator<Item = impl Into<Position>>) -> GeometryValue {
        let coordinates: Vec<Position> = value.into_iter().map(Into::into).collect();
        GeometryValue::LineString { coordinates }
    }
    pub fn new_multi_point(value: impl IntoIterator<Item = impl Into<Position>>) -> GeometryValue {
        let coordinates: Vec<Position> = value.into_iter().map(Into::into).collect();
        GeometryValue::MultiPoint { coordinates }
    }
    pub fn new_multi_line_string(
        value: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<Position>>>,
    ) -> GeometryValue {
        let coordinates: Vec<Vec<Position>> = value
            .into_iter()
            .map(|line_string| line_string.into_iter().map(Into::into).collect())
            .collect();
        GeometryValue::MultiLineString { coordinates }
    }
    pub fn new_polygon(
        value: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<Position>>>,
    ) -> GeometryValue {
        let coordinates: Vec<Vec<Position>> = value
            .into_iter()
            .map(|ring| ring.into_iter().map(Into::into).collect())
            .collect();
        GeometryValue::Polygon { coordinates }
    }
    pub fn new_multi_polygon(
        value: impl IntoIterator<
            Item = impl IntoIterator<Item = impl IntoIterator<Item = impl Into<Position>>>,
        >,
    ) -> GeometryValue {
        let coordinates: Vec<Vec<Vec<Position>>> = value
            .into_iter()
            .map(|polygon| {
                polygon
                    .into_iter()
                    .map(|ring| ring.into_iter().map(Into::into).collect())
                    .collect()
            })
            .collect();
        GeometryValue::MultiPolygon { coordinates }
    }
    pub fn new_geometry_collection(
        value: impl IntoIterator<Item = impl Into<Geometry>>,
    ) -> GeometryValue {
        let geometries: Vec<Geometry> = value.into_iter().map(Into::into).collect();
        GeometryValue::GeometryCollection { geometries }
    }
}

impl<'a> From<&'a GeometryValue> for JsonObject {
    fn from(value: &'a GeometryValue) -> JsonObject {
        let value = ::serde_json::to_value(value)
            .expect("GeometryValue contains only JSON-serializable types");
        let serde_json::Value::Object(object) = value else {
            unreachable!("GeometryValue always serializes to a JsonObject");
        };
        object
    }
}

impl GeometryValue {
    pub fn from_json_object(object: JsonObject) -> Result<Self> {
        Self::try_from(object)
    }

    pub fn from_json_value(value: JsonValue) -> Result<Self> {
        Self::try_from(value)
    }
}

impl TryFrom<JsonObject> for GeometryValue {
    type Error = Error;

    fn try_from(mut object: JsonObject) -> Result<Self> {
        util::get_value(&mut object)
    }
}

impl TryFrom<JsonValue> for GeometryValue {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self> {
        if let JsonValue::Object(obj) = value {
            Self::try_from(obj)
        } else {
            Err(Error::GeoJsonExpectedObject(value))
        }
    }
}

impl fmt::Display for GeometryValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ::serde_json::to_string(self)
            .map_err(|_| fmt::Error)
            .and_then(|s| f.write_str(&s))
    }
}

impl<'a> From<&'a GeometryValue> for JsonValue {
    fn from(value: &'a GeometryValue) -> JsonValue {
        ::serde_json::to_value(value).unwrap()
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
/// use geojson::{Geometry, GeometryValue, Position};
///
/// let geometry = Geometry::new(GeometryValue::Point {
///     coordinates: Position::from([7.428959, 1.513394]),
/// });
/// ```
///
/// Constructors make this more concise.
/// ```
/// # use geojson::{Geometry, GeometryValue};
/// let geometry = Geometry::new(GeometryValue::new_point([7.428959, 1.513394]));
/// ```
///
/// `GeometryValue` can be converted `into` a `Geometry`.
/// ```
/// # use geojson::{Geometry, Position, GeometryValue};
/// let geometry: Geometry = GeometryValue::new_point([7.428959, 1.513394]).into();
/// ```
///
/// Serializing a `Geometry` to a GeoJSON string:
///
/// ```
/// use geojson::{GeoJson, Geometry, GeometryValue, Position};
/// use serde_json;
///
/// let geometry: Geometry = GeometryValue::new_point([7.428959, 1.513394]).into();
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
/// use geojson::{GeoJson, Geometry, GeometryValue, Position};
///
/// let geojson_str = r#"{"type":"Point", "coordinates":[7.428959,1.513394]}"#;
///
/// let geometry = geojson_str
///     .parse::<Geometry>()
///     .expect("valid Geometry GeoJSON");
///
/// assert_eq!(
///     Geometry::new(GeometryValue::new_point([7.428959, 1.513394])),
///     geometry,
/// );
/// ```
///
/// Transforming a `Geometry` into a `geo_types::Geometry<f64>` (which requires the `geo-types`
/// feature):
///
/// ```
/// use geojson::{Geometry, GeometryValue, Position};
/// use std::convert::TryInto;
///
/// let geometry = Geometry::new(GeometryValue::new_point([7.428959, 1.513394]));
/// # #[cfg(feature = "geo-types")]
/// let geom: geo_types::Geometry<f64> = geometry.try_into().unwrap();
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "deserialize::RawGeometry")]
pub struct Geometry {
    /// Bounding Box
    ///
    /// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Bbox>,

    #[serde(flatten)]
    pub value: GeometryValue,

    /// Foreign Members
    ///
    /// [GeoJSON Format Specification § 6](https://tools.ietf.org/html/rfc7946#section-6)
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub foreign_members: Option<JsonObject>,
}

impl Geometry {
    /// Returns a new `Geometry` with the specified `value`. `bbox` and `foreign_members` will be
    /// set to `None`.
    pub fn new(value: GeometryValue) -> Self {
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

impl<V> From<V> for Geometry
where
    V: Into<GeometryValue>,
{
    fn from(v: V) -> Geometry {
        Geometry::new(v.into())
    }
}

pub(crate) mod deserialize {
    use super::*;
    use crate::util::normalize_foreign_members;
    use serde::de::{Deserializer, SeqAccess, Visitor};
    use std::fmt::Formatter;
    use tinyvec::TinyVec;

    /// Internal enum for geometry type discrimination during deserialization.
    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub(crate) enum GeometryType {
        Point,
        LineString,
        Polygon,
        MultiPoint,
        MultiLineString,
        MultiPolygon,
        GeometryCollection,
    }

    /// An efficiently deserializable representation for Geometry coordinates
    #[derive(Debug, Clone, PartialEq)]
    #[allow(clippy::enum_variant_names)]
    pub(crate) enum Coordinates {
        ZeroDimensional(Position),
        OneDimensional(Vec<Position>),
        TwoDimensional(Vec<Vec<Position>>),
        ThreeDimensional(Vec<Vec<Vec<Position>>>),
    }

    impl<'de> Deserialize<'de> for Coordinates {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            /// While parsing the coordinates field, the next element will be either an individual Float
            /// or a (potentially nested) sequence of floats.
            enum CoordsElement {
                Float(f64),
                Coords(Coordinates),
            }

            impl<'de> Deserialize<'de> for CoordsElement {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    struct CoordsElementVisitor;
                    impl<'de> Visitor<'de> for CoordsElementVisitor {
                        type Value = CoordsElement;

                        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                            formatter.write_str("a coordinate element (number or array)")
                        }

                        fn visit_i64<E>(self, value: i64) -> std::result::Result<CoordsElement, E> {
                            Ok(CoordsElement::Float(value as f64))
                        }

                        fn visit_u64<E>(self, value: u64) -> std::result::Result<CoordsElement, E> {
                            Ok(CoordsElement::Float(value as f64))
                        }

                        fn visit_f64<E>(self, value: f64) -> std::result::Result<CoordsElement, E> {
                            Ok(CoordsElement::Float(value))
                        }

                        fn visit_seq<A>(
                            self,
                            mut seq: A,
                        ) -> std::result::Result<Self::Value, A::Error>
                        where
                            A: SeqAccess<'de>,
                        {
                            let coords = match seq.next_element::<CoordsElement>()? {
                                // Empty array [] - treat as OneDimensional([])
                                None => Coordinates::OneDimensional(vec![]),
                                // First element is a float -> this is a position [x, y, ...]
                                Some(CoordsElement::Float(first)) => {
                                    let mut floats = TinyVec::<[f64; 2]>::new();
                                    floats.push(first);
                                    while let Some(next) = seq.next_element::<f64>()? {
                                        floats.push(next);
                                    }
                                    Coordinates::ZeroDimensional(Position::from(floats))
                                }
                                // First element is a sequence, collect the rest of the elements.
                                Some(CoordsElement::Coords(coords)) => match coords {
                                    Coordinates::ZeroDimensional(first) => {
                                        let mut positions_1d = vec![first];
                                        while let Some(next) = seq.next_element::<Position>()? {
                                            positions_1d.push(next);
                                        }
                                        Coordinates::OneDimensional(positions_1d)
                                    }
                                    Coordinates::OneDimensional(positions_1d) => {
                                        let mut positions_2d = vec![positions_1d];
                                        while let Some(next) =
                                            seq.next_element::<Vec<Position>>()?
                                        {
                                            positions_2d.push(next);
                                        }
                                        Coordinates::TwoDimensional(positions_2d)
                                    }
                                    Coordinates::TwoDimensional(positions_2d) => {
                                        let mut positions_3d = vec![positions_2d];
                                        while let Some(next) =
                                            seq.next_element::<Vec<Vec<Position>>>()?
                                        {
                                            positions_3d.push(next);
                                        }
                                        Coordinates::ThreeDimensional(positions_3d)
                                    }
                                    Coordinates::ThreeDimensional(_) => {
                                        return Err(serde::de::Error::custom(
                                            "coordinate nesting too deep",
                                        ))
                                    }
                                },
                            };
                            Ok(CoordsElement::Coords(coords))
                        }
                    }
                    deserializer.deserialize_any(CoordsElementVisitor)
                }
            }

            match CoordsElement::deserialize(deserializer)? {
                CoordsElement::Float(_) => {
                    Err(serde::de::Error::custom("expected array, got number"))
                }
                CoordsElement::Coords(coords) => Ok(coords),
            }
        }
    }

    /// Internal struct for deserializing geometry JSON into before converting to Geometry.
    /// This captures all possible geometry fields, allowing validation during TryFrom conversion.
    #[derive(Debug, Clone, Deserialize)]
    #[serde(expecting = "Geometry object")]
    pub(crate) struct RawGeometry {
        pub(crate) r#type: GeometryType,
        #[serde(default)]
        pub(crate) coordinates: Option<Coordinates>,
        #[serde(default)]
        pub(crate) geometries: Option<Vec<Geometry>>,
        #[serde(default)]
        pub(crate) bbox: Option<Bbox>,
        /// Captures all other fields as foreign members
        #[serde(flatten)]
        pub(crate) foreign_members: Option<JsonObject>,
    }

    impl TryFrom<RawGeometry> for Geometry {
        type Error = Error;

        fn try_from(mut raw: RawGeometry) -> Result<Self> {
            normalize_foreign_members(&mut raw.foreign_members);

            let value = match (raw.r#type, raw.coordinates, raw.geometries) {
                // Point: ZeroDimensional coordinates
                (GeometryType::Point, Some(Coordinates::ZeroDimensional(coordinates)), None) => {
                    if coordinates.len() < 2 {
                        return Err(Error::PositionTooShort(coordinates.len()));
                    }
                    GeometryValue::Point { coordinates }
                }
                // Empty Point (coordinates: [] deserializes as OneDimensional([]))
                (GeometryType::Point, Some(Coordinates::OneDimensional(coordinates)), None)
                    if coordinates.is_empty() =>
                {
                    return Err(Error::PositionTooShort(0));
                }

                // LineString: OneDimensional coordinates (handles empty case too)
                (
                    GeometryType::LineString,
                    Some(Coordinates::OneDimensional(coordinates)),
                    None,
                ) => GeometryValue::LineString { coordinates },

                // Polygon: TwoDimensional coordinates
                (GeometryType::Polygon, Some(Coordinates::TwoDimensional(coordinates)), None) => {
                    GeometryValue::Polygon { coordinates }
                }
                // Empty Polygon (coordinates: [] deserializes as OneDimensional([]))
                (GeometryType::Polygon, Some(Coordinates::OneDimensional(coordinates)), None)
                    if coordinates.is_empty() =>
                {
                    GeometryValue::Polygon {
                        coordinates: vec![],
                    }
                }

                // MultiPoint: OneDimensional coordinates (handles empty case too)
                (
                    GeometryType::MultiPoint,
                    Some(Coordinates::OneDimensional(coordinates)),
                    None,
                ) => GeometryValue::MultiPoint { coordinates },

                // MultiLineString: TwoDimensional coordinates
                (
                    GeometryType::MultiLineString,
                    Some(Coordinates::TwoDimensional(coordinates)),
                    None,
                ) => GeometryValue::MultiLineString { coordinates },
                // Empty MultiLineString (coordinates: [] deserializes as OneDimensional([]))
                (
                    GeometryType::MultiLineString,
                    Some(Coordinates::OneDimensional(coordinates)),
                    None,
                ) if coordinates.is_empty() => GeometryValue::MultiLineString {
                    coordinates: vec![],
                },

                // MultiPolygon: ThreeDimensional coordinates
                (
                    GeometryType::MultiPolygon,
                    Some(Coordinates::ThreeDimensional(coordinates)),
                    None,
                ) => GeometryValue::MultiPolygon { coordinates },
                // Empty MultiPolygon (coordinates: [] deserializes as OneDimensional([]))
                (
                    GeometryType::MultiPolygon,
                    Some(Coordinates::OneDimensional(coordinates)),
                    None,
                ) if coordinates.is_empty() => GeometryValue::MultiPolygon {
                    coordinates: vec![],
                },

                // GeometryCollection: geometries array, no coordinates
                (GeometryType::GeometryCollection, None, Some(geometries)) => {
                    GeometryValue::GeometryCollection { geometries }
                }

                // Invalid combinations
                _ => {
                    return Err(Error::GeometryUnknownType(
                        "invalid geometry: mismatched type and coordinates".to_string(),
                    ))
                }
            };

            Ok(Geometry {
                bbox: raw.bbox,
                value,
                foreign_members: raw.foreign_members,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, GeoJson, Geometry, GeometryValue, JsonObject};
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
            value: GeometryValue::new_point([1.1, 2.1]),
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
                value: GeometryValue::new_point([0.0, 0.1]),
                bbox: None,
                foreign_members: None,
            }
        )
    }

    #[test]
    fn test_geometry_display() {
        let v = GeometryValue::new_line_string([[0.0, 0.1], [0.1, 0.2], [0.2, 0.3]]);
        let geometry = Geometry::new(v);
        assert_eq!(
            geometry.to_string(),
            "{\"type\":\"LineString\",\"coordinates\":[[0.0,0.1],[0.1,0.2],[0.2,0.3]]}"
        );
    }

    #[test]
    fn test_value_display() {
        let v = GeometryValue::new_line_string([[0.0, 0.1], [0.1, 0.2], [0.2, 0.3]]);
        assert_eq!(
            r#"{"type":"LineString","coordinates":[[0.0,0.1],[0.1,0.2],[0.2,0.3]]}"#,
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
            value: GeometryValue::new_point([1.1, 2.1]),
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
            value: GeometryValue::new_geometry_collection([
                GeometryValue::new_point([100.0, 0.0]),
                GeometryValue::new_line_string([[101.0, 0.0], [102.0, 1.0]]),
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

        let geometry = Geometry::from_str(&geometry_json).unwrap();
        assert!(matches!(geometry.value, GeometryValue::Point { .. }));
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
        assert!(err
            .to_string()
            .contains("A position must contain two or more elements, but got `0`"));

        let err = Geometry::from_str(r#"{"type": "Point", "coordinates": [23.42]}"#).unwrap_err();
        assert!(err
            .to_string()
            .contains("A position must contain two or more elements, but got `1`"));
    }
}
