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

use serde::ser::SerializeMap;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::str::FromStr;

use crate::errors::{Error, Result};
use crate::{util, Bbox, Feature};
use crate::{JsonObject, JsonValue};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Feature Collection Objects
///
/// [GeoJSON Format Specification § 3.3](https://tools.ietf.org/html/rfc7946#section-3.3)
///
/// # Examples
///
/// Serialization:
///
/// ```
/// use geojson::FeatureCollection;
/// use geojson::GeoJson;
///
/// let feature_collection: FeatureCollection<f64> = FeatureCollection {
///     bbox: None,
///     features: vec![],
///     foreign_members: None,
/// };
///
/// let serialized = GeoJson::from(feature_collection).to_string();
///
/// assert_eq!(
///     serialized,
///     "{\"type\":\"FeatureCollection\",\"features\":[]}"
/// );
/// ```
///
/// Collect from an iterator:
///
/// ```rust
/// use geojson::{Feature, FeatureCollection, Value};
///
/// let fc: FeatureCollection<f64> = (0..10)
///     .map(|idx| -> Feature {
///         let c = idx as f64;
///         Value::Point(vec![1.0 * c, 2.0 * c, 3.0 * c]).into()
///     })
///     .collect();
/// assert_eq!(fc.features.len(), 10);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct FeatureCollection<T = f64>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    /// Bounding Box
    ///
    /// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
    pub bbox: Option<Bbox<T>>,
    pub features: Vec<Feature<T>>,
    /// Foreign Members
    ///
    /// [GeoJSON Format Specification § 6](https://tools.ietf.org/html/rfc7946#section-6)
    pub foreign_members: Option<JsonObject>,
}

impl<T> IntoIterator for FeatureCollection<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    type Item = Feature<T>;
    type IntoIter = std::vec::IntoIter<Feature<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.features.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a FeatureCollection<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    type Item = &'a Feature<T>;
    type IntoIter = std::slice::Iter<'a, Feature<T>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self.features)
    }
}

impl<'a, T> From<&'a FeatureCollection<T>> for JsonObject
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn from(fc: &'a FeatureCollection<T>) -> JsonObject {
        // The unwrap() should never panic, because FeatureCollection contains only JSON-serializable types
        match serde_json::to_value(fc).unwrap() {
            serde_json::Value::Object(obj) => obj,
            value => {
                // Panic should never happen, because `impl Serialize for FeatureCollection` always produces an
                // Object
                panic!("serializing FeatureCollection should result in an Object, but got something {:?}", value)
            }
        }
    }
}

impl<T> FeatureCollection<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    pub fn from_json_object(object: JsonObject) -> Result<Self, T> {
        Self::try_from(object)
    }

    pub fn from_json_value(value: JsonValue) -> Result<Self, T> {
        Self::try_from(value)
    }
}

impl<T> TryFrom<JsonObject> for FeatureCollection<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    type Error = Error<T>;

    fn try_from(mut object: JsonObject) -> Result<Self, T> {
        match util::expect_type(&mut object)? {
            ref type_ if type_ == "FeatureCollection" => Ok(Self {
                bbox: util::get_bbox(&mut object)?,
                features: util::get_features(&mut object)?,
                foreign_members: util::get_foreign_members(object)?,
            }),
            type_ => Err(Error::ExpectedType {
                expected: "FeatureCollection".to_owned(),
                actual: type_,
            }),
        }
    }
}

impl<T> TryFrom<JsonValue> for FeatureCollection<T>
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

impl<T> FromStr for FeatureCollection<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    type Err = Error<T>;

    fn from_str(s: &str) -> Result<Self, T> {
        Self::try_from(crate::GeoJson::from_str(s)?)
    }
}

impl<T> Serialize for FeatureCollection<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", "FeatureCollection")?;
        map.serialize_entry("features", &self.features)?;

        if let Some(ref bbox) = self.bbox {
            map.serialize_entry("bbox", bbox)?;
        }

        if let Some(ref foreign_members) = self.foreign_members {
            for (key, value) in foreign_members {
                map.serialize_entry(key, value)?;
            }
        }

        map.end()
    }
}

impl<'de, T> Deserialize<'de> for FeatureCollection<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<FeatureCollection<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as SerdeError;

        let val = JsonObject::deserialize(deserializer)?;

        FeatureCollection::from_json_object(val).map_err(|e| D::Error::custom(e.to_string()))
    }
}

