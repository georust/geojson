//!
//! To output your struct to GeoJSON, either as a String, bytes, or to a file, your type *must*
//! implement or derive [`serde::Serialize`]:
//!
//! ```rust, ignore
//! #[derive(serde::Serialize)]
//! struct MyStruct {
//!     ...
//! }
//! ```
//!
//! Your type *must* have a field called `geometry` and it must be `serialized_with` [`serialize_geometry`](crate::ser::serialize_geometry):
//!  ```rust, ignore
//! #[derive(serde::Serialize)]
//! struct MyStruct {
//!     #[serde(serialize_with = "geojson::ser::serialize_geometry")]
//!     geometry: geo_types::Point<f64>,
//!     ...
//! }
//! ```
//!
//! All fields in your struct other than `geometry` will be serialized as `properties` of the
//! GeoJSON Feature.
//!
//! # Examples
#![cfg_attr(feature = "geo-types", doc = "```")]
#![cfg_attr(not(feature = "geo-types"), doc = "```ignore")]
//! use serde::Serialize;
//! use geojson::ser::serialize_geometry;
//!
//! #[derive(Serialize)]
//! struct MyStruct {
//!     // Serialize as geojson, rather than using the type's default serialization
//!     #[serde(serialize_with = "serialize_geometry")]
//!     geometry: geo_types::Point<f64>,
//!     name: String,
//!     population: u64
//! }
//!
//! let my_structs = vec![
//!     MyStruct {
//!         geometry: geo_types::Point::new(11.1, 22.2),
//!         name: "Downtown".to_string(),
//!         population: 123
//!     },
//!     MyStruct {
//!         geometry: geo_types::Point::new(33.3, 44.4),
//!         name: "Uptown".to_string(),
//!         population: 456
//!     }
//! ];
//!
//! let output_geojson = geojson::ser::to_feature_collection_string(&my_structs).unwrap();
//!
//! let expected_geojson = serde_json::json!(
//!     {
//!         "type":"FeatureCollection",
//!         "features": [
//!             {
//!                 "type": "Feature",
//!                 "geometry": { "coordinates": [11.1,22.2], "type": "Point" },
//!                 "properties": {
//!                     "name": "Downtown",
//!                     "population": 123
//!                 }
//!             },
//!             {
//!                 "type": "Feature",
//!                 "geometry": { "coordinates": [33.3, 44.4], "type": "Point" },
//!                 "properties": {
//!                     "name": "Uptown",
//!                     "population": 456
//!                 }
//!             }
//!         ]
//!     }
//! );
//! #
//! # // re-parse the json to do a structural comparison, rather than worry about formatting
//! # // or other meaningless deviations in an exact String comparison.
//! # let output_geojson: serde_json::Value = serde_json::from_str(&output_geojson).unwrap();
//! #
//! # assert_eq!(output_geojson, expected_geojson);
//! ```
//!
//! # Reading *and* Writing GeoJSON
//!
//! This module is only concerned with Writing out GeoJSON. If you'd also like to reading GeoJSON,
//! you'll want to combine this with the functionality from the [`crate::de`] module:
//! ```ignore
//! #[derive(serde::Serialize, serde::Deserialize)]
//! struct MyStruct {
//!     // Serialize as geojson, rather than using the type's default serialization
//!     #[serde(serialize_with = "serialize_geometry", deserialize_with = "deserialize_geometry")]
//!     geometry: geo_types::Point<f64>,
//!     ...
//! }
//! ```
use crate::{JsonObject, JsonValue, Result};

use serde::{ser::Error, Serialize, Serializer};

use std::io;

/// Serialize a single data structure to a GeoJSON Feature string.
///
/// Note that `T` must have a column called `geometry`.
///
/// See [`to_feature_collection_string`] if instead you'd like to serialize multiple features to a
/// FeatureCollection.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_feature_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let vec = to_feature_byte_vec(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize elements to a GeoJSON FeatureCollection string.
///
/// Note that `T` must have a column called `geometry`.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_feature_collection_string<T>(values: &[T]) -> Result<String>
where
    T: Serialize,
{
    let vec = to_feature_collection_byte_vec(values)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize a single data structure to a GeoJSON Feature byte vector.
///
/// Note that `T` must have a column called `geometry`.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_feature_byte_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_feature_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize elements to a GeoJSON FeatureCollection byte vector.
///
/// Note that `T` must have a column called `geometry`.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_feature_collection_byte_vec<T>(values: &[T]) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_feature_collection_writer(&mut writer, values)?;
    Ok(writer)
}

