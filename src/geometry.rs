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

use ::{Bbox, Crs, Error, LineStringType, PointType, PolygonType, FromObject, util};


#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Point(PointType),
    MultiPoint(Vec<PointType>),
    LineString(LineStringType),
    MultiLineString(Vec<LineStringType>),
    Polygon(PolygonType),
    MultiPolygon(Vec<PolygonType>),
    GeometryCollection(Vec<Geometry>),
}

impl ToJson for Value {
    fn to_json(&self) -> json::Json {
        return match *self {
            Value::Point(ref x) => x.to_json(),
            Value::MultiPoint(ref x) => x.to_json(),
            Value::LineString(ref x) => x.to_json(),
            Value::MultiLineString(ref x) => x.to_json(),
            Value::Polygon(ref x) => x.to_json(),
            Value::MultiPolygon(ref x) => x.to_json(),
            Value::GeometryCollection(ref x) => x.to_json(),
        };
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Geometry {
    pub bbox: Option<Bbox>,
    pub value: Value,
    pub crs: Option<Crs>,
}

impl<'a> From<&'a Geometry> for json::Object {
    fn from(geometry: &'a Geometry) -> json::Object {
        let mut map = BTreeMap::new();
        if let Some(ref crs) = geometry.crs {
            map.insert(String::from("crs"), crs.to_json());
        }
        if let Some(ref bbox) = geometry.bbox {
            map.insert(String::from("bbox"), bbox.to_json());
        }

        map.insert(String::from("type"), match geometry.value {
            Value::Point(..) => "Point",
            Value::MultiPoint(..) => "MultiPoint",
            Value::LineString(..) => "LineString",
            Value::MultiLineString(..) => "MultiLineString",
            Value::Polygon(..) => "Polygon",
            Value::MultiPolygon(..) => "MultiPolygon",
            Value::GeometryCollection(..) => "GeometryCollection",
        }.to_json());

        map.insert(String::from(match geometry.value {
            Value::GeometryCollection(..) => "geometries",
            _ => "coordinates",
        }), geometry.value.to_json());
        return map;
    }
}

impl FromObject for Geometry {
    fn from_object(object: &json::Object) -> Result<Self, Error> {
        let type_ = expect_type!(object);
        let value = match type_ {
            "Point" =>
                Value::Point(try!(util::get_coords_one_pos(object))),
            "MultiPoint" =>
                Value::MultiPoint(try!(util::get_coords_1d_pos(object))),
            "LineString" =>
                Value::LineString(try!(util::get_coords_1d_pos(object))),
            "MultiLineString" =>
                Value::MultiLineString(try!(util::get_coords_2d_pos(object))),
            "Polygon" =>
                Value::Polygon(try!(util::get_coords_2d_pos(object))),
            "MultiPolygon" =>
                Value::MultiPolygon(try!(util::get_coords_3d_pos(object))),
            "GeometryCollection" =>
                Value::GeometryCollection(try!(util::get_geometries(object))),
            _ => return Err(Error::new("Unknown geometry type")),
        };

        let bbox = try!(util::get_bbox(object));
        let crs = try!(util::get_crs(object));

        return Ok(Geometry {
            bbox: bbox,
            value: value,
            crs: crs,
        });
    }
}

impl ToJson for Geometry {
    fn to_json(&self) -> json::Json {
        return json::Json::Object(self.into());
    }
}


#[cfg(test)]
mod tests {
    use rustc_serialize::json::{self, ToJson};
    use super::super::{GeoJson, Geometry, Value};

    #[test]
    fn encode_decode_geometry() {
        let geometry_json_str = "{\"coordinates\":[1.0,2.0],\"type\":\"Point\"}";
        let geometry = Geometry {
            value: Value::Point(vec![1., 2.]),
            crs: None,
            bbox: None,
        };

        // Test encode
        let json_string = json::encode(&geometry.to_json()).unwrap();
        assert_eq!(json_string, geometry_json_str);

        // Test decode
        let decoded_geometry = match json_string.parse() {
            Ok(GeoJson::Geometry(g)) => g,
            _ => unreachable!(),
        };
        assert_eq!(decoded_geometry, geometry);
    }
}
