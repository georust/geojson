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

use std::collections::BTreeMap;

#[cfg(not(feature = "with-serde"))]
use ::json::ToJson;
#[cfg(feature = "with-serde")]
use ::json::{Serialize, Deserialize, Serializer, Deserializer, SerdeError};

use ::json::{JsonValue, JsonObject, json_val};

use ::{Bbox, Crs, Error, Feature, FromObject, util};


/// Feature Collection Objects
///
/// [GeoJSON Format Specification § 2.3]
/// (http://geojson.org/geojson-spec.html#feature-collection-objects)
#[derive(Clone, Debug, PartialEq)]
pub struct FeatureCollection {
    pub bbox: Option<Bbox>,
    pub crs: Option<Crs>,
    pub features: Vec<Feature>,
}


impl<'a> From<&'a FeatureCollection> for JsonObject {
    fn from(fc: &'a FeatureCollection) -> JsonObject {
        let mut map = BTreeMap::new();
        map.insert(String::from("type"), json_val(&String::from("FeatureCollection")));
        map.insert(String::from("features"), json_val(&fc.features));

        if let Some(ref crs) = fc.crs {
            map.insert(String::from("crs"), json_val(crs));
        }

        if let Some(ref bbox) = fc.bbox {
            map.insert(String::from("bbox"), json_val(bbox));
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

#[cfg(not(feature = "with-serde"))]
impl ToJson for FeatureCollection {
    fn to_json(&self) -> JsonValue {
        return ::rustc_serialize::json::Json::Object(self.into());
    }
}

#[cfg(feature = "with-serde")]
impl Serialize for FeatureCollection {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where S: Serializer {
        JsonObject::from(self).serialize(serializer)
    }
}

#[cfg(feature = "with-serde")]
impl Deserialize for FeatureCollection {
    fn deserialize<D>(deserializer: &mut D) -> Result<FeatureCollection, D::Error>
    where D: Deserializer {
        use std::error::Error as StdError;

        let val = try!(JsonValue::deserialize(deserializer));

        if let Some(features) = val.as_object() {
            FeatureCollection::from_object(features).map_err(|e| D::Error::custom(e.description()))
        }
        else {
            Err(D::Error::custom("expected json object"))
        }
    }
}
