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
//!
//! This crate helps you read and write [GeoJSON](https://geojson.org) — a format for encoding
//! geographic data structures.
//!
//! To get started, add `geojson` to your `Cargo.toml`.
//!
//! ```sh
//! cargo add geojson
//! ```
//!
//! # Types and crate structure
//!
//! This crate is structured around the GeoJSON spec ([IETF RFC 7946](https://tools.ietf.org/html/rfc7946)),
//! and users are encouraged to familiarise themselves with it. The elements specified in this spec
//! have corresponding struct and type definitions in this crate, e.g. [`FeatureCollection`], [`Feature`],
//! etc.
//!
//! There are two primary ways to use this crate.
//!
//! The first, most general, approach is to write your code to deal in terms of these structs from
//! the GeoJSON spec. This allows you to access the full expressive power of GeoJSON with the speed
//! and safety of Rust.
//!
//! Alternatively, and commonly, if you only need geometry and properties (and not, e.g.
//! [foreign members](https://www.rfc-editor.org/rfc/rfc7946#section-6.1)), you can bring your own
//! types, and use this crate's [`serde`] integration to serialize and deserialize your custom
//! types directly to and from a GeoJSON Feature Collection. [See more on using your own types with
//! serde](#using-your-own-types-with-serde).
//!
//! If you want to use GeoJSON as input to or output from a geometry processing crate like
//! [`geo`](https://docs.rs/geo), see the section on [using geojson with
//! geo-types](#use-geojson-with-other-crates-by-converting-to-geo-types).
//!
//! ## Using structs from the GeoJSON spec
//!
//! A GeoJSON object can be one of three top-level objects, reflected in this crate as the
//! [`GeoJson`] enum members of the same name.
//!
//! 1. A [`Geometry`] represents points, curves, and surfaces in coordinate space.
//! 2. A [`Feature`] usually contains a `Geometry` and some associated data, for example a "name"
//!    field or any other properties you'd like associated with the `Geometry`.
//! 3. A [`FeatureCollection`] is a list of one or more `Feature`s.
//!
//! Because [`Feature`] and [`FeatureCollection`] are more flexible, bare [`Geometry`] GeoJSON
//! documents are rarely encountered in the wild. As such, conversions from [`Geometry`]
//! or [Geometry `Value`](Value) to [`Feature`] objects are provided via the [`From`] trait.
//!
//! *Beware:* A common point of confusion arises when converting a [GeoJson
//! `GeometryCollection`](Value::GeometryCollection). Do you want it converted to a single
//! [`Feature`] whose geometry is a [`GeometryCollection`](Value::GeometryCollection), or do you
//! want a [`FeatureCollection`] with each *element* of the
//! [`GeometryCollection`](Value::GeometryCollection) converted to its own [`Feature`], potentially
//! with their own individual properties. Either is possible, but it's important you understand
//! which one you want.
//!
//! # Examples
//! ## Reading
//!
//! [`GeoJson`] can be deserialized by calling [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse):
//!
//! ```
//! use geojson::{Feature, GeoJson, Geometry, Value};
//! use std::convert::TryFrom;
//!
//! let geojson_str = r#"
//! {
//!   "type": "Feature",
//!   "properties": { "food": "donuts" },
//!   "geometry": {
//!     "type": "Point",
//!     "coordinates": [ -118.2836, 34.0956 ]
//!   }
//! }
//! "#;
//!
//! let geojson: GeoJson = geojson_str.parse::<GeoJson>().unwrap();
//! let feature: Feature = Feature::try_from(geojson).unwrap();
//!
//! // read property data
//! assert_eq!("donuts", feature.property("food").unwrap());
//!
//! // read geometry data
//! let geometry: Geometry = feature.geometry.unwrap();
//! if let Value::Point(coords) = geometry.value {
//!     assert_eq!(coords, vec![-118.2836, 34.0956]);
//! }
//!
//! # else {
//! #    unreachable!("should be point");
//! # }
//! ```
//!
//! ## Writing
//!
//! `GeoJson` can be serialized by calling [`to_string`](geojson/enum.GeoJson.html#impl-ToString):
//!
//! ```rust
//! use geojson::{Feature, GeoJson, Geometry, Value};
//! # fn get_properties() -> ::geojson::JsonObject {
//! # let mut properties = ::geojson::JsonObject::new();
//! # properties.insert(
//! #     String::from("name"),
//! #     ::geojson::JsonValue::from("Firestone Grill"),
//! # );
//! # properties
//! # }
//! # fn main() {
//!
//! let geometry = Geometry::new(Value::Point(vec![-120.66029, 35.2812]));
//!
//! let geojson = GeoJson::Feature(Feature {
//!     bbox: None,
//!     geometry: Some(geometry),
//!     id: None,
//!     // See the next section about Feature properties
//!     properties: Some(get_properties()),
//!     foreign_members: None,
//! });
//!
//! let geojson_string = geojson.to_string();
//! # }
//! ```
//!
//! ### Feature properties
//!
//! The `geojson` crate is built on top of [`serde_json`](../serde_json/index.html). Consequently,
//! some fields like [`feature.properties`](Feature#structfield.properties) hold [serde_json
//! values](../serde_json/value/index.html).
//!
//! ```
//! use geojson::{JsonObject, JsonValue};
//!
//! let mut properties = JsonObject::new();
//! let key = "name".to_string();
//! properties.insert(key, JsonValue::from("Firestone Grill"));
//! ```
//!
//! ## Parsing
//!
//! GeoJSON's [spec](https://tools.ietf.org/html/rfc7946) is quite simple, but
//! it has several subtleties that must be taken into account when parsing it:
//!
//! - The `geometry` field of a [`Feature`] is an [`Option`] — it can be blank.
//! - [`GeometryCollection`](Value::GeometryCollection)s contain other [`Geometry`] objects, and can nest.
//! - We strive to produce strictly valid output, but we are more permissive about what we accept
//!   as input.
//!
//! Here's a minimal example which will parse and process a GeoJSON string.
//!
//! ```rust
//! use geojson::{GeoJson, Geometry, Value};
//!
//! /// Process top-level GeoJSON Object
//! fn process_geojson(gj: &GeoJson) {
//!     match *gj {
//!         GeoJson::FeatureCollection(ref ctn) => {
//!             for feature in &ctn.features {
//!                 if let Some(ref geom) = feature.geometry {
//!                     match_geometry(geom)
//!                 }
//!             }
//!         }
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
//!             // !!! GeometryCollections contain other Geometry types, and can
//!             // nest — we deal with this by recursively processing each geometry
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
//! ## Use geojson with other crates by converting to geo-types
//!
//! [`geo-types`](../geo_types/index.html#structs) are a common geometry format used across many
//! geospatial processing crates. The `geo-types` feature is enabled by default.
//!
//! ### Convert `geo-types` to `geojson`
//!
//! [`From`] is implemented on the [`Value`] enum variants to allow conversion _from_ [`geo-types`
//! Geometries](../geo_types/index.html#structs).
//!
//! ```
//! # #[cfg(feature = "geo-types")]
//! # {
//! // requires enabling the `geo-types` feature
//! let geo_point: geo_types::Point<f64> = geo_types::Point::new(2., 9.);
//! let geo_geometry: geo_types::Geometry<f64> = geo_types::Geometry::from(geo_point);
//!
//! assert_eq!(
//!     geojson::Value::from(&geo_point),
//!     geojson::Value::Point(vec![2., 9.]),
//! );
//! assert_eq!(
//!     geojson::Value::from(&geo_geometry),
//!     geojson::Value::Point(vec![2., 9.]),
//! );
//! # }
//! ```
//!
//! If you wish to produce a [`FeatureCollection`] from a homogenous collection of `geo-types`, a
//! `From` impl is provided for `geo_types::GeometryCollection`:
//!
//! ```rust
//! # #[cfg(feature = "geo-types")]
//! # {
//! // requires enabling the `geo-types` feature
//! use geojson::FeatureCollection;
//! use geo_types::{polygon, point, Geometry, GeometryCollection};
//! use std::iter::FromIterator;
//!
//! let poly: Geometry<f64> = polygon![
//!     (x: -111., y: 45.),
//!     (x: -111., y: 41.),
//!     (x: -104., y: 41.),
//!     (x: -104., y: 45.),
//! ].into();
//!
//! let point: Geometry<f64> = point!(x: 1.0, y: 2.0).into();
//!
//! let geometry_collection = GeometryCollection::from_iter(vec![poly, point]);
//! let feature_collection = FeatureCollection::from(&geometry_collection);
//!
//! assert_eq!(2, feature_collection.features.len());
//! # }
//! ```
//!
//! ### Convert `geojson` to `geo-types`
//!
//! The `geo-types` feature implements the [`TryFrom`](../std/convert/trait.TryFrom.html) trait,
//! providing **fallible** conversions _to_ [geo-types Geometries](../geo_types/index.html#structs)
//! from [`GeoJson`], [`Value`], or [`Geometry`] types.
//!
//! #### Convert `geojson` to `geo_types::Geometry<f64>`
//!
//! ```
//! # #[cfg(feature = "geo-types")]
//! # {
//! // This example requires the `geo-types` feature
//! use geo_types::Geometry;
//! use geojson::GeoJson;
//! use std::convert::TryFrom;
//! use std::str::FromStr;
//!
//! let geojson_str = r#"
//! {
//!  "type": "Feature",
//!  "properties": {},
//!  "geometry": {
//!    "type": "Point",
//!    "coordinates": [
//!      -0.13583511114120483,
//!      51.5218870403801
//!    ]
//!  }
//! }
//! "#;
//! let geojson = GeoJson::from_str(geojson_str).unwrap();
//! // Turn the GeoJSON string into a geo_types Geometry
//! let geom = geo_types::Geometry::<f64>::try_from(geojson).unwrap();
//! # }
//! ```
//!
//! ### Caveats
//! - Round-tripping with intermediate processing using the `geo` types may not produce identical output,
//! as e.g. outer `Polygon` rings are automatically closed.
//! - `geojson` attempts to output valid geometries. In particular, it may re-orient `Polygon` rings when serialising.
//!
//! The [`geojson_example`](https://github.com/urschrei/geojson_example) and
//! [`polylabel_cmd`](https://github.com/urschrei/polylabel_cmd/blob/master/src/main.rs) crates contain example
//! implementations which may be useful if you wish to perform this kind of processing yourself and require
//! more granular control over performance and / or memory allocation.
//!
//! ## Using your own types with serde
//!
//! If your use case is simple enough, you can read and write GeoJSON directly to and from your own
//! types using serde.
//!
//! Specifically, the requirements are:
//! 1. Your type has a `geometry` field.
//!     1. If your `geometry` field is a [`geo-types` Geometry](geo_types::geometry), you must use
//!         the provided `serialize_with`/`deserialize_with` helpers.
//!     2. Otherwise, your `geometry` field must be a [`crate::Geometry`].
//! 2. Other than `geometry`, you may only use a Feature's `properties` - all other fields, like
//!    foreign members, will be lost.
//!
//! ```ignore
//! #[derive(serde::Serialize, serde::Deserialize)]
//! struct MyStruct {
//!     // Serialize as geojson, rather than using the type's default serialization
//!     #[serde(serialize_with = "serialize_geometry", deserialize_with = "deserialize_geometry")]
//!     geometry: geo_types::Point<f64>,
//!     name: String,
//!     count: u64,
//! }
//! ```
//!
//! See more in the [serialization](ser) and [deserialization](de) modules.
// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Bounding Boxes
///
/// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
pub type Bbox = Vec<f64>;

