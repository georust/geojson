#![doc(html_logo_url = "https://raw.githubusercontent.com/georust/meta/master/logo/logo.png")]
// Copyright 2014-2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//!
//! # Introduction
//! The `geojson` crate reads and writes `GeoJSON` ([IETF RFC 7946](https://tools.ietf.org/html/rfc7946)) files,
//! optionally using `serde` for serialisation. Crate users are encouraged to familiarise themselves with the spec,
//! as the crate is structured around it.
//! # Structure of the Crate
//! GeoJSON can contain one of three top-level objects, reflected in the top-level `geojson::GeoJson`
//! enum members of the same name:
//!
//! - [`Feature`](struct.Feature.html)
//! - [`FeatureCollection`](struct.FeatureCollection.html)
//! - [`Geometry`](struct.Geometry.html)
//!
//! With `FeatureCollection` being the most commonly used, since it can contain multiple child objects.
//! A `FeatureCollection` contains `Feature` objects, each of which contains a `Geometry` object, which may be empty.
//! A potentially complicating factor is the `GeometryCollection` geometry type, which can contain
//! one more `Geometry` objects, _including nested `GeometryCollection` objects_.
//! The use of `GeometryCollection` is discouraged, however.
//!
//! If your primary use case for this crate is ingesting `GeoJSON` strings in order to process geometries
//! using the algorithms in the [`geo`](https://docs.rs/geo) crate, you can do so by enabling the `geo-types` feature in
//! your `Cargo.toml`, and using the [`quick_collection`](fn.quick_collection.html) function to
//! parse [`GeoJson`](enum.GeoJson.html) objects into
//! a [`geo_types::GeometryCollection`](../geo_types/struct.GeometryCollection.html).
//! See [here](#conversion-to-geo-objects) for details.
//!
//! This crate uses `serde` for serialization.
//! To get started, add `geojson` to your `Cargo.toml`:
//!
//! ```text
//! [dependencies]
//! geojson= "*"
//! ```
//! # Examples
//! ## Reading
//!
//! ```
//! use geojson::GeoJson;
//!
//! let geojson_str = r#"
//! {
//!   "type": "FeatureCollection",
//!   "features": [
//!     {
//!       "type": "Feature",
//!       "properties": {},
//!       "geometry": {
//!         "type": "Point",
//!         "coordinates": [
//!           -0.13583511114120483,
//!           51.5218870403801
//!         ]
//!       }
//!     }
//!   ]
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
//! ```
//! use serde_json;
//!
//! use serde_json::{Map, to_value};
//!
//! let mut properties = Map::new();
//! properties.insert(
//!     String::from("name"),
//!     to_value("Firestone Grill").unwrap(),
//! );
//! ```
//!
//! `GeoJson` can then be serialized by calling `to_string`:
//!
//! ```rust
//! use geojson::{Feature, GeoJson, Geometry, Value};
//! # fn properties() -> ::serde_json::Map<String, ::serde_json::Value> {
//! # let mut properties = ::serde_json::Map::new();
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
//!     bbox: None,
//!     geometry: Some(geometry),
//!     id: None,
//!     properties: Some(properties),
//!     foreign_members: None
//! });
//!
//! let geojson_string = geojson.to_string();
//! # }
//! ```
//!
//! ## Parsing
//!
//! GeoJSON's [spec](https://tools.ietf.org/html/rfc7946) is quite simple, but
//! it has several subtleties that must be taken into account when parsing it:  
//!
//! - The `geometry` field of a `Feature` is an `Option`
//! - `GeometryCollection`s contain other `Geometry` objects, and can nest.
//!
//! Here's a minimal example which will parse valid GeoJSON without taking
//! ownership of it.
//!
//! ```rust
//! use geojson::{GeoJson, Geometry, Value};
//!
//! /// Process top-level GeoJSON items
//! fn process_geojson(gj: &GeoJson) {
//!     match *gj {
//!         GeoJson::FeatureCollection(ref ctn) => for feature in &ctn.features {
//!             if let Some(ref geom) = feature.geometry {
//!                 match_geometry(geom)
//!             }
//!         },
//!         GeoJson::Feature(ref feature) => {
//!             if let Some(ref geom) = feature.geometry {
//!                 match_geometry(geom)
//!             }
//!         }
//!         GeoJson::Geometry(ref geometry) => match_geometry(geometry),
//!     }
//! }
//!
//! /// Process GeoJSON geometries
//! fn match_geometry(geom: &Geometry) {
//!     match geom.value {
//!         Value::Polygon(_) => println!("Matched a Polygon"),
//!         Value::MultiPolygon(_) => println!("Matched a MultiPolygon"),
//!         Value::GeometryCollection(ref gc) => {
//!             println!("Matched a GeometryCollection");
//!             // GeometryCollections contain other Geometry types, and can nest
//!             // we deal with this by recursively processing each geometry
//!             for geometry in gc {
//!                 match_geometry(geometry)
//!             }
//!         }
//!         // Point, LineString, and their Multi– counterparts
//!         _ => println!("Matched some other geometry"),
//!     }
//! }
//!
//! fn main() {
//!     let geojson_str = r#"
//!     {
//!       "type": "GeometryCollection",
//!       "geometries": [
//!         {"type": "Point", "coordinates": [0,1]},
//!         {"type": "MultiPoint", "coordinates": [[-1,0],[1,0]]},
//!         {"type": "LineString", "coordinates": [[-1,-1],[1,-1]]},
//!         {"type": "MultiLineString", "coordinates": [
//!           [[-2,-2],[2,-2]],
//!           [[-3,-3],[3,-3]]
//!         ]},
//!         {"type": "Polygon", "coordinates": [
//!           [[-5,-5],[5,-5],[0,5],[-5,-5]],
//!           [[-4,-4],[4,-4],[0,4],[-4,-4]]
//!         ]},
//!         { "type": "MultiPolygon", "coordinates": [[
//!           [[-7,-7],[7,-7],[0,7],[-7,-7]],
//!           [[-6,-6],[6,-6],[0,6],[-6,-6]]
//!         ],[
//!           [[-9,-9],[9,-9],[0,9],[-9,-9]],
//!           [[-8,-8],[8,-8],[0,8],[-8,-8]]]
//!         ]},
//!         {"type": "GeometryCollection", "geometries": [
//!           {"type": "Polygon", "coordinates": [
//!             [[-5.5,-5.5],[5,-5],[0,5],[-5,-5]],
//!             [[-4,-4],[4,-4],[0,4],[-4.5,-4.5]]
//!           ]}
//!         ]}
//!       ]
//!     }
//!     "#;
//!     let geojson = geojson_str.parse::<GeoJson>().unwrap();
//!     process_geojson(&geojson);
//! }
//! ```
//!
//! ## Conversion to Geo objects
//!
//! The [`TryFrom`](../std/convert/trait.TryFrom.html) trait provides
//! **fallible** conversions _to_ [Geo](../geo_types/index.html#structs) types from GeoJSON [`Value`](enum.Value.html) enums,
//! allowing them to be measured or used in calculations. Conversely, `From` is
//! implemented on the [`Value`](enum.Value.html) enum variants to allow conversion _from_ `Geo` types.
//!
//! **In most cases it is assumed that you want to convert GeoJSON into `geo` primitive types in order to process, transform, or measure them:**  
//! - `match` on `geojson`, iterating over its `features` field, yielding `Option<Feature>`.
//! - process each `Feature`, accessing its `Value` field, yielding `Option<Value>`.
//!
//! Each [`Value`](enum.Value.html) represents a primitive type, such as a
//! coordinate, point, linestring, polygon, or its multi- equivalent, **and each of these has
//! an equivalent `geo` primitive type**, which you can convert to using the `std::convert::TryFrom` trait.
//!
//! Unifying these features, the [`quick_collection`](fn.quick_collection.html) function accepts a [`GeoJson`](enum.GeoJson.html) enum
//! and processes it, producing a [`GeometryCollection`](../geo_types/struct.GeometryCollection.html)
//! whose members can be transformed, measured, rotated, etc using the algorithms and functions in
//! the [`geo`](https://docs.rs/geo) crate:
//!
//! ```
//! # #[cfg(feature = "geo-types")]
//! use geojson::{GeoJson, quick_collection};
//! # #[cfg(feature = "geo-types")]
//! use geo_types::GeometryCollection;
//! # #[cfg(feature = "geo-types")]
//! let geojson_str = r#"
//! {
//!   "type": "FeatureCollection",
//!   "features": [
//!     {
//!       "type": "Feature",
//!       "properties": {},
//!       "geometry": {
//!         "type": "Point",
//!         "coordinates": [
//!           -0.13583511114120483,
//!           51.5218870403801
//!         ]
//!       }
//!     }
//!   ]
//! }
//! "#;
//! # #[cfg(feature = "geo-types")]
//! let geojson = geojson_str.parse::<GeoJson>().unwrap();
//! // Turn the GeoJSON string into a geo_types GeometryCollection
//! # #[cfg(feature = "geo-types")]
//! let mut collection: GeometryCollection<f64> = quick_collection(&geojson).unwrap();
//! ```
//! ### Caveats
//! - Round-tripping with intermediate processing using the `geo` types may not produce identical output,
//! as e.g. outer `Polygon` rings are automatically closed.
//! - `geojson` attempts to output valid geometries. In particular, it may re-orient `Polygon` rings when serialising.
//!
//! The [`geojson_example`](https://github.com/urschrei/geojson_example) and
//! [`polylabel_cmd`](https://github.com/urschrei/polylabel_cmd/blob/master/src/main.rs) crates contain example
//! implementations which may be useful if you wish to perform this kind of processing yourself and require
//! more granular control over performance and / or memory allocation.

