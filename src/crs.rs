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

use ::json::{Serialize, Deserialize, Serializer, Deserializer, JsonObject, json_val};

use ::{Error, FromObject};


/// Coordinate Reference System Objects
///
/// [GeoJSON Format Specification § 3]
/// (http://geojson.org/geojson-spec.html#coordinate-reference-system-objects)
#[derive(Clone, Debug, PartialEq)]
pub enum Crs {
    /// Named CRS
    ///
    /// [GeoJSON Format Specification § 3.1]
    /// (http://geojson.org/geojson-spec.html#named-crs)
    Named {
        name: String,
    },

    /// Linked CRS
    ///
    /// [GeoJSON Format Specification § 3.2]
    /// (http://geojson.org/geojson-spec.html#linked-crs)
    Linked {
        href: String,
        type_: Option<String>,
    },
}

impl<'a> From<&'a Crs> for JsonObject {
    fn from(crs: &'a Crs) -> JsonObject {
        let mut crs_map = JsonObject::new();
        let mut properties_map = JsonObject::new();
        match *crs {
            Crs::Named{ref name} => {
                crs_map.insert(String::from("type"), json_val(&String::from("name")));
                properties_map.insert(String::from("name"), json_val(name));
            }
            Crs::Linked{ref href, ref type_} => {
                crs_map.insert(String::from("type"), json_val(&String::from("link")));
                properties_map.insert(String::from("href"), json_val(href));
                if let Some(ref type_) = *type_ {
                    properties_map.insert(String::from("type"), json_val(type_));
                }
            }
        };
        crs_map.insert(String::from("properties"), json_val(&properties_map));
        return crs_map;
    }
}

impl FromObject for Crs {
    fn from_object(object: &JsonObject) -> Result<Self, Error> {
        let type_ = expect_type!(object);
        let properties = expect_object!(expect_property!(object, "properties", "Encountered CRS object type with no properties"));

        return Ok(match type_ {
            "name" => {
                let name = expect_string!(expect_property!(&properties, "name", "Encountered Named CRS object with no name"));
                Crs::Named {name: String::from(name)}
            },
            "link" => {
                let href = expect_string!(expect_property!(&properties, "href", "Encountered Linked CRS object with no link")).to_string();
                let type_ = match properties.get("type") {
                    Some(type_) => Some(expect_string!(type_).to_string()),
                    None => None,
                };
                Crs::Linked {type_: type_, href: href}
            },
            t => return Err(Error::CrsUnknownType(t.into())),
        });
    }
}

impl Serialize for Crs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        JsonObject::from(self).serialize(serializer)
    }
}

impl Deserialize for Crs {
    fn deserialize<D>(deserializer: D) -> Result<Crs, D::Error>
    where D: Deserializer {
        use std::error::Error as StdError;
        use serde::de::Error as SerdeError;

        let val = try!(JsonObject::deserialize(deserializer));

        Crs::from_object(&val).map_err(|e| D::Error::custom(e.description()))
    }
}
