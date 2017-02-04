// Copyright 2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ::json::{Serialize, Deserialize, Serializer, Deserializer, JsonObject};
use serde_json;

use ::{Bbox, Crs, Error, Feature, FromObject, util};


/// Feature Collection Objects
///
/// [GeoJSON Format Specification § 2.3]
/// (http://geojson.org/geojson-spec.html#feature-collection-objects)
///
/// # Examples
///
/// Serialization:
///
/// ```
/// # extern crate geojson;
/// # fn main() {
/// use geojson::FeatureCollection;
/// use geojson::GeoJson;
///
/// let feature_collection = FeatureCollection {
///     bbox: None,
///     crs: None,
///     features: vec![],
/// };
///
/// let serialized = GeoJson::from(feature_collection).to_string();
///
/// assert_eq!(
///     serialized,
///     "{\"features\":[],\"type\":\"FeatureCollection\"}"
/// );
/// # }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct FeatureCollection {
    pub bbox: Option<Bbox>,
    pub crs: Option<Crs>,
    pub features: Vec<Feature>,
}

impl<'a> From<&'a FeatureCollection> for JsonObject {
    fn from(fc: &'a FeatureCollection) -> JsonObject {
        let mut map = JsonObject::new();
        map.insert(String::from("type"), json!("FeatureCollection"));
        map.insert(String::from("features"), serde_json::to_value(&fc.features).unwrap());

        if let Some(ref crs) = fc.crs {
            map.insert(String::from("crs"), serde_json::to_value(crs).unwrap());
        }

        if let Some(ref bbox) = fc.bbox {
            map.insert(String::from("bbox"), serde_json::to_value(bbox).unwrap());
        }

        return map;
    }
}

impl FromObject for FeatureCollection {
    fn from_object(object: &JsonObject) -> Result<Self, Error> {
        return Ok(FeatureCollection{
            bbox: try!(util::get_bbox(object)),
            features: try!(util::get_features(object)),
            crs: try!(util::get_crs(object)),
        });
    }
}

impl Serialize for FeatureCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        JsonObject::from(self).serialize(serializer)
    }
}

impl Deserialize for FeatureCollection {
    fn deserialize<D>(deserializer: D) -> Result<FeatureCollection, D::Error>
    where D: Deserializer {
        use std::error::Error as StdError;
        use serde::de::Error as SerdeError;

        let val = try!(JsonObject::deserialize(deserializer));

        FeatureCollection::from_object(&val).map_err(|e| D::Error::custom(e.description()))
    }
}
