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

//! # Examples
//!
//! ## Reading
//!
//! ```
//! use geojson::GeoJson;
//!
//! let geojson_str = r#"
//! {
//!     "type": "Feature",
//!     "properties": {
//!         "name": "Pleasantville Station"
//!     },
//!     "geometry": {
//!         "type": "Point",
//!         "coordinates": [73.7922, 41.1342]
//!     }
//! }
//! "#;
//!
//! let geojson = geojson_str.parse::<GeoJson>().unwrap();
//! ```
//!
//! ## Writing
//!
//! ```norun
//! use std::collections::HashMap;
//! use rustc_serialize::json::ToJson;
//! use geojson::{Feature, GeoJson, Geometry, Value};
//!
//! let geometry = Geometry::new(
//!     Value::Point(vec![73.7922, 41.1342])
//! );
//!
//! let mut properties = HashMap::new();
//! properties.insert(
//!     String::from("name"),
//!     "Pleasantville Station".to_json(),
//! );
//!
//! let geojson = GeoJson::Feature(Feature {
//!     crs: None,
//!     bbox: None,
//!     geometry: geometry,
//!     id: None,
//!     properties: Some(properties),
//! });
//!
//! let geojson_string = geojson.to_string();
//! ```

extern crate rustc_serialize;

use rustc_serialize::json;

/// Bounding Boxes
///
/// [GeoJSON Format Specification § 4]
/// (http://geojson.org/geojson-spec.html#bounding-boxes)
pub type Bbox = Vec<f64>;

/// Positions
///
/// [GeoJSON Format Specification § 2.1.1]
/// (http://geojson.org/geojson-spec.html#positions)
pub type Position = Vec<f64>;

pub type PointType = Position;
pub type LineStringType = Vec<Position>;
pub type PolygonType = Vec<Vec<Position>>;

#[macro_use]
mod macros;

mod util;

mod crs;
pub use crs::Crs;

mod geojson;
pub use geojson::GeoJson;

mod geometry;
pub use geometry::{Geometry, Value};

mod feature;
pub use feature::Feature;

mod feature_collection;
pub use feature_collection::FeatureCollection;

/// Error when reading a GeoJSON object from a str or Object
#[derive(Debug)]
pub struct Error {
    pub desc: &'static str,
}

impl Error {
    pub fn new(desc: &'static str) -> Error {
        return Error{desc: desc};
    }
}

trait FromObject: Sized {
    fn from_object(object: &json::Object) -> Result<Self, Error>;
}
