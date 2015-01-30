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
use {Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon, GeometryCollection, GeoJsonResult, GeoJsonError};

/// Geometry
#[derive(RustcEncodable, Clone, Debug)]
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
    pub fn from_json(json_geometry: &Object) -> GeoJsonResult<Geometry> {
        match expect_string!(expect_property!(json_geometry, "type", "Missing 'type' field")) {
            "Point" => Ok(Geometry::Point(try!(Point::from_json(json_geometry)))),
            "MultiPoint" => Ok(Geometry::MultiPoint(try!(MultiPoint::from_json(json_geometry)))),
            "LineString" => Ok(Geometry::LineString(try!(LineString::from_json(json_geometry)))),
            "MultiLineString" => Ok(Geometry::MultiLineString(try!(MultiLineString::from_json(json_geometry)))),
            "Polygon" => Ok(Geometry::Polygon(try!(Polygon::from_json(json_geometry)))),
            "MultiPolygon" => Ok(Geometry::MultiPolygon(try!(MultiPolygon::from_json(json_geometry)))),
            "GeometryCollection" => Ok(Geometry::GeometryCollection(try!(GeometryCollection::from_json(json_geometry)))),
            _ => Err(GeoJsonError::new("Unknown geometry type")),
        }
    }
}


#[cfg(test)]

mod tests {
    use rustc_serialize::json::Json;
    use Geometry;
    use GeoJsonResult;

    #[test]
    fn test_match_geometry_type() {
        fn geom(json_str: &str) -> GeoJsonResult<Geometry> {
            let json = Json::from_str(json_str).unwrap();
            return Geometry::from_json(expect_object!(json));
        }

        match geom("{\"coordinates\":[],\"type\":\"Point\"}") {
            Ok(Geometry::Point(ref _geom)) => (),
            _ => panic!("expected Point")
        };

        match geom("{\"coordinates\":[],\"type\":\"MultiPoint\"}") {
            Ok(Geometry::MultiPoint(ref _geom)) => (),
            _ => panic!("expected MultiPoint")
        };

        match geom("{\"coordinates\":[],\"type\":\"LineString\"}") {
            Ok(Geometry::LineString(ref _geom)) => (),
            _ => panic!("expected LineString")
        };

        match geom("{\"coordinates\":[],\"type\":\"MultiLineString\"}") {
            Ok(Geometry::MultiLineString(ref _geom)) => (),
            _ => panic!("expected MultiLineString")
        };

        match geom("{\"coordinates\":[],\"type\":\"Polygon\"}") {
            Ok(Geometry::Polygon(ref _geom)) => (),
            _ => panic!("expected Polygon")
        };

        match geom("{\"coordinates\":[],\"type\":\"MultiPolygon\"}") {
            Ok(Geometry::MultiPolygon(ref _geom)) => (),
            _ => panic!("expected MultiPolygon")
        };

        match geom("{\"geometries\":[],\"type\":\"GeometryCollection\"}") {
            Ok(Geometry::GeometryCollection(ref _geom)) => (),
            _ => panic!("expected GeometryCollection")
        };

        match geom("{\"type\":\"something else\"}") {
            Ok(_) => panic!("expected error value"),
            Err(_) => ()
        };

        match geom("{}") {
            Ok(_) => panic!("expected error value"),
            Err(_) => ()
        };
    }
}
