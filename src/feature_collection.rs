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

use rustc_serialize::json::{self, Json, ToJson};

use ::{Bbox, Crs, Error, Feature, FromObject};
use ::util::ObjectUtils;


/// FeatureCollection
///
/// [GeoJSON Format Specification § 2.3](http://geojson.org/geojson-spec.html#feature-collection-objects)
#[derive(Clone, Debug, PartialEq)]
pub struct FeatureCollection {
    pub bbox: Option<Bbox>,
    pub crs: Option<Crs>,
    pub features: Vec<Feature>,
}


impl<'a> From<&'a FeatureCollection> for json::Object {
    fn from(fc: &'a FeatureCollection) -> json::Object {
        let mut map = BTreeMap::new();
        map.insert(String::from("type"), "FeatureCollection".to_json());
        map.insert(String::from("features"), fc.features.to_json());

        if let Some(ref crs) = fc.crs {
            map.insert(String::from("crs"), crs.to_json());
        }

        if let Some(ref bbox) = fc.bbox {
            map.insert(String::from("bbox"), bbox.to_json());
        }

        map
    }
}

impl FromObject for FeatureCollection {
    fn from_object(object: &json::Object) -> Result<Self, Error> {
        Ok(FeatureCollection{
            bbox: try!(object.get_bbox()),
            features: try!(object.get_features()),
            crs: try!(object.get_crs()),
        })
    }
}

impl ToJson for FeatureCollection {
    fn to_json(&self) -> json::Json {
        json::Json::Object(self.into())
    }
}
