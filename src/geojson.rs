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
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;

/// GeoJSON Objects
///
/// ```
/// use std::convert::TryInto;
/// use geojson::{Feature, GeoJson, Geometry, GeometryValue};
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged, try_from = "deserialize::RawGeoJson")]
pub enum GeoJson {
    Geometry(Geometry),
    Feature(Feature),
    FeatureCollection(FeatureCollection),
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
        use crate::GeometryValue;
        let collection = GeometryValue::new_geometry_collection(iter.into_iter().map(Into::into));
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

impl From<Vec<Feature>> for GeoJson {
    fn from(features: Vec<Feature>) -> GeoJson {
        GeoJson::from(features.into_iter().collect::<FeatureCollection>())
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
    /// use geojson::{Feature, GeoJson, Geometry, Position, GeometryValue};
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
    ///         geometry: Some(GeometryValue::new_point([102.0, 0.5]).into()),
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

    /// Convenience wrapper for [serde_json::to_string_pretty()]
    pub fn to_string_pretty(self) -> Result<String> {
        ::serde_json::to_string_pretty(&self)
            .map_err(Error::MalformedJson)
            .map(|s| s.to_string())
    }
}

// REVIEW NOTE: Previously, we deserialized a `JsonObject`, and then converted that to `GeoJson`.
// Now that we can deserialize directly to `GeoJson` a lot of conversions to/from JsonObject/JsonValue
// feel vestigial. Should we remove them? Maybe somebody has a use for them?
// Unfortunately, you cannot deprecate trait impls.
impl TryFrom<JsonObject> for GeoJson {
    type Error = Error;

    fn try_from(object: JsonObject) -> Result<Self> {
        Self::try_from(JsonValue::Object(object))
    }
}

impl TryFrom<JsonValue> for GeoJson {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self> {
        serde_json::from_value(value).map_err(Error::MalformedJson)
    }
}

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
        Ok(serde_json::from_str(s)?)
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

mod deserialize {
    use crate::geometry::deserialize::{Coordinates, GeometryType, RawGeometry};
    use crate::util::normalize_foreign_members;
    use crate::{feature, Bbox, Error, Feature, FeatureCollection, GeoJson, Geometry, JsonObject};
    use serde::Deserialize;
    use std::convert::TryFrom;

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    enum GeoJsonType {
        Feature,
        FeatureCollection,
        Point,
        LineString,
        Polygon,
        MultiPoint,
        MultiLineString,
        MultiPolygon,
        GeometryCollection,
    }

    impl GeoJsonType {
        /// Convert to GeometryType if this is a geometry variant
        fn as_geometry_type(&self) -> Option<GeometryType> {
            match self {
                GeoJsonType::Point => Some(GeometryType::Point),
                GeoJsonType::LineString => Some(GeometryType::LineString),
                GeoJsonType::Polygon => Some(GeometryType::Polygon),
                GeoJsonType::MultiPoint => Some(GeometryType::MultiPoint),
                GeoJsonType::MultiLineString => Some(GeometryType::MultiLineString),
                GeoJsonType::MultiPolygon => Some(GeometryType::MultiPolygon),
                GeoJsonType::GeometryCollection => Some(GeometryType::GeometryCollection),
                GeoJsonType::Feature | GeoJsonType::FeatureCollection => None,
            }
        }
    }

    /// Internal struct for deserializing any GeoJSON object before converting to GeoJson.
    /// This captures all possible fields that can appear in any GeoJSON object type.
    #[derive(Debug, Clone, Deserialize)]
    #[serde(expecting = "GeoJson object")] // TODO test this "expecting"
    pub(crate) struct RawGeoJson {
        r#type: GeoJsonType,

        // Common field
        bbox: Option<Bbox>,

        // Geometry field (except GeometryCollection)
        coordinates: Option<Coordinates>,

        // GeometryCollection field
        geometries: Option<Vec<Geometry>>,

        // FeatureCollection field
        features: Option<Vec<Feature>>,

        // Feature fields
        id: Option<feature::Id>,
        geometry: Option<Geometry>,
        properties: Option<JsonObject>,

        // Foreign members (captures all other fields)
        #[serde(flatten)]
        foreign_members: Option<JsonObject>,
    }

    impl TryFrom<RawGeoJson> for GeoJson {
        type Error = Error;

        fn try_from(mut raw: RawGeoJson) -> crate::Result<Self> {
            normalize_foreign_members(&mut raw.foreign_members);

            match raw.r#type {
                GeoJsonType::FeatureCollection => {
                    let features = raw.features.ok_or_else(|| {
                        use serde::de::Error as _;
                        Error::MalformedJson(serde_json::Error::missing_field("features"))
                    })?;
                    Ok(GeoJson::FeatureCollection(FeatureCollection {
                        bbox: raw.bbox,
                        features,
                        foreign_members: raw.foreign_members,
                    }))
                }

                GeoJsonType::Feature => Ok(GeoJson::Feature(Feature {
                    bbox: raw.bbox,
                    geometry: raw.geometry,
                    id: raw.id,
                    properties: raw.properties,
                    foreign_members: raw.foreign_members,
                })),

                // Delegate all geometry types to RawGeometry
                geojson_type => {
                    let geometry_type = geojson_type.as_geometry_type().expect(
                        "as_geometry_type returns Some for all variants except Feature/FeatureCollection",
                    );
                    let raw_geom = RawGeometry {
                        r#type: geometry_type,
                        coordinates: raw.coordinates,
                        geometries: raw.geometries,
                        bbox: raw.bbox,
                        foreign_members: raw.foreign_members,
                    };
                    Ok(GeoJson::Geometry(Geometry::try_from(raw_geom)?))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Feature, FeatureCollection, GeoJson, GeometryValue};
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
                geometry: Some(GeometryValue::new_point([102.0, 0.5]).into()),
                id: None,
                properties: None,
                foreign_members: None,
            })
        );
    }

    #[test]
    fn test_geojson_from_features() {
        let features: Vec<Feature> = vec![
            GeometryValue::new_point([0., 0., 0.]).into(),
            GeometryValue::new_point([1., 1., 1.]).into(),
        ];

        let geojson: GeoJson = features.into();
        assert_eq!(
            geojson,
            GeoJson::FeatureCollection(FeatureCollection {
                features: vec![
                    Feature {
                        bbox: None,
                        geometry: Some(GeometryValue::new_point([0., 0., 0.]).into()),
                        id: None,
                        properties: None,
                        foreign_members: None,
                    },
                    Feature {
                        bbox: None,
                        geometry: Some(GeometryValue::new_point([1., 1., 1.]).into()),
                        id: None,
                        properties: None,
                        foreign_members: None,
                    },
                ],
                bbox: None,
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
                geometry: Some(GeometryValue::new_point([102.0, 0.5]).into()),
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
}
