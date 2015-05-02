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

use rustc_serialize::json::{self, ToJson};

use ::{Error, FromObject};


#[derive(Clone, Debug, PartialEq)]
pub enum Crs {
    Named {
        name: String,
    },
    Linked {
        href: String,
        type_: Option<String>,
    },
}

impl<'a> From<&'a Crs> for json::Object {
    fn from(crs: &'a Crs) -> json::Object {
        let mut crs_map = BTreeMap::new();
        let mut properties_map = BTreeMap::new();
        match *crs {
            Crs::Named{ref name} => {
                crs_map.insert(String::from("type"), "name".to_json());
                properties_map.insert(String::from("name"), name.to_json());
            }
            Crs::Linked{ref href, ref type_} => {
                crs_map.insert(String::from("type"), "link".to_json());
                properties_map.insert(String::from("href"), href.to_json());
                if let Some(ref type_) = *type_ {
                    properties_map.insert(String::from("type"), type_.to_json());
                }
            }
        };
        crs_map.insert(String::from("properties"), properties_map.to_json());
        crs_map
    }
}

impl FromObject for Crs {
    fn from_object(object: &json::Object) -> Result<Self, Error> {
        let type_ = expect_type!(object);
        let properties = expect_object!(expect_property!(object, "properties", "Encountered CRS object type with no properties"));
        Ok(match type_ {
            "name" => {
                let name = expect_string!(expect_property!(properties, "name", "Encountered Named CRS object with no name"));
                Crs::Named {name: String::from(name)}
            },
            "link" => {
                let href = expect_string!(expect_property!(properties, "href", "Encountered Linked CRS object with no link")).to_string();
                let type_ = match properties.get("type") {
                    Some(type_) => Some(expect_string!(type_).to_string()),
                    None => None,
                };
                Crs::Linked {type_: type_, href: href}
            },
            _ => return Err(Error::new("Encountered unknown CRS type")),
        })
    }
}

impl ToJson for Crs {
    fn to_json(&self) -> json::Json {
        json::Json::Object(self.into())
    }
}