/// Create a [`FeatureCollection`] using the [`collect`]
/// method on an iterator of `Feature`s. If every item
/// contains a bounding-box of the same dimension, then the
/// output has a bounding-box of the union of them.
/// Otherwise, the output will not have a bounding-box.
///
/// [`collect`]: std::iter::Iterator::collect
impl<T> FromIterator<Feature<T>> for FeatureCollection<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn from_iter<U: IntoIterator<Item = Feature<T>>>(iter: U) -> Self {
        let mut bbox = Some(vec![]);

        let features = iter
            .into_iter()
            .inspect(|feat| {
                // Try to compute the bounding-box

                let (curr_bbox, curr_len) = match &mut bbox {
                    Some(curr_bbox) => {
                        let curr_len = curr_bbox.len();
                        (curr_bbox, curr_len)
                    }
                    None => {
                        // implies we can't compute a
                        // bounding-box for this collection
                        return;
                    }
                };

                match &feat.bbox {
                    None => {
                        bbox = None;
                    }
                    Some(fbox) if fbox.is_empty() || fbox.len() % 2 != 0 => {
                        bbox = None;
                    }
                    Some(fbox) if curr_len == 0 => {
                        // First iteration: just copy values from fbox
                        *curr_bbox = fbox.clone();
                    }
                    Some(fbox) if curr_len != fbox.len() => {
                        bbox = None;
                    }
                    Some(fbox) => {
                        // Update bbox by computing min/max
                        curr_bbox.iter_mut().zip(fbox.iter()).enumerate().for_each(
                            |(idx, (bc, fc))| {
                                if idx < curr_len / 2 {
                                    // These are the min coords
                                    *bc = fc.min(*bc);
                                } else {
                                    *bc = fc.max(*bc);
                                }
                            },
                        );
                    }
                };
            })
            .collect();
        Self {
            bbox,
            features,
            foreign_members: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Feature, FeatureCollection, Value};
    use serde_json::json;

    use std::str::FromStr;

    #[test]
    fn test_fc_from_iterator() {
        let features: Vec<Feature> = vec![
            {
                let mut feat: Feature = Value::Point(vec![0., 0., 0.]).into();
                feat.bbox = Some(vec![-1., -1., -1., 1., 1., 1.]);
                feat
            },
            {
                let mut feat: Feature =
                    Value::MultiPoint(vec![vec![10., 10., 10.], vec![11., 11., 11.]]).into();
                feat.bbox = Some(vec![10., 10., 10., 11., 11., 11.]);
                feat
            },
        ];

        let fc: FeatureCollection = features.into_iter().collect();
        assert_eq!(fc.features.len(), 2);
        assert_eq!(fc.bbox, Some(vec![-1., -1., -1., 11., 11., 11.]));
    }

    fn feature_collection_json() -> String {
        json!({ "type": "FeatureCollection", "features": [
        {
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [11.1, 22.2]
            },
            "properties": {
                "name": "Downtown"
            }
        },
        {
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [33.3, 44.4]
            },
            "properties": {
                "name": "Uptown"
            }
        },
        ]})
        .to_string()
    }

    #[test]
    fn test_from_str_ok() {
        let feature_collection =
            FeatureCollection::<f64>::from_str(&feature_collection_json()).unwrap();
        assert_eq!(2, feature_collection.features.len());
    }

    #[test]
    fn iter_features() {
        let feature_collection =
            FeatureCollection::<f64>::from_str(&feature_collection_json()).unwrap();

        let mut names: Vec<String> = vec![];
        for feature in &feature_collection {
            let name = feature
                .property("name")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            names.push(name);
        }

        assert_eq!(names, vec!["Downtown", "Uptown"]);
    }

    #[test]
    fn test_from_str_with_unexpected_type() {
        let geometry_json = json!({
            "type": "Point",
            "coordinates": [125.6, 10.1]
        })
        .to_string();

        let actual_failure = FeatureCollection::<f64>::from_str(&geometry_json).unwrap_err();
        match actual_failure {
            Error::ExpectedType { actual, expected } => {
                assert_eq!(actual, "Geometry");
                assert_eq!(expected, "FeatureCollection");
            }
            e => panic!("unexpected error: {}", e),
        };
    }
}