/// Positions
///
/// [GeoJSON Format Specification § 3.1.1](https://tools.ietf.org/html/rfc7946#section-3.1.1)
pub type Position = Vec<f64>;

pub type PointType = Position;
pub type LineStringType = Vec<Position>;
pub type PolygonType = Vec<Vec<Position>>;

mod util;

mod geojson;
pub use crate::geojson::GeoJson;

mod geometry;
pub use crate::geometry::{Geometry, Value};

pub mod feature;

mod feature_collection;
pub use crate::feature_collection::FeatureCollection;

mod feature_iterator;
#[allow(deprecated)]
#[doc(hidden)]
pub use crate::feature_iterator::FeatureIterator;

pub mod errors;
pub use crate::errors::{Error, Result};

#[cfg(feature = "geo-types")]
mod conversion;

/// Build your struct from GeoJSON using [`serde`]
pub mod de;

/// Write your struct to GeoJSON using [`serde`]
pub mod ser;

mod feature_reader;
pub use feature_reader::FeatureReader;

mod feature_writer;
pub use feature_writer::FeatureWriter;

#[cfg(feature = "geo-types")]
pub use conversion::quick_collection;

/// Feature Objects
///
/// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Feature {
    /// Bounding Box
    ///
    /// [GeoJSON Format Specification § 5](https://tools.ietf.org/html/rfc7946#section-5)
    pub bbox: Option<Bbox>,
    /// Geometry
    ///
    /// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
    pub geometry: Option<Geometry>,
    /// Identifier
    ///
    /// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
    pub id: Option<feature::Id>,
    /// Properties
    ///
    /// [GeoJSON Format Specification § 3.2](https://tools.ietf.org/html/rfc7946#section-3.2)
    ///
    /// NOTE: This crate will permissively parse a Feature whose json is missing a `properties` key.
    /// Because the spec implies that the `properties` key must be present, we will always include
    /// the `properties` key when serializing.
    pub properties: Option<JsonObject>,
    /// Foreign Members
    ///
    /// [GeoJSON Format Specification § 6](https://tools.ietf.org/html/rfc7946#section-6)
    pub foreign_members: Option<JsonObject>,
}

pub type JsonValue = serde_json::Value;
pub type JsonObject = serde_json::Map<String, JsonValue>;