#[cfg(feature = "geo-types")]
use geo_types;
use serde;
use serde_json;

/// Bounding Boxes
///
/// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
pub type Bbox = Vec<f64>;

mod util;

mod position;
pub use position::Position;

mod geojson;
pub use crate::geojson::GeoJson;

mod geometry;
pub use crate::geometry::{Geometry, Value};

pub mod feature;

mod feature_collection;
pub use crate::feature_collection::FeatureCollection;

#[cfg(feature = "geo-types")]
mod conversion;

#[cfg(feature = "geo-types")]
pub use conversion::quick_collection;

/// Feature Objects
///
/// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
#[derive(Clone, Debug, PartialEq)]
pub struct Feature<Pos> {
    /// Bounding Box
    ///
    /// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
    pub bbox: Option<Bbox>,
    /// Geometry
    ///
    /// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
    pub geometry: Option<Geometry<Pos>>,
    /// Identifier
    ///
    /// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
    pub id: Option<feature::Id>,
    /// Properties
    ///
    /// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
    pub properties: Option<json::JsonObject>,
    /// Foreign Members
    ///
    /// [GeoJSON Format Specification § 6](https://tools.ietf.org/html/rfc7946#section-6)
    pub foreign_members: Option<json::JsonObject>,
}

/// Error when reading a GeoJSON object from a str or Object
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    BboxExpectedArray,
    BboxExpectedNumericValues,
    GeoJsonExpectedObject,
    GeoJsonUnknownType,
    GeometryUnknownType,
    MalformedJson,
    PropertiesExpectedObjectOrNull,
    FeatureInvalidGeometryValue,
    FeatureInvalidIdentifierType,
    ExpectedType { expected: String, actual: String },

    // FIXME: make these types more specific
    ExpectedStringValue,
    ExpectedProperty(String),
    ExpectedF64Value,
    ExpectedArrayValue,
    ExpectedObjectValue,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::BboxExpectedArray =>
            // FIXME: inform what type we actually found
            {
                write!(f, "Encountered non-array type for a 'bbox' object.")
            }
            Error::BboxExpectedNumericValues =>
            // FIXME: inform what type we actually found
            {
                write!(f, "Encountered non-numeric value within 'bbox' array.")
            }
            Error::GeoJsonExpectedObject =>
            // FIXME: inform what type we actually found
            {
                write!(f, "Encountered non-object type for GeoJSON.")
            }
            Error::GeoJsonUnknownType =>
            // FIXME: inform what type we actually found
            {
                write!(f, "Encountered unknown GeoJSON object type.")
            }
            Error::GeometryUnknownType => write!(f, "Encountered unknown 'geometry' object type."),
            Error::MalformedJson =>
            // FIXME: can we report specific serialization error?
            {
                write!(f, "Encountered malformed JSON.")
            }
            Error::PropertiesExpectedObjectOrNull =>
            // FIXME: inform what type we actually found
            {
                write!(
                    f,
                    "Encountered neither object type nor null type for \
                     'properties' object."
                )
            }
            Error::FeatureInvalidGeometryValue =>
            // FIXME: inform what type we actually found
            {
                write!(
                    f,
                    "Encountered neither object type nor null type for \
                     'geometry' field on 'feature' object."
                )
            }
            Error::FeatureInvalidIdentifierType =>
            // FIXME: inform what type we actually found
            {
                write!(
                    f,
                    "Encountered neither number type nor string type for \
                     'id' field on 'feature' object."
                )
            }
            Error::ExpectedType {
                ref expected,
                ref actual,
            } => write!(
                f,
                "Expected GeoJSON type '{}', found '{}'",
                expected, actual,
            ),
            Error::ExpectedStringValue => write!(f, "Expected a string value."),
            Error::ExpectedProperty(ref prop_name) => {
                write!(f, "Expected GeoJSON property '{}'.", prop_name)
            }
            Error::ExpectedF64Value => write!(f, "Expected a floating-point value."),
            Error::ExpectedArrayValue => write!(f, "Expected an array."),
            Error::ExpectedObjectValue => write!(f, "Expected an object."),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BboxExpectedArray => "non-array 'bbox' type",
            Error::BboxExpectedNumericValues => "non-numeric 'bbox' array",
            Error::GeoJsonExpectedObject => "non-object GeoJSON type",
            Error::GeoJsonUnknownType => "unknown GeoJSON object type",
            Error::GeometryUnknownType => "unknown 'geometry' object type",
            Error::MalformedJson => "malformed JSON",
            Error::PropertiesExpectedObjectOrNull => {
                "neither object type nor null type for properties' object."
            }
            Error::FeatureInvalidGeometryValue => {
                "neither object type nor null type for 'geometry' field on 'feature' object."
            }
            Error::FeatureInvalidIdentifierType => {
                "neither number type nor string type for 'id' field on 'feature' object."
            }
            Error::ExpectedType { .. } => "mismatched GeoJSON type",
            Error::ExpectedStringValue => "expected a string value",
            Error::ExpectedProperty(..) => "expected a GeoJSON property",
            Error::ExpectedF64Value => "expected a floating-point value",
            Error::ExpectedArrayValue => "expected an array",
            Error::ExpectedObjectValue => "expected an object",
        }
    }
}

mod json {
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub use serde_json::{Map, Value as JsonValue};
    pub type JsonObject = Map<String, JsonValue>;
}
