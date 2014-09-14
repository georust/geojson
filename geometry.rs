// TODO
// impl ToGeojson for ....
// generic number instead of f64 for position?
// rename Position to Pos

extern crate serialize;

use std::collections::TreeMap;
use serialize::json::ToJson;
use serialize::json;


/*
 * Position
 * GeoJSON Format Specification § 2.1.1
 * http://geojson.org/geojson-spec.html#positions
 */
pub struct Position(Vec<f64>);

impl ToJson for Position {
    fn to_json(&self) -> json::Json {
        let &Position(ref nums) = self;
        nums.to_json()
    }
}


/*
 * Point
 * GeoJSON Format Specification § 2.1.2
 * http://geojson.org/geojson-spec.html#point
 */
pub struct Point {
    coordinates: Position,
}

impl ToJson for Point {
    fn to_json(&self) -> json::Json {
        let mut d = TreeMap::new();
        d.insert("type".to_string(), json::String("Point".to_string()));
        d.insert("coordinates".to_string(), self.coordinates.to_json());
        d.to_json()
    }
}


/*
 * MultiPoint
 * GeoJSON Format Specification § 2.1.3
 * http://geojson.org/geojson-spec.html#multipoint
 */
pub struct MultiPoint {
    coordinates: Vec<Position>,
}


fn main() {
    let point = Point {
        coordinates: Position(vec![1., 2., 3.]),
    };

    let j: json::Json = point.to_json();
    let s: String = j.to_pretty_str();

    println!("{}", s);
}
