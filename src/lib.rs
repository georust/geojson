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
pub use types::featurecollection::from_str;

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
