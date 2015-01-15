// Copyright 2014-2015 The GeoRust Developers
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

// TODO
// generic number instead of f64 for position?

extern crate "rustc-serialize" as rustc_serialize;

use std::collections::BTreeMap;
use rustc_serialize::json::{self, ToJson, Json};

// /// ToGeoJson
// pub trait ToGeoJson {
//     // TODO: change this to return a generic GeoJSON object
//     fn to_geojson(&self) -> Geometry;
// }


/// Pos (alias for Positions)
///
/// [GeoJSON Format Specification § 2.1.1](http://geojson.org/geojson-spec.html#positions)
#[derive(RustcEncodable, Clone)]
pub struct Pos(pub Vec<f64>);

impl ToJson for Pos {
    fn to_json(&self) -> json::Json {
        let &Pos(ref nums) = self;
        nums.to_json()
    }
}

/// Ring
#[derive(RustcEncodable, Clone)]
pub struct Ring(pub Vec<Pos>);

impl ToJson for Ring {
    fn to_json(&self) -> json::Json {
       let &Ring(ref points) = self;
        points.to_json()
    }
}

/// Poly  (alias for Polygon)
#[derive(RustcEncodable, Clone)]
pub struct Poly(pub Vec<Ring>);

impl ToJson for Poly {
    fn to_json(&self) -> json::Json {
        let &Poly(ref rings) = self;
        rings.to_json()
    }
}

/// Point
///
/// [GeoJSON Format Specification § 2.1.2](http://geojson.org/geojson-spec.html#point)
#[derive(RustcEncodable, Clone)]
pub struct Point {
    pub coordinates: Pos,
}

impl ToJson for Point {
    fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "Point".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}
#[test]
fn test_point_tojson() {
    let point = Point {coordinates: Pos(vec![1., 2., 3.])};
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"coordinates\":[1.0,2.0,3.0],\"type\":\"Point\"}", json_string);
}

/// MultiPoint
///
/// [GeoJSON Format Specification § 2.1.3](http://geojson.org/geojson-spec.html#multipoint)
#[derive(RustcEncodable, Clone)]
pub struct MultiPoint {
    pub coordinates: Vec<Pos>,
}

impl ToJson for MultiPoint {
    fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "MultiPoint".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}
#[test]
fn test_multi_point_tojson() {
    let point = MultiPoint {coordinates: vec![Pos(vec![1., 2., 3.])]};
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"coordinates\":[[1.0,2.0,3.0]],\"type\":\"MultiPoint\"}", json_string);
}

/// LineString
///
/// [GeoJSON Format Specification § 2.1.4](http://geojson.org/geojson-spec.html#linestring)
#[derive(RustcEncodable, Clone)]
pub struct LineString {
    pub coordinates: Ring,
}

impl ToJson for LineString {
    fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "LineString".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}

#[test]
fn test_line_string_tojson() {
    let point = LineString {coordinates: Ring(vec![Pos(vec![1., 2., 3.]), Pos(vec![2., 4., 3.])])};
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"coordinates\":[[1.0,2.0,3.0],[2.0,4.0,3.0]],\"type\":\"LineString\"}", json_string);
}
/// MultiLineString
///
/// [GeoJSON Format Specification § 2.1.5](http://geojson.org/geojson-spec.html#multilinestring)
#[derive(RustcEncodable, Clone)]
pub struct MultiLineString {
    pub coordinates: Vec<Ring>,
}

impl ToJson for MultiLineString {
    fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "MultiLineString".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}
#[test]
fn test_multi_line_string_tojson() {
    let point = MultiLineString {coordinates: vec![
        Ring(vec![Pos(vec![1., 2., 3.]), Pos(vec![2., 4., 3.])]),
        Ring(vec![Pos(vec![3., 2., 3.]), Pos(vec![2., 4., 3.])])
        ]};
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"coordinates\":[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]],\"type\":\"MultiLineString\"}", json_string);
}

/// Polygon
///
/// [GeoJSON Format Specification § 2.1.6](http://geojson.org/geojson-spec.html#polygon)
#[derive(RustcEncodable, Clone)]
pub struct Polygon {
    pub coordinates: Poly
}
impl ToJson for Polygon {
    fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "Polygon".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}
#[test]
fn test_polygon_string_tojson() {
    let point = Polygon {coordinates: Poly(vec![
        Ring(vec![Pos(vec![1., 2., 3.]), Pos(vec![2., 4., 3.])]),
        Ring(vec![Pos(vec![3., 2., 3.]), Pos(vec![2., 4., 3.])])
        ])};
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"coordinates\":[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]],\"type\":\"Polygon\"}", json_string);
}

