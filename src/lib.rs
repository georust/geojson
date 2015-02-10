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

#![feature(core)]

extern crate "rustc-serialize" as rustc_serialize;

pub use types::pos::Pos;

pub use types::ring::Ring;

pub use types::point::Point;
pub use types::multipoint::MultiPoint;

pub use types::linestring::LineString;
pub use types::multilinestring::MultiLineString;

pub use types::poly::Poly;
pub use types::polygon::Polygon;
pub use types::multipolygon::MultiPolygon;

pub use types::geometry::Geometry;
pub use types::geometrycollection::GeometryCollection;

pub use types::feature::Feature;
pub use types::featurecollection::FeatureCollection;

macro_rules! expect_string {
    ($value:expr) => (try!(
        match $value.as_string() {
            Some(v) => Ok(v),
            None => Err({use GeoJsonError; GeoJsonError::new("Expected string value")})
        }
    ))
}

macro_rules! expect_f64 {
    ($value:expr) => (try!(
        match $value.as_f64() {
            Some(v) => Ok(v),
            None => Err({use GeoJsonError; GeoJsonError::new("Expected f64 value")})
        }
    ))
}

macro_rules! expect_array {
    ($value:expr) => (try!(
        match $value.as_array() {
            Some(v) => Ok(v),
            None => Err({use GeoJsonError; GeoJsonError::new("Expected array value")})
        }
    ))
}

macro_rules! expect_object {
    ($value:expr) => (try!(
        match $value.as_object() {
            Some(v) => Ok(v),
            None => Err({use GeoJsonError; GeoJsonError::new("Expected object value")})
        }
    ))
}

macro_rules! expect_property {
    ($obj:expr, $name:expr, $desc:expr) => (
        match $obj.get($name) {
            Some(v) => v,
            None => return Err({use GeoJsonError; GeoJsonError::new($desc)}),
        };
    )
}

mod types;

#[derive(Copy)]
pub struct GeoJsonError {
    pub desc: &'static str,
}

impl GeoJsonError {
    pub fn new(desc: &'static str) -> GeoJsonError {
        return GeoJsonError{desc: desc};
    }
}

pub type GeoJsonResult<T> = Result<T, GeoJsonError>;
