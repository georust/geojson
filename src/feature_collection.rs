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

use std::iter::FromIterator;
use std::str::FromStr;

use crate::errors::{Error, Result};
use crate::JsonObject;
use crate::{Bbox, Feature};
use serde::{Deserialize, Serialize};

/// Feature Collection Object
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
/// let serialized = feature_collection.to_string();
/// assert_eq!(serialized, r#"{"type":"FeatureCollection","features":[]}"#);
/// ```
///
/// Deserializing a GeoJSON string into a `FeatureCollection`:
///
/// ```
/// use geojson::{FeatureCollection, Feature, Geometry};
///
/// let geojson_str = r#"
/// {
///   "type": "FeatureCollection",
///   "features": [
///     {
///       "type": "Feature",
///       "properties": {},
///       "geometry": {
///         "type": "Point",
///         "coordinates": [-1.0, -2.0]
///       }
///     }
///   ]
/// }"#;
///
/// let feature_collection = geojson_str
///     .parse::<FeatureCollection>()
///     .expect("valid FeatureCollection GeoJSON");
///
/// let expected = FeatureCollection::from_iter(vec![
///     Feature::from(Geometry::new_point([-1.0, -2.0]))
/// ]);
/// assert_eq!(feature_collection, expected);
/// ```
///
/// Collect from an iterator:
///
/// ```rust
/// use geojson::{Feature, FeatureCollection, Geometry};
///
/// let fc: FeatureCollection = (0..10)
///     .map(|idx| -> Feature {
///         let c = idx as f64;
///         Geometry::new_point([1.0 * c, 2.0 * c, 3.0 * c]).into()
///     })
///     .collect();
/// assert_eq!(fc.features.len(), 10);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", from = "deserialize::DeserializeFeatureCollectionHelper")]
pub struct FeatureCollection {
    /// Bounding Box
    ///
    /// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Bbox>,
    pub features: Vec<Feature>,
    /// Foreign Members
    ///
    /// [GeoJSON Format Specification § 6](https://tools.ietf.org/html/rfc7946#section-6.1)
    ///
    /// See the [crate-level foreign members documentation](crate#foreign-members) for details,
    /// including limitations on key names.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub foreign_members: Option<JsonObject>,
}

mod deserialize {
    use super::*;
    use crate::util::normalize_foreign_members;

    /// The purpose of this helper is to verify that "type = FeatureCollection" during
    /// deserialization by explicitly encoding the type as an enum with one member.
    ///
    /// It's dumb, but otherwise serde will ignore the `#[serde(tag="type")]`, and happily
    /// deserialize (e.g.) `"type": "Point"` as a FeatureCollection.
    ///
    /// See: https://github.com/serde-rs/serde/issues/3028
    #[derive(Deserialize)]
    pub(crate) struct DeserializeFeatureCollectionHelper {
        #[allow(unused)]
        r#type: FeatureCollectionType,
        bbox: Option<Bbox>,
        features: Vec<Feature>,
        #[serde(flatten)]
        foreign_members: Option<JsonObject>,
    }

    #[derive(Deserialize)]
    enum FeatureCollectionType {
        FeatureCollection,
    }

    impl From<DeserializeFeatureCollectionHelper> for FeatureCollection {
        fn from(mut value: DeserializeFeatureCollectionHelper) -> Self {
            normalize_foreign_members(&mut value.foreign_members);
            Self {
                bbox: value.bbox,
                features: value.features,
                foreign_members: value.foreign_members,
            }
        }
    }
}

impl IntoIterator for FeatureCollection {
    type Item = Feature;
    type IntoIter = std::vec::IntoIter<Feature>;

    fn into_iter(self) -> Self::IntoIter {
        self.features.into_iter()
    }
}

impl<'a> IntoIterator for &'a FeatureCollection {
    type Item = &'a Feature;
    type IntoIter = std::slice::Iter<'a, Feature>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self.features)
    }
}

impl FromStr for FeatureCollection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
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
                        curr_bbox.clone_from(fbox);
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
    use crate::{Feature, FeatureCollection, GeoJson, Geometry};
    use serde_json::json;

    use std::str::FromStr;
    #[test]
    fn test_fc_from_iterator() {
        let features: Vec<Feature> = vec![
            {
                let mut feat: Feature = Geometry::new_point([0., 0., 0.]).into();
                feat.bbox = Some(vec![-1., -1., -1., 1., 1., 1.]);
                feat
            },
            {
                let mut feat: Feature =
                    Geometry::new_multi_point([[10., 10., 10.], [11., 11., 11.]]).into();
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
    fn parsing() {
        let geojson_str = feature_collection_json();
        let feature_collection_1: FeatureCollection = geojson_str.parse().unwrap();
        let feature_collection_2: FeatureCollection = serde_json::from_str(&geojson_str).unwrap();
        assert_eq!(feature_collection_1, feature_collection_2);
        let GeoJson::FeatureCollection(feature_collection_3): GeoJson =
            geojson_str.parse().unwrap()
        else {
            panic!("unexpected GeoJSON type");
        };
        let GeoJson::FeatureCollection(feature_collection_4): GeoJson =
            serde_json::from_str(&geojson_str).unwrap()
        else {
            panic!("unexpected GeoJSON type");
        };
        assert_eq!(feature_collection_3, feature_collection_4);

        assert_eq!(feature_collection_1, feature_collection_4);
    }

    #[test]
    fn test_from_str_ok() {
        let feature_collection = FeatureCollection::from_str(&feature_collection_json()).unwrap();
        assert_eq!(2, feature_collection.features.len());
    }

    #[test]
    fn wrong_type() {
        let geojson_str = json!({
            "type": "Point",
            "coordinates": [1.1, 2.1]
        })
        .to_string();
        Geometry::from_str(&geojson_str).unwrap();
        FeatureCollection::from_str(&geojson_str).unwrap_err();
        serde_json::from_str::<FeatureCollection>(&geojson_str).unwrap_err();
    }

    #[test]
    fn iter_features() {
        let feature_collection = FeatureCollection::from_str(&feature_collection_json()).unwrap();

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
    fn encode_decode_feature_collection_with_foreign_members() {
        let mut foreign_members = serde_json::Map::new();
        foreign_members.insert("extra".to_string(), serde_json::json!("data"));

        let feature_collection = FeatureCollection {
            bbox: None,
            features: vec![],
            foreign_members: Some(foreign_members),
        };

        let json_string = serde_json::to_string(&feature_collection).unwrap();
        let decoded: FeatureCollection = json_string.parse().unwrap();
        assert_eq!(decoded, feature_collection);
    }
}