/// MultiPolygon
///
/// [GeoJSON Format Specification § 2.1.7](http://geojson.org/geojson-spec.html#multipolygon)
#[derive(RustcEncodable, Clone)]
pub struct MultiPolygon {
    pub coordinates: Vec<Poly>,
}

impl ToJson for MultiPolygon {
    fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "MultiPolygon".to_json());
        d.insert(format!("coordinates"), self.coordinates.to_json());
        d.to_json()
    }
}
#[test]
fn test_multi_polygon_string_tojson() {
    let point = MultiPolygon {coordinates: vec![Poly(vec![
        Ring(vec![Pos(vec![1., 2., 3.]), Pos(vec![2., 4., 3.])]),
        Ring(vec![Pos(vec![3., 2., 3.]), Pos(vec![2., 4., 3.])])
        ])]};
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"}", json_string);
}

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
    fn to_json(&self) -> json::Json {
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
/// GeometryCollection
///
/// [GeoJSON Format Specification § 2.1.8](http://geojson.org/geojson-spec.html#geometry-collection)
#[derive(RustcEncodable, Clone)]
pub struct GeometryCollection {
    geometries: Vec<Geometry>,
}

impl ToJson for GeometryCollection {
    fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "GeometryCollection".to_json());
        d.insert(format!("geometries"), self.geometries.to_json());
        d.to_json()
    }
}
#[test]
fn test_geometry_collection_string_tojson() {
    let point = GeometryCollection {
        geometries: vec![Geometry::MultiPolygon(MultiPolygon {
                    coordinates: vec![
                        Poly(vec![
                            Ring(vec![
                                Pos(vec![1., 2., 3.]),
                                Pos(vec![2., 4., 3.])
                            ]),
                            Ring(vec![
                                Pos(vec![3., 2., 3.]),
                                Pos(vec![2., 4., 3.])
                            ])
                        ])
                    ]
                })
        ]
    };
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"geometries\":[{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"}],\"type\":\"GeometryCollection\"}", json_string);
}
/// Feature
///
/// [GeoJSON Format Specification § 2.2](http://geojson.org/geojson-spec.html#feature-objects)
pub struct Feature {
    geometry: Geometry,
    properties: json::Json,
}

impl ToJson for Feature {
    fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "Feature".to_json());
        d.insert(format!("geometry"), self.geometry.to_json());
        d.insert(format!("properties"), self.properties.to_json());
        d.to_json()
    }
}
#[test]
fn test_feature_string_tojson() {
    let mut map = BTreeMap::new();
    map.insert(format!("hi"), "there".to_json());
    let point = Feature {
        geometry: Geometry::MultiPolygon(MultiPolygon {
                coordinates: vec![
                    Poly(vec![
                        Ring(vec![
                            Pos(vec![1., 2., 3.]),
                            Pos(vec![2., 4., 3.])
                        ]),
                        Ring(vec![
                            Pos(vec![3., 2., 3.]),
                            Pos(vec![2., 4., 3.])
                        ])
                    ])
                ]
            }),
    properties: map.to_json()

    };
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"geometry\":{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"},\"properties\":{\"hi\":\"there\"},\"type\":\"Feature\"}", json_string);
}
/// FeatureCollection
///
/// [GeoJSON Format Specification § 2.3](http://geojson.org/geojson-spec.html#feature-collection-objects)
pub struct FeatureCollection {
    features: Vec<Feature>,
}

impl ToJson for FeatureCollection {
    fn to_json(&self) -> json::Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "FeatureCollection".to_json());
        d.insert(format!("features"), self.features.to_json());
        d.to_json()
    }
}
#[test]
fn test_feature_collection_string_tojson() {
    let mut map = BTreeMap::new();
    map.insert(format!("hi"), "there".to_json());
    let point = FeatureCollection {
        features:vec![
          Feature {
            geometry: Geometry::MultiPolygon(MultiPolygon {
                coordinates: vec![
                    Poly(vec![
                        Ring(vec![
                            Pos(vec![1., 2., 3.]),
                            Pos(vec![2., 4., 3.])
                        ]),
                        Ring(vec![
                            Pos(vec![3., 2., 3.]),
                            Pos(vec![2., 4., 3.])
                        ])
                    ])
                ]
            }),
            properties: map.to_json()
        }
    ]};
    let json_string = format!("{}",point.to_json());
    assert_eq!("{\"features\":[{\"geometry\":{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"},\"properties\":{\"hi\":\"there\"},\"type\":\"Feature\"}],\"type\":\"FeatureCollection\"}", json_string);
}

