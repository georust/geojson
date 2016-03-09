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

use std::str::FromStr;

use rustc_serialize::json::{self, ToJson};

use ::{Error, Geometry, Feature, FeatureCollection, FromObject};


/// GeoJSON Objects
///
/// [GeoJSON Format Specification § 2]
/// (http://geojson.org/geojson-spec.html#geojson-objects)
#[derive(Clone, Debug, PartialEq)]
pub enum GeoJson {
    Geometry(Geometry),
    Feature(Feature),
    FeatureCollection(FeatureCollection),
}

impl<'a> From<&'a GeoJson> for json::Object {
    fn from(geojson: &'a GeoJson) -> json::Object {
        return match *geojson {
            GeoJson::Geometry(ref geometry) => geometry.into(),
            GeoJson::Feature(ref feature) => feature.into(),
            GeoJson::FeatureCollection(ref fc) => fc.into(),
        };
    }
}


impl FromObject for GeoJson {
    fn from_object(object: &json::Object) -> Result<Self, Error> {
        let type_ = expect_string!(expect_property!(object, "type", "Missing 'type' field"));
        return match &type_ as &str {
            "Point" | "MultiPoint" | "LineString" | "MultiLineString" | "Polygon" | "MultiPolygon" =>
                Geometry::from_object(object).map(GeoJson::Geometry),
            "Feature" =>
                Feature::from_object(object).map(GeoJson::Feature),
            "FeatureCollection" =>
                FeatureCollection::from_object(object).map(GeoJson::FeatureCollection),
            _ => Err(Error::GeoJsonUnknownType),
        };
    }
}

impl json::ToJson for GeoJson {
    fn to_json(&self) -> json::Json {
        return json::Json::Object(self.into());
    }
}

impl FromStr for GeoJson {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded_json = match json::Json::from_str(s) {
            Ok(j) => j,
            Err(..) => return Err(Error::MalformedJson),
        };
        let object = match decoded_json {
            json::Json::Object(object) => object,
            _ => return Err(Error::GeoJsonExpectedObject),
        };
        return GeoJson::from_object(&object);
    }
}

impl ToString for GeoJson {
    fn to_string(&self) -> String {
        return self.to_json().to_string();
    }
}
