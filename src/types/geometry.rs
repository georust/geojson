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
        match(json_geometry.get("type").unwrap().as_string().unwrap()) {
            "MultiPolygon" => Geometry::MultiPolygon(MultiPolygon::from_json(json_geometry)),
            _ => panic!(),
        }
    }
}
