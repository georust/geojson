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
use std::iter::FromIterator;

use crate::errors::Error;
use crate::json::{json, Deserialize, Deserializer, JsonObject, JsonValue, Serialize, Serializer};
use crate::{util, Bbox, Feature};

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
/// let feature_collection = FeatureCollection {
///     bbox: None,
///     features: vec![],
///     foreign_members: None,
/// };
///
/// let serialized = GeoJson::from(feature_collection).to_string();
///
/// assert_eq!(
///     serialized,
///     "{\"features\":[],\"type\":\"FeatureCollection\"}"
/// );
/// ```
///
/// Collect from an iterator:
///
/// ```rust
/// use geojson::{Feature, FeatureCollection, Value};
///
/// let fc: FeatureCollection = (0..10)
///     .map(|idx| -> Feature {
///         let c = idx as f64;
///         Value::Point(vec![1.0 * c, 2.0 * c, 3.0 * c]).into()
///     })
///     .collect();
/// assert_eq!(fc.features.len(), 10);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct FeatureCollection {
    /// Bounding Box
    ///
    /// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
    pub bbox: Option<Bbox>,
    pub features: Vec<Feature>,
    /// Foreign Members
    ///
    /// [GeoJSON Format Specification § 6](https://tools.ietf.org/html/rfc7946#section-6)
    pub foreign_members: Option<JsonObject>,
}

impl<'a> From<&'a FeatureCollection> for JsonObject {
    fn from(fc: &'a FeatureCollection) -> JsonObject {
        let mut map = JsonObject::new();
        map.insert(String::from("type"), json!("FeatureCollection"));
        map.insert(
            String::from("features"),
            serde_json::to_value(&fc.features).unwrap(),
        );

        if let Some(ref bbox) = fc.bbox {
            map.insert(String::from("bbox"), serde_json::to_value(bbox).unwrap());
        }

        if let Some(ref foreign_members) = fc.foreign_members {
            for (key, value) in foreign_members {
                map.insert(key.to_owned(), value.to_owned());
            }
        }

        map
    }
}

impl FeatureCollection {
    pub fn from_json_object(object: JsonObject) -> Result<Self, Error> {
        Self::try_from(object)
    }

    pub fn from_json_value(value: JsonValue) -> Result<Self, Error> {
        Self::try_from(value)
    }
}

impl TryFrom<JsonObject> for FeatureCollection {
    type Error = Error;

    fn try_from(mut object: JsonObject) -> Result<Self, Error> {
        match util::expect_type(&mut object)? {
            ref type_ if type_ == "FeatureCollection" => Ok(FeatureCollection {
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

impl TryFrom<JsonValue> for FeatureCollection {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self, Error> {
        if let JsonValue::Object(obj) = value {
            Self::try_from(obj)
        } else {
            Err(Error::GeoJsonExpectedObject(value))
        }
    }
}

impl Serialize for FeatureCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        JsonObject::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FeatureCollection {
    fn deserialize<D>(deserializer: D) -> Result<FeatureCollection, D::Error>
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
impl FromIterator<Feature> for FeatureCollection {
    fn from_iter<T: IntoIterator<Item = Feature>>(iter: T) -> Self {
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
        FeatureCollection {
            bbox,
            features,
            foreign_members: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Feature, FeatureCollection, Value};

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
}
