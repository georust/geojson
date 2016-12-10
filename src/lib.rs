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
//! Both `rustc_serialize` (default) and `serde` are supported.
//! To use `serde`, add the `with-serde` feature to your Cargo.toml:
//!
//! ```text
//! [dependencies]
//! geojson={version="*", features=["with-serde"]}
//! ```
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
//!         "name": "Firestone Grill"
//!     },
//!     "geometry": {
//!         "type": "Point",
//!         "coordinates": [-120.66029,35.2812]
//!     }
//! }
//! "#;
//!
//! let geojson = geojson_str.parse::<GeoJson>().unwrap();
//! ```
//!
//! ## Writing
//!
//! Writing `geojson` depends on the serialization framework because some structs
//! (like `Feature`) use json values for properties.
//!
//! For `rustc_serialize` use `rustc_serialize::json::Object`:
//!
//! ```
//! # #[cfg(not(feature = "with-serde"))]
//! # extern crate rustc_serialize;
//! # #[cfg(not(feature = "with-serde"))]
//! # fn main () {
//! use rustc_serialize::json::ToJson;
//! use std::collections::BTreeMap;
//!
//! let mut properties = BTreeMap::new();
//! properties.insert(
//!     String::from("name"),
//!     "Firestone Grill".to_json(),
//! );
//! # }
//! #
//! # #[cfg(feature = "with-serde")]
//! # fn main() {}
//! ```
//!
//! For `serde` use `serde_json::Value::Object`:
//!
//! ```
//! # #[cfg(feature = "with-serde")]
//! # extern crate serde_json;
//! # #[cfg(feature = "with-serde")]
//! # fn main () {
//! use serde_json::to_value;
//! use std::collections::BTreeMap;
//!
//! let mut properties = BTreeMap::new();
//! properties.insert(
//!     String::from("name"),
//!     to_value("Firestone Grill"),
//! );
//! # }
//! #
//! # #[cfg(not(feature = "with-serde"))]
//! # fn main() {}
//! ```
//!
//! `GeoJson` can then be serialized by calling `to_string`:
//!
//! ```rust
//! # #[cfg(not(feature = "with-serde"))]
//! # extern crate rustc_serialize;
//! # #[cfg(not(feature = "with-serde"))]
//! use rustc_serialize::json::ToJson;
//! # #[cfg(feature = "with-serde")]
//! # extern crate serde_json;
//! # extern crate geojson;
//! use std::collections::BTreeMap;
//! use geojson::{Feature, GeoJson, Geometry, Value};
//! # #[cfg(not(feature = "with-serde"))]
//! # fn properties() -> ::rustc_serialize::json::Object {
//! # let mut properties = std::collections::BTreeMap::new();
//! # properties.insert(
//! #     String::from("name"),
//! #     "Firestone Grill".to_json(),
//! # );
//! # properties
//! # }
//! # #[cfg(feature = "with-serde")]
//! # fn properties() -> ::std::collections::BTreeMap<String, ::serde_json::Value> {
//! # let mut properties = std::collections::BTreeMap::new();
//! # properties.insert(
//! #     String::from("name"),
//! #     ::serde_json::Value::String(String::from("Firestone Grill")),
//! # );
//! # properties
//! # }
//! # fn main() {
//! # let properties = properties();
//!
//! let geometry = Geometry::new(
//!     Value::Point(vec![-120.66029,35.2812])
//! );
//!
//! let geojson = GeoJson::Feature(Feature {
//!     crs: None,
//!     bbox: None,
//!     geometry: Some(geometry),
//!     id: None,
//!     properties: Some(properties),
//! });
//!
//! let geojson_string = geojson.to_string();
//! # }
//! ```

#[cfg(not(feature = "with-serde"))]
include!("lib.rustc_serialize.rs.in");

#[cfg(feature = "with-serde")]
include!("lib.serde.rs.in");

extern crate geo;
extern crate num;

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

/// Convert geo::types to geometry::Geometry
#[doc(hidden)]
pub mod conversion;

/// Error when reading a GeoJSON object from a str or Object
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    BboxExpectedArray,
    BboxExpectedNumericValues,
    CrsExpectedObject,
    CrsUnknownType(String),
    GeoJsonExpectedObject,
    GeoJsonUnknownType,
    GeometryUnknownType,
    MalformedJson,
    PropertiesExpectedObjectOrNull,
    FeatureInvalidGeometryValue,

    // FIXME: make these types more specific
    ExpectedStringValue,
    ExpectedProperty,
    ExpectedF64Value,
    ExpectedArrayValue,
    ExpectedObjectValue,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::BboxExpectedArray =>
                // FIXME: inform what type we actually found
                write!(f, "Encountered non-array type for a 'bbox' object."),
            Error::BboxExpectedNumericValues =>
                // FIXME: inform what type we actually found
                write!(f, "Encountered non-numeric value within 'bbox' array."),
            Error::CrsExpectedObject =>
                // FIXME: inform what type we actually found
                write!(f, "Encountered non-object type for a 'crs' object."),
            Error::CrsUnknownType(ref t) =>
                write!(f, "Encountered unknown type '{}' for a 'crs' object.", t),
            Error::GeoJsonExpectedObject =>
                // FIXME: inform what type we actually found
                write!(f, "Encountered non-object type for GeoJSON."),
            Error::GeoJsonUnknownType =>
                // FIXME: inform what type we actually found
                write!(f, "Encountered unknown GeoJSON object type."),
            Error::GeometryUnknownType =>
                write!(f, "Encountered unknown 'geometry' object type."),
            Error::MalformedJson =>
                // FIXME: can we report specific serialization error?
                write!(f, "Encountered malformed JSON."),
            Error::PropertiesExpectedObjectOrNull =>
                // FIXME: inform what type we actually found
                write!(f, "Encountered neither object type nor null type for \
                           'properties' object."),
            Error::FeatureInvalidGeometryValue =>
                // FIXME: inform what type we actually found
                write!(f, "Encountered neither object type nor null type for \
                           'geometry' field on 'feature' object."),
            Error::ExpectedStringValue =>
                write!(f, "Expected a string value."),
            Error::ExpectedProperty =>
                write!(f, "Expected a GeoJSON 'property'."),
            Error::ExpectedF64Value =>
                write!(f, "Expected a floating-point value."),
            Error::ExpectedArrayValue =>
                write!(f, "Expected an array."),
            Error::ExpectedObjectValue =>
                write!(f, "Expected an object."),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BboxExpectedArray => "non-array 'bbox' type",
            Error::BboxExpectedNumericValues => "non-numeric 'bbox' array",
            Error::CrsExpectedObject => "non-object 'crs' type",
            Error::CrsUnknownType(..) => "unknown 'crs' type",
            Error::GeoJsonExpectedObject => "non-object GeoJSON type",
            Error::GeoJsonUnknownType => "unknown GeoJSON object type",
            Error::GeometryUnknownType => "unknown 'geometry' object type",
            Error::MalformedJson => "malformed JSON",
            Error::PropertiesExpectedObjectOrNull =>
                "neither object type nor null type for properties' object.",
            Error::FeatureInvalidGeometryValue =>
                "neither object type nor null type for 'geometry' field on 'feature' object.",
            Error::ExpectedStringValue => "expected a string value",
            Error::ExpectedProperty => "expected a GeoJSON 'property'",
            Error::ExpectedF64Value => "expected a floating-point value",
            Error::ExpectedArrayValue => "expected an array",
            Error::ExpectedObjectValue => "expected an object",
        }
    }
}

trait FromObject: Sized {
    fn from_object(object: &json::JsonObject) -> Result<Self, Error>;
}
