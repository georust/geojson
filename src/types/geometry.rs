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

use rustc_serialize::json::{Json, ToJson, Object};
use {Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon, GeometryCollection};

/// Geometry
#[derive(RustcEncodable, Clone)]
pub enum Geometry {
    Point(Point),
    MultiPoint(MultiPoint),
    LineString(LineString),
    MultiLineString(MultiLineString),
    Polygon(Polygon),
    MultiPolygon(MultiPolygon),
    GeometryCollection(GeometryCollection),
}

impl ToJson for Geometry {
    fn to_json(&self) -> Json {
        match *self {
            Geometry::Point(ref geom) => geom.to_json(),
            Geometry::MultiPoint(ref geom) => geom.to_json(),
            Geometry::LineString(ref geom) => geom.to_json(),
            Geometry::MultiLineString(ref geom) => geom.to_json(),
            Geometry::Polygon(ref geom) => geom.to_json(),
            Geometry::MultiPolygon(ref geom) => geom.to_json(),
            Geometry::GeometryCollection(ref geom) => geom.to_json(),
        }
    }
}

impl Geometry {
    pub fn from_json(json_geometry: &Object) -> Geometry {
        match json_geometry.get("type").unwrap().as_string().unwrap() {
            "Point" => Geometry::Point(Point::from_json(json_geometry)),
            "MultiPoint" => Geometry::MultiPoint(MultiPoint::from_json(json_geometry)),
            "LineString" => Geometry::LineString(LineString::from_json(json_geometry)),
            "MultiLineString" => Geometry::MultiLineString(MultiLineString::from_json(json_geometry)),
            "Polygon" => Geometry::Polygon(Polygon::from_json(json_geometry)),
            "MultiPolygon" => Geometry::MultiPolygon(MultiPolygon::from_json(json_geometry)),
            "GeometryCollection" => Geometry::GeometryCollection(GeometryCollection::from_json(json_geometry)),
            _ => panic!(),
        }
    }
}

#[test]
fn test_match_geometry_type() {
    fn geom(json_str: &str) -> Geometry {
        let json = Json::from_str(json_str).unwrap();
        return Geometry::from_json(json.as_object().unwrap());
    }

    match geom("{\"coordinates\":[],\"type\":\"Point\"}") {
        Geometry::Point(ref _geom) => (),
        _ => panic!("expected Point")
    };

    match geom("{\"coordinates\":[],\"type\":\"MultiPoint\"}") {
        Geometry::MultiPoint(ref _geom) => (),
        _ => panic!("expected MultiPoint")
    };

    match geom("{\"coordinates\":[],\"type\":\"LineString\"}") {
        Geometry::LineString(ref _geom) => (),
        _ => panic!("expected LineString")
    };

    match geom("{\"coordinates\":[],\"type\":\"MultiLineString\"}") {
        Geometry::MultiLineString(ref _geom) => (),
        _ => panic!("expected MultiLineString")
    };

    match geom("{\"coordinates\":[],\"type\":\"Polygon\"}") {
        Geometry::Polygon(ref _geom) => (),
        _ => panic!("expected Polygon")
    };

    match geom("{\"coordinates\":[],\"type\":\"MultiPolygon\"}") {
        Geometry::MultiPolygon(ref _geom) => (),
        _ => panic!("expected MultiPolygon")
    };

    match geom("{\"geometries\":[],\"type\":\"GeometryCollection\"}") {
        Geometry::GeometryCollection(ref _geom) => (),
        _ => panic!("expected GeometryCollection")
    };
}
