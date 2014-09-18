// TODO
// impl ToGeojson for ....
// generic number instead of f64 for position?
// rename Position to Pos or Positions

extern crate serialize;

use std::collections::TreeMap;
use serialize::json::ToJson;
use serialize::json;


/// Position
///
/// [GeoJSON Format Specification § 2.1.1](http://geojson.org/geojson-spec.html#positions)
pub struct Position(pub Vec<f64>);

impl ToJson for Position {
    fn to_json(&self) -> json::Json {
        let &Position(ref nums) = self;
        nums.to_json()
    }
}

impl Clone for Position {
    fn clone(&self) -> Position {
        let &Position(ref nums) = self;
        Position(nums.clone())
    }
}


/// Point
///
/// [GeoJSON Format Specification § 2.1.2](http://geojson.org/geojson-spec.html#point)
pub struct Point {
    pub coordinates: Position,
}

impl ToJson for Point {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "Point".to_string().to_json());
        d.insert("coordinates".to_string(), self.coordinates.to_json());
        d.to_json()
    }
}


/// MultiPoint
///
/// [GeoJSON Format Specification § 2.1.3](http://geojson.org/geojson-spec.html#multipoint)
pub struct MultiPoint {
    pub points: Vec<Point>,
}

impl ToJson for MultiPoint {
    fn to_json(&self) -> json::Json {
        let coordinates: Vec<Position> =
            self.points.iter().map(|p| p.coordinates.clone()).collect();
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "MultiPoint".to_string().to_json());
        d.insert("coordinates".to_string(), coordinates.to_json());
        d.to_json()
    }
}


/// LineString
///
/// [GeoJSON Format Specification § 2.1.4](http://geojson.org/geojson-spec.html#linestring)
pub struct LineString {
    pub coordinates: Vec<Position>,
}

impl ToJson for LineString {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "LineString".to_string().to_json());
        d.insert("coordinates".to_string(), self.coordinates.to_json());
        d.to_json()
    }
}


/// MultiLineString
///
/// [GeoJSON Format Specification § 2.1.5](http://geojson.org/geojson-spec.html#multilinestring)
pub struct MultiLineString {
    pub line_strings: Vec<LineString>,
}

impl ToJson for MultiLineString {
    fn to_json(&self) -> json::Json {
        let coordinates: Vec<Vec<Position>> =
            self.line_strings.iter().map(|l| l.coordinates.clone()).collect();
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "MultiLineString".to_string().to_json());
        d.insert("coordinates".to_string(), coordinates.to_json());
        d.to_json()
    }
}


/// Polygon
///
/// [GeoJSON Format Specification § 2.1.6](http://geojson.org/geojson-spec.html#polygon)
pub struct Polygon {
    pub exterior: Vec<Position>,
    pub holes: Vec<Vec<Position>>,
}

impl Polygon {
    fn coordinates(&self) -> Vec<Vec<Position>> {
        let mut coordinates = self.holes.clone();
        coordinates.insert(0, self.exterior.clone());
        coordinates
    }
}

impl ToJson for Polygon {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "Polygon".to_string().to_json());
        d.insert("coordinates".to_string(), self.coordinates().to_json());
        d.to_json()
    }
}


/// MultiPolygon
///
/// [GeoJSON Format Specification § 2.1.7](http://geojson.org/geojson-spec.html#multipolygon)
pub struct MultiPolygon {
    pub polygons: Vec<Polygon>,
}

impl ToJson for MultiPolygon {
    fn to_json(&self) -> json::Json {
        let coordinates: Vec<Vec<Vec<Position>>> =
            self.polygons.iter().map(|p| p.coordinates()).collect();
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "MultiPolygon".to_string().to_json());
        d.insert("coordinates".to_string(), coordinates.to_json());
        d.to_json()
    }
}


/// Geometry
pub enum Geometry {
    Point(Point),
    MultiPoint(MultiPoint),
    LineString(LineString),
    MultiLineString(MultiLineString),
    Polygon(Polygon),
    MultiPolygon(MultiPolygon),
}

impl ToJson for Geometry {
    fn to_json(&self) -> json::Json {
        match *self {
            // TODO: is there a better way of doing this?
            Point(ref geom) => geom.to_json(),
            MultiPoint(ref geom) => geom.to_json(),
            LineString(ref geom) => geom.to_json(),
            MultiLineString(ref geom) => geom.to_json(),
            Polygon(ref geom) => geom.to_json(),
            MultiPolygon(ref geom) => geom.to_json(),
            // TODO: GeometryCollection(ref geom) => geom.to_json(),
        }
    }
}


/// GeometryCollection
///
/// [GeoJSON Format Specification § 2.1.8](http://geojson.org/geojson-spec.html#geometry-collection)
pub struct GeometryCollection {
    geometries: Vec<Geometry>,
}


impl ToJson for GeometryCollection {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), "GeometryCollection".to_string().to_json());
        d.insert("coordinates".to_string(), self.geometries.to_json());
        d.to_json()
    }
}


fn main() {
    let point = Point {
        coordinates: Position(vec![1., 2., 3.]),
    };

    let j: json::Json = point.to_json();
    let s: String = j.to_pretty_str();

    println!("{}", s);
}