/// Serialize a single data structure as a GeoJSON Feature into the IO stream.
///
/// Note that `T` must have a column called `geometry`.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_feature_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: Serialize,
{
    let feature_serializer = FeatureWrapper::new(value);
    let mut serializer = serde_json::Serializer::new(writer);
    feature_serializer.serialize(&mut serializer)?;
    Ok(())
}

/// Serialize elements as a GeoJSON FeatureCollection into the IO stream.
///
/// Note that `T` must have a column called `geometry`.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
pub fn to_feature_collection_writer<W, T>(writer: W, features: &[T]) -> Result<()>
where
    W: io::Write,
    T: Serialize,
{
    use serde::ser::SerializeMap;

    let mut ser = serde_json::Serializer::new(writer);
    let mut map = ser.serialize_map(Some(2))?;
    map.serialize_entry("type", "FeatureCollection")?;
    map.serialize_entry("features", &Features::new(features))?;
    map.end()?;
    Ok(())
}

/// [`serde::serialize_with`](https://serde.rs/field-attrs.html#serialize_with) helper to serialize a type like a
/// [`geo_types`], as a GeoJSON Geometry.
///
/// # Examples
#[cfg_attr(feature = "geo-types", doc = "```")]
#[cfg_attr(not(feature = "geo-types"), doc = "```ignore")]
/// use serde::Serialize;
/// use geojson::ser::serialize_geometry;
///
/// #[derive(Serialize)]
/// struct MyStruct {
///     // Serialize as geojson, rather than using the type's default serialization
///     #[serde(serialize_with = "serialize_geometry")]
///     geometry: geo_types::Point<f64>,
///     name: String,
/// }
///
/// let my_structs = vec![
///     MyStruct {
///         geometry: geo_types::Point::new(11.1, 22.2),
///         name: "Downtown".to_string()
///     },
///     MyStruct {
///         geometry: geo_types::Point::new(33.3, 44.4),
///         name: "Uptown".to_string()
///     }
/// ];
///
/// let geojson_string = geojson::ser::to_feature_collection_string(&my_structs).unwrap();
///
/// assert!(geojson_string.contains(r#""geometry":{"coordinates":[11.1,22.2],"type":"Point"}"#));
/// ```
pub fn serialize_geometry<IG, S>(geometry: IG, ser: S) -> std::result::Result<S::Ok, S::Error>
where
    IG: std::convert::TryInto<crate::Geometry>,
    S: serde::Serializer,
{
    geometry
        .try_into()
        .map_err(|_e| Error::custom("failed to convert geometry to geojson"))
        .and_then(|geojson_geometry| geojson_geometry.serialize(ser))
}

struct Features<'a, T>
where
    T: Serialize,
{
    features: &'a [T],
}

impl<'a, T> Features<'a, T>
where
    T: Serialize,
{
    fn new(features: &'a [T]) -> Self {
        Self { features }
    }
}

impl<'a, T> serde::Serialize for Features<'a, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(None)?;
        for feature in self.features.iter() {
            seq.serialize_element(&FeatureWrapper::new(feature))?;
        }
        seq.end()
    }
}

struct FeatureWrapper<'t, T> {
    feature: &'t T,
}

impl<'t, T> FeatureWrapper<'t, T> {
    fn new(feature: &'t T) -> Self {
        Self { feature }
    }
}

