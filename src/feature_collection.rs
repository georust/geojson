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

use crate::json::{Deserialize, Deserializer, JsonObject, JsonValue, Serialize, Serializer};
use crate::serde_json::json;
use crate::{util, Bbox, Error, Feature, Position};

/// Feature Collection Objects
///
/// [GeoJSON Format Specification § 3.3](https://tools.ietf.org/html/rfc7946#section-3.3)
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
///     features: vec![],
///     foreign_members: None,
/// };
///
/// let serialized = GeoJson::<(f64, f64)>::from(feature_collection).to_string();
///
/// assert_eq!(
///     serialized,
///     "{\"features\":[],\"type\":\"FeatureCollection\"}"
/// );
/// # }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct FeatureCollection<Pos: Position = crate::DefaultPositionImpl> {
    /// Bounding Box
    ///
    /// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
    pub bbox: Option<Bbox>,
    pub features: Vec<Feature<Pos>>,
    /// Foreign Members
    ///
    /// [GeoJSON Format Specification § 6](https://tools.ietf.org/html/rfc7946#section-6)
    pub foreign_members: Option<JsonObject>,
}

impl<'a, Pos: Position> From<&'a FeatureCollection<Pos>> for JsonObject {
    fn from(fc: &'a FeatureCollection<Pos>) -> JsonObject {
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

impl<Pos: Position> FeatureCollection<Pos> {
    pub fn from_json_object(object: JsonObject) -> Result<Self, Error<Pos>> {
        Self::try_from(object)
    }

    pub fn from_json_value(value: JsonValue) -> Result<Self, Error<Pos>> {
        Self::try_from(value)
    }
}

impl<Pos: Position> TryFrom<JsonObject> for FeatureCollection<Pos> {
    type Error = Error<Pos>;

    fn try_from(mut object: JsonObject) -> Result<Self, Error<Pos>> {
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

impl<Pos: Position> TryFrom<JsonValue> for FeatureCollection<Pos> {
    type Error = Error<Pos>;

    fn try_from(value: JsonValue) -> Result<Self, Error<Pos>> {
        if let JsonValue::Object(obj) = value {
            Self::try_from(obj)
        } else {
            Err(Error::GeoJsonExpectedObject(value))
        }
    }
}

impl<Pos: Position> Serialize for FeatureCollection<Pos> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        JsonObject::from(self).serialize(serializer)
    }
}

impl<'de, Pos: Position> Deserialize<'de> for FeatureCollection<Pos> {
    fn deserialize<D>(deserializer: D) -> Result<FeatureCollection<Pos>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as SerdeError;

        let val = JsonObject::deserialize(deserializer)?;

        FeatureCollection::from_json_object(val).map_err(|e| D::Error::custom(e.to_string()))
    }
}