impl<T> Serialize for FeatureWrapper<'_, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut json_object: JsonObject = {
            let value = serde_json::to_value(self.feature).map_err(|e| {
                S::Error::custom(format!("Feature was not serializable as JSON - {}", e))
            })?;
            match value {
                JsonValue::Object(object) => object,
                JsonValue::Null => {
                    return Err(S::Error::custom("expected JSON object but found `null`"))
                }
                JsonValue::Bool(_) => {
                    return Err(S::Error::custom("expected JSON object but found `bool`"))
                }
                JsonValue::Number(_) => {
                    return Err(S::Error::custom("expected JSON object but found `number`"))
                }
                JsonValue::String(_) => {
                    return Err(S::Error::custom("expected JSON object but found `string`"))
                }
                JsonValue::Array(_) => {
                    return Err(S::Error::custom("expected JSON object but found `array`"))
                }
            }
        };

        if !json_object.contains_key("geometry") {
            // Currently it's *required* that the struct's geometry field be named `geometry`.
            //
            // A likely failure case for users is naming it anything else, e.g. `point: geo::Point`.
            //
            // We could just silently blunder on and set `geometry` to None in that case, but
            // printing a specific error message seems more likely to be helpful.
            return Err(S::Error::custom("missing `geometry` field"));
        }
        let geometry = json_object.remove("geometry");

        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("type", "Feature")?;
        map.serialize_entry("geometry", &geometry)?;
        map.serialize_entry("properties", &json_object)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{JsonValue, Position};

    use serde_json::json;

    use std::str::FromStr;

    #[test]
    fn happy_path() {
        #[derive(Serialize)]
        struct MyStruct {
            geometry: crate::Geometry,
            name: String,
        }

        let my_feature = {
            let geometry = crate::Geometry::new(crate::Value::Point(Position::from([0.0, 1.0])));
            let name = "burbs".to_string();
            MyStruct { geometry, name }
        };

        let expected_output_json = json!({
            "type": "Feature",
            "geometry": {
                "coordinates":[0.0,1.0],
                "type":"Point"
            },
            "properties": {
                "name": "burbs"
            }
        });

        let actual_output = to_feature_string(&my_feature).unwrap();
        let actual_output_json = JsonValue::from_str(&actual_output).unwrap();
        assert_eq!(actual_output_json, expected_output_json);
    }

    mod optional_geometry {
        use super::*;
        #[derive(Serialize)]
        struct MyStruct {
            geometry: Option<crate::Geometry>,
            name: String,
        }

        #[test]
        fn with_some_geom() {
            let my_feature = {
                let geometry = Some(crate::Geometry::new(crate::Value::Point(Position::from([
                    0.0, 1.0,
                ]))));
                let name = "burbs".to_string();
                MyStruct { geometry, name }
            };

            let expected_output_json = json!({
                "type": "Feature",
                "geometry": {
                    "coordinates":[0.0,1.0],
                    "type":"Point"
                },
                "properties": {
                    "name": "burbs"
                }
            });

            let actual_output = to_feature_string(&my_feature).unwrap();
            let actual_output_json = JsonValue::from_str(&actual_output).unwrap();
            assert_eq!(actual_output_json, expected_output_json);
        }

        #[test]
        fn with_none_geom() {
            let my_feature = {
                let geometry = None;
                let name = "burbs".to_string();
                MyStruct { geometry, name }
            };

            let expected_output_json = json!({
                "type": "Feature",
                "geometry": null,
                "properties": {
                    "name": "burbs"
                }
            });

            let actual_output = to_feature_string(&my_feature).unwrap();
            let actual_output_json = JsonValue::from_str(&actual_output).unwrap();
            assert_eq!(actual_output_json, expected_output_json);
        }

        #[test]
        fn without_geom_field() {
            #[derive(Serialize)]
            struct MyStructWithoutGeom {
                // geometry: Option<crate::Geometry>,
                name: String,
            }
            let my_feature = {
                let name = "burbs".to_string();
                MyStructWithoutGeom { name }
            };

            let actual_output = to_feature_string(&my_feature).unwrap_err();
            let error_message = actual_output.to_string();

            // BRITTLE: we'll need to update this test if the error message changes.
            assert!(error_message.contains("missing"));
            assert!(error_message.contains("geometry"));
        }

        #[test]
        fn serializes_whatever_geometry() {
            #[derive(Serialize)]
            struct MyStructWithWeirdGeom {
                // This isn't a valid geometry representation, but we don't really have a way to "validate" it
                // so serde will serialize whatever. This test exists just to document current behavior
                // not that it's exactly desirable.
                geometry: Vec<u32>,
                name: String,
            }
            let my_feature = {
                let geometry = vec![1, 2, 3];
                let name = "burbs".to_string();
                MyStructWithWeirdGeom { geometry, name }
            };

            let expected_output_json = json!({
                "type": "Feature",
                "geometry": [1, 2, 3],
                "properties": {
                    "name": "burbs"
                }
            });

            let actual_output = to_feature_string(&my_feature).unwrap();
            let actual_output_json = JsonValue::from_str(&actual_output).unwrap();
            assert_eq!(actual_output_json, expected_output_json);
        }
    }

    #[cfg(feature = "geo-types")]
    mod geo_types_tests {
        use super::*;
        use crate::de::tests::feature_collection;

        #[test]
        fn geometry_field_without_helper() {
            #[derive(Serialize)]
            struct MyStruct {
                // If we forget the "serialize_with" helper, bad things happen.
                // This test documents that:
                //
                // #[serde(serialize_with = "serialize_geometry")]
                geometry: geo_types::Point<f64>,
                name: String,
                age: u64,
            }

            let my_struct = MyStruct {
                geometry: geo_types::point!(x: 125.6, y: 10.1),
                name: "Dinagat Islands".to_string(),
                age: 123,
            };

            let expected_invalid_output = json!({
              "type": "Feature",
              // This isn't a valid geojson-Geometry. This behavior probably isn't desirable, but this
              // test documents the current behavior of what happens if the users forgets "serialize_geometry"
              "geometry": { "x": 125.6, "y": 10.1 },
              "properties": {
                "name": "Dinagat Islands",
                "age": 123
              }
            });

            // Order might vary, so re-parse to do a semantic comparison of the content.
            let output_string = to_feature_string(&my_struct).expect("valid serialization");
            let actual_output = JsonValue::from_str(&output_string).unwrap();

            assert_eq!(actual_output, expected_invalid_output);
        }

        #[test]
        fn geometry_field() {
            #[derive(Serialize)]
            struct MyStruct {
                #[serde(serialize_with = "serialize_geometry")]
                geometry: geo_types::Point<f64>,
                name: String,
                age: u64,
            }

            let my_struct = MyStruct {
                geometry: geo_types::point!(x: 125.6, y: 10.1),
                name: "Dinagat Islands".to_string(),
                age: 123,
            };

            let expected_output = json!({
              "type": "Feature",
              "geometry": {
                "type": "Point",
                "coordinates": [125.6, 10.1]
              },
              "properties": {
                "name": "Dinagat Islands",
                "age": 123
              }
            });

            // Order might vary, so re-parse to do a semantic comparison of the content.
            let output_string = to_feature_string(&my_struct).expect("valid serialization");
            let actual_output = JsonValue::from_str(&output_string).unwrap();

            assert_eq!(actual_output, expected_output);
        }

        #[test]
        fn serialize_feature_collection() {
            #[derive(Serialize)]
            struct MyStruct {
                #[serde(serialize_with = "serialize_geometry")]
                geometry: geo_types::Point<f64>,
                name: String,
                age: u64,
            }

            let my_structs = vec![
                MyStruct {
                    geometry: geo_types::point!(x: 125.6, y: 10.1),
                    name: "Dinagat Islands".to_string(),
                    age: 123,
                },
                MyStruct {
                    geometry: geo_types::point!(x: 2.3, y: 4.5),
                    name: "Neverland".to_string(),
                    age: 456,
                },
            ];

            let output_string =
                to_feature_collection_string(&my_structs).expect("valid serialization");

            // Order might vary, so re-parse to do a semantic comparison of the content.
            let expected_output = feature_collection();
            let actual_output = JsonValue::from_str(&output_string).unwrap();

            assert_eq!(actual_output, expected_output);
        }
    }
}
