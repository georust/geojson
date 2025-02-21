//!
//! To build instances of your struct from a GeoJSON String or reader, your type *must*
//! implement or derive [`serde::Deserialize`]:
//!
//! ```rust, ignore
//! #[derive(serde::Deserialize)]
//! struct MyStruct {
//!     ...
//! }
//! ```
//!
//! Your type *must* have a field called `geometry` and it must be `deserialize_with` [`deserialize_geometry`](crate::de::deserialize_geometry):
//!  ```rust, ignore
//! #[derive(serde::Deserialize)]
//! struct MyStruct {
//!     #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
//!     geometry: geo_types::Point<f64>,
//!     ...
//! }
//! ```
//!
//! All fields in your struct other than `geometry` will be deserialized from the `properties` of the
//! GeoJSON Feature.
//!
//! # Examples
#![cfg_attr(feature = "geo-types", doc = "```")]
#![cfg_attr(not(feature = "geo-types"), doc = "```ignore")]
//! use serde::Deserialize;
//! use geojson::de::deserialize_geometry;
//!
//! #[derive(Deserialize)]
//! struct MyStruct {
//!     // Deserialize from GeoJSON, rather than expecting the type's default serialization
//!     #[serde(deserialize_with = "deserialize_geometry")]
//!     geometry: geo_types::Point<f64>,
//!     name: String,
//!     population: u64
//! }
//!
//! let input_geojson = serde_json::json!(
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
//! ).to_string();
//!
//! let my_structs: Vec<MyStruct> = geojson::de::deserialize_feature_collection_str_to_vec(&input_geojson).unwrap();
//! assert_eq!("Downtown", my_structs[0].name);
//! assert_eq!(11.1, my_structs[0].geometry.x());
//!
//! assert_eq!("Uptown", my_structs[1].name);
//! assert_eq!(33.3, my_structs[1].geometry.x());
//! ```
//!
//! # Reading *and* Writing GeoJSON
//!
//! This module is only concerned with _reading in_ GeoJSON. If you'd also like to write GeoJSON
//! output, you'll want to combine this with the functionality from the [`crate::ser`] module:
//! ```ignore
//! #[derive(serde::Serialize, serde::Deserialize)]
//! struct MyStruct {
//!     // Serialize as GeoJSON, rather than using the type's default serialization
//!     #[serde(serialize_with = "serialize_geometry", deserialize_with = "deserialize_geometry")]
//!     geometry: geo_types::Point<f64>,
//!     ...
//! }
//! ```
use crate::{Feature, FeatureReader, JsonValue, Result};

use std::convert::{TryFrom, TryInto};
use std::fmt::Formatter;
use std::io::Read;
use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer, Error, IntoDeserializer};

/// Deserialize a GeoJSON FeatureCollection into your custom structs.
///
/// Your struct must implement or derive `serde::Deserialize`.
///
/// You must use the [`deserialize_geometry`] helper if you are using geo_types or some other geometry
/// representation other than geojson::Geometry.
///
/// # Examples
#[cfg_attr(feature = "geo-types", doc = "```")]
#[cfg_attr(not(feature = "geo-types"), doc = "```ignore")]
/// use serde::Deserialize;
/// use geojson::de::deserialize_geometry;
///
/// #[derive(Deserialize)]
/// struct MyStruct {
///     // You must use the `deserialize_geometry` helper if you are using geo_types or some other
///     // geometry representation other than geojson::Geometry
///     #[serde(deserialize_with = "deserialize_geometry")]
///     geometry: geo_types::Point<f64>,
///     name: String,
/// }
///
/// let feature_collection_str = r#"{
///     "type": "FeatureCollection",
///     "features": [
///         {
///             "type": "Feature",
///             "geometry": { "type": "Point", "coordinates": [11.1, 22.2] },
///             "properties": { "name": "Downtown" }
///         },
///         {
///             "type": "Feature",
///             "geometry": { "type": "Point", "coordinates": [33.3, 44.4] },
///             "properties": { "name": "Uptown" }
///         }
///     ]
/// }"#;
/// let reader = feature_collection_str.as_bytes();
///
/// // enumerate over the features in the feature collection
/// for (idx, feature_result) in geojson::de::deserialize_feature_collection::<MyStruct>(reader).unwrap().enumerate() {
///     let my_struct = feature_result.expect("valid geojson for MyStruct");
///     if idx == 0 {
///         assert_eq!(my_struct.name, "Downtown");
///         assert_eq!(my_struct.geometry.x(), 11.1);
///     } else if idx == 1 {
///         assert_eq!(my_struct.name, "Uptown");
///         assert_eq!(my_struct.geometry.x(), 33.3);
///     } else {
///         unreachable!("there are only two features in this collection");
///     }
/// }
/// ```
pub fn deserialize_feature_collection<'de, T>(
    feature_collection_reader: impl Read,
) -> Result<impl Iterator<Item = Result<T>>>
where
    T: Deserialize<'de>,
{
    #[allow(deprecated)]
    let iter = crate::FeatureIterator::new(feature_collection_reader).map(
        |feature_value: Result<JsonValue>| {
            let deserializer = feature_value?.into_deserializer();
            let visitor = FeatureVisitor::new();
            let record: T = deserializer.deserialize_map(visitor)?;

            Ok(record)
        },
    );
    Ok(iter)
}

/// Build a `Vec` of structs from a GeoJson `&str`.
///
/// See [`deserialize_feature_collection`] for more.
pub fn deserialize_feature_collection_str_to_vec<'de, T>(
    feature_collection_str: &str,
) -> Result<Vec<T>>
where
    T: Deserialize<'de>,
{
    let feature_collection_reader = feature_collection_str.as_bytes();
    deserialize_feature_collection(feature_collection_reader)?.collect()
}

/// Build a `Vec` of structs from a GeoJson reader.
///
/// See [`deserialize_feature_collection`] for more.
pub fn deserialize_feature_collection_to_vec<'de, T>(
    feature_collection_reader: impl Read,
) -> Result<Vec<T>>
where
    T: Deserialize<'de>,
{
    deserialize_feature_collection(feature_collection_reader)?.collect()
}

/// [`serde::deserialize_with`](https://serde.rs/field-attrs.html#deserialize_with) helper to deserialize a GeoJSON Geometry into another type, like a
/// [`geo_types`] Geometry.
///
/// # Examples
#[cfg_attr(feature = "geo-types", doc = "```")]
#[cfg_attr(not(feature = "geo-types"), doc = "```ignore")]
/// use serde::Deserialize;
/// use geojson::de::deserialize_geometry;
///
/// #[derive(Deserialize)]
/// struct MyStruct {
///     #[serde(deserialize_with = "deserialize_geometry")]
///     geometry: geo_types::Point<f64>,
///     name: String,
/// }
///
/// let feature_collection_str = r#"{
///     "type": "FeatureCollection",
///     "features": [
///         {
///             "type": "Feature",
///             "geometry": { "type": "Point", "coordinates": [11.1, 22.2] },
///             "properties": { "name": "Downtown" }
///         },
///         {
///             "type": "Feature",
///             "geometry": { "type": "Point", "coordinates": [33.3, 44.4] },
///             "properties": { "name": "Uptown" }
///         }
///     ]
/// }"#;
///
/// let features: Vec<MyStruct> = geojson::de::deserialize_feature_collection_str_to_vec(feature_collection_str).unwrap();
///
/// assert_eq!(features[0].name, "Downtown");
/// assert_eq!(features[0].geometry.x(), 11.1);
/// ```
pub fn deserialize_geometry<'de, D, G>(deserializer: D) -> std::result::Result<G, D::Error>
where
    D: Deserializer<'de>,
    G: TryFrom<crate::Geometry>,
    G::Error: std::fmt::Display,
{
    let geojson_geometry = crate::Geometry::deserialize(deserializer)?;
    geojson_geometry
        .try_into()
        .map_err(deserialize_error_msg::<D>)
}

/// [`serde::deserialize_with`](https://serde.rs/field-attrs.html#deserialize_with) helper to deserialize an optional GeoJSON Geometry into another type,
/// like an optional [`geo_types`] Geometry.
///
/// # Examples
#[cfg_attr(feature = "geo-types", doc = "```")]
#[cfg_attr(not(feature = "geo-types"), doc = "```ignore")]
/// use geojson::de::deserialize_optional_geometry;
/// use serde::Deserialize;
/// use serde_json::{json, from_value};
///
/// #[derive(Deserialize)]
/// struct MyStruct {
///     #[serde(rename = "count")]
///     _count: usize,
///     #[serde(default, deserialize_with = "deserialize_optional_geometry")]
///     geometry: Option<geo_types::Point<f64>>,
/// }
///
/// let json = json! {{
///     "count": 0,
///     "geometry": {
///         "type": "Point",
///         "coordinates": [125.6, 10.1]
///     },
/// }};
/// let feature: MyStruct = from_value(json).unwrap();
/// assert!(feature.geometry.is_some());
///
/// let json = json! {{
///     "count": 1,
/// }};
/// let feature: MyStruct = from_value(json).unwrap();
/// assert!(feature.geometry.is_none())
/// ```
pub fn deserialize_optional_geometry<'de, D, G>(
    deserializer: D,
) -> std::result::Result<Option<G>, D::Error>
where
    D: Deserializer<'de>,
    G: TryFrom<crate::Geometry>,
    G::Error: std::fmt::Display,
{
    Option::<crate::Geometry>::deserialize(deserializer)?
        .map(TryInto::try_into)
        .transpose()
        .map_err(deserialize_error_msg::<D>)
}

fn deserialize_error_msg<'de, D: Deserializer<'de>>(
    error: impl std::fmt::Display,
) -> <D as serde::Deserializer<'de>>::Error {
    Error::custom(format!(
        "unable to convert from geojson Geometry: {}",
        error
    ))
}

/// Deserialize a GeoJSON FeatureCollection into [`Feature`] structs.
///
/// If instead you'd like to deserialize your own structs from GeoJSON, see [`deserialize_feature_collection`].
pub fn deserialize_features_from_feature_collection(
    feature_collection_reader: impl Read,
) -> impl Iterator<Item = Result<Feature>> {
    FeatureReader::from_reader(feature_collection_reader).features()
}

/// Deserialize a single GeoJSON Feature into your custom struct.
///
/// It's more common to deserialize a FeatureCollection than a single feature. If you're looking to
/// do that, see [`deserialize_feature_collection`] instead.
///
/// Your struct must implement or derive `serde::Deserialize`.
///
/// # Examples
#[cfg_attr(feature = "geo-types", doc = "```")]
#[cfg_attr(not(feature = "geo-types"), doc = "```ignore")]
/// use serde::Deserialize;
/// use geojson::de::deserialize_geometry;
///
/// #[derive(Deserialize)]
/// struct MyStruct {
///     // You must use the `deserialize_geometry` helper if you are using geo_types or some other
///     // geometry representation other than geojson::Geometry
///     #[serde(deserialize_with = "deserialize_geometry")]
///     geometry: geo_types::Point<f64>,
///     name: String,
/// }
///
/// let feature_str = r#"{
///     "type": "Feature",
///     "geometry": { "type": "Point", "coordinates": [11.1, 22.2] },
///     "properties": { "name": "Downtown" }
/// }"#;
/// let reader = feature_str.as_bytes();
///
/// // build your struct from GeoJSON
/// let my_struct = geojson::de::deserialize_single_feature::<MyStruct>(reader).expect("valid geojson for MyStruct");
///
/// assert_eq!(my_struct.name, "Downtown");
/// assert_eq!(my_struct.geometry.x(), 11.1);
/// ```
pub fn deserialize_single_feature<'de, T>(feature_reader: impl Read) -> Result<T>
where
    T: Deserialize<'de>,
{
    let feature_value: JsonValue = serde_json::from_reader(feature_reader)?;
    let deserializer = feature_value.into_deserializer();
    let visitor = FeatureVisitor::new();
    Ok(deserializer.deserialize_map(visitor)?)
}

/// Interpret a [`Feature`] as an instance of type `T`.
///
/// This is analogous to [`serde_json::from_value`](https://docs.rs/serde_json/latest/serde_json/fn.from_value.html)
///
/// `T`'s `geometry` field will be deserialized from `feature.geometry`.
/// All other fields will be deserialized from `feature.properties`.
///
/// # Examples
#[cfg_attr(feature = "geo-types", doc = "```")]
#[cfg_attr(not(feature = "geo-types"), doc = "```ignore")]
/// use serde::Deserialize;
/// use geojson::Feature;
/// use geojson::de::{from_feature, deserialize_geometry, deserialize_single_feature};
/// use std::str::FromStr;
///
/// #[derive(Deserialize)]
/// struct MyStruct {
///     // Deserialize `geometry` as GeoJSON, rather than using the type's default deserialization
///     #[serde(deserialize_with = "deserialize_geometry")]
///     geometry: geo_types::Point,
///     name: String,
/// }
///
/// let geojson_str = r#"{
///     "type": "Feature",
///     "geometry": { "type": "Point", "coordinates": [1.0, 2.0] },
///     "properties": {
///         "name": "My Name"
///     }
/// }"#;
/// let feature  = Feature::from_str(geojson_str).unwrap();
///
/// let my_struct: MyStruct = from_feature(feature).unwrap();
///
/// assert_eq!("My Name", my_struct.name);
/// assert_eq!(geo_types::Point::new(1.0, 2.0), my_struct.geometry);
/// ```
///
/// # Errors
///
/// Deserialization can fail if `T`'s implementation of `Deserialize` decides to fail.
pub fn from_feature<'de, T>(feature: Feature) -> Result<T>
where
    T: Deserialize<'de>,
{
    let feature_value: JsonValue = serde_json::to_value(feature)?;
    let deserializer = feature_value.into_deserializer();
    let visitor = FeatureVisitor::new();
    Ok(deserializer.deserialize_map(visitor)?)
}

struct FeatureVisitor<D> {
    _marker: PhantomData<D>,
}

impl<D> FeatureVisitor<D> {
    fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<'de, D> serde::de::Visitor<'de> for FeatureVisitor<D>
where
    D: Deserialize<'de>,
{
    type Value = D;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a valid GeoJSON Feature object")
    }

    fn visit_map<A>(self, mut map_access: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut has_feature_type = false;
        use std::collections::HashMap;
        let mut hash_map: HashMap<String, JsonValue> = HashMap::new();

        while let Some((key, value)) = map_access.next_entry::<String, JsonValue>()? {
            if key == "type" {
                if value.as_str() == Some("Feature") {
                    has_feature_type = true;
                } else {
                    return Err(Error::custom(
                        "GeoJSON Feature had a `type` other than \"Feature\"",
                    ));
                }
            } else if key == "geometry" {
                if let JsonValue::Object(_) = value {
                    hash_map.insert("geometry".to_string(), value);
                } else {
                    return Err(Error::custom("GeoJSON Feature had a unexpected geometry"));
                }
            } else if key == "properties" {
                if let JsonValue::Object(properties) = value {
                    // flatten properties onto struct
                    for (prop_key, prop_value) in properties {
                        hash_map.insert(prop_key, prop_value);
                    }
                } else {
                    return Err(Error::custom("GeoJSON Feature had a unexpected geometry"));
                }
            } else {
                log::debug!("foreign members are not handled by Feature deserializer")
            }
        }

        if has_feature_type {
            let d2 = hash_map.into_deserializer();
            let result =
                Deserialize::deserialize(d2).map_err(|e| Error::custom(format!("{}", e)))?;
            Ok(result)
        } else {
            Err(Error::custom(
                "A GeoJSON Feature must have a `type: \"Feature\"` field, but found none.",
            ))
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use crate::{JsonObject, JsonValue};

    use serde_json::json;

    pub(crate) fn feature_collection() -> JsonValue {
        json!({
            "type": "FeatureCollection",
            "features": [
                {
                  "type": "Feature",
                  "geometry": {
                    "type": "Point",
                    "coordinates": [125.6, 10.1]
                  },
                  "properties": {
                    "name": "Dinagat Islands",
                    "age": 123
                  }
                },
                {
                  "type": "Feature",
                  "geometry": {
                    "type": "Point",
                    "coordinates": [2.3, 4.5]
                  },
                  "properties": {
                    "name": "Neverland",
                    "age": 456
                  }
                }
            ]
        })
    }

    #[test]
    fn test_deserialize_feature_collection() {
        use crate::Feature;

        let feature_collection_string = feature_collection().to_string();
        let bytes_reader = feature_collection_string.as_bytes();

        let records: Vec<Feature> = deserialize_features_from_feature_collection(bytes_reader)
            .map(|feature_result: Result<Feature>| feature_result.unwrap())
            .collect();

        assert_eq!(records.len(), 2);
        let first_age = {
            let props = records.first().unwrap().properties.as_ref().unwrap();
            props.get("age").unwrap().as_i64().unwrap()
        };
        assert_eq!(first_age, 123);

        let second_age = {
            let props = records.get(1).unwrap().properties.as_ref().unwrap();
            props.get("age").unwrap().as_i64().unwrap()
        };
        assert_eq!(second_age, 456);
    }

    #[cfg(feature = "geo-types")]
    mod geo_types_tests {
        use super::*;

        use serde::Deserialize;

        #[test]
        fn geometry_field() {
            #[derive(Deserialize)]
            struct MyStruct {
                #[serde(deserialize_with = "deserialize_geometry")]
                geometry: geo_types::Geometry<f64>,
                name: String,
                age: u64,
            }

            let feature_collection_string = feature_collection().to_string();
            let bytes_reader = feature_collection_string.as_bytes();

            let records: Vec<MyStruct> = deserialize_feature_collection(bytes_reader)
                .expect("a valid feature collection")
                .collect::<Result<Vec<_>>>()
                .expect("valid features");

            assert_eq!(records.len(), 2);

            assert_eq!(
                records[0].geometry,
                geo_types::point!(x: 125.6, y: 10.1).into()
            );
            assert_eq!(records[0].name, "Dinagat Islands");
            assert_eq!(records[0].age, 123);

            assert_eq!(
                records[1].geometry,
                geo_types::point!(x: 2.3, y: 4.5).into()
            );
            assert_eq!(records[1].name, "Neverland");
            assert_eq!(records[1].age, 456);
        }

        #[test]
        fn specific_geometry_variant_field() {
            #[derive(Deserialize)]
            struct MyStruct {
                #[serde(deserialize_with = "deserialize_geometry")]
                geometry: geo_types::Point<f64>,
                name: String,
                age: u64,
            }

            let feature_collection_string = feature_collection().to_string();
            let bytes_reader = feature_collection_string.as_bytes();

            let records: Vec<MyStruct> = deserialize_feature_collection(bytes_reader)
                .expect("a valid feature collection")
                .collect::<Result<Vec<_>>>()
                .expect("valid features");

            assert_eq!(records.len(), 2);

            assert_eq!(records[0].geometry, geo_types::point!(x: 125.6, y: 10.1));
            assert_eq!(records[0].name, "Dinagat Islands");
            assert_eq!(records[0].age, 123);

            assert_eq!(records[1].geometry, geo_types::point!(x: 2.3, y: 4.5));
            assert_eq!(records[1].name, "Neverland");
            assert_eq!(records[1].age, 456);
        }

        #[test]
        fn wrong_geometry_variant_field() {
            #[allow(unused)]
            #[derive(Deserialize)]
            struct MyStruct {
                #[serde(deserialize_with = "deserialize_geometry")]
                geometry: geo_types::LineString<f64>,
                name: String,
                age: u64,
            }

            let feature_collection_string = feature_collection().to_string();
            let bytes_reader = feature_collection_string.as_bytes();

            let records: Vec<Result<MyStruct>> = deserialize_feature_collection(bytes_reader)
                .unwrap()
                .collect();
            assert_eq!(records.len(), 2);
            assert!(records[0].is_err());
            assert!(records[1].is_err());

            let err = match records[0].as_ref() {
                Ok(_ok) => panic!("expected Err, but found OK"),
                Err(e) => e,
            };

            // This will fail if we update our error text, but I wanted to show that the error text
            // is reasonably discernible.
            let expected_err_text = r#"Error while deserializing JSON: unable to convert from geojson Geometry: Expected type: `LineString`, but found `Point`"#;
            assert_eq!(err.to_string(), expected_err_text);
        }

        #[test]
        fn deserializes_optional_point() {
            #[derive(serde::Deserialize)]
            struct MyStruct {
                #[serde(rename = "count")]
                _count: usize,
                #[serde(default, deserialize_with = "deserialize_optional_geometry")]
                geometry: Option<geo_types::Point<f64>>,
            }

            let json = json! {{
                "count": 0,
                "geometry": {
                    "type": "Point",
                    "coordinates": [125.6, 10.1]
                },
            }};
            let feature: MyStruct = serde_json::from_value(json).unwrap();
            assert!(feature.geometry.is_some());

            let json = json! {{
                "count": 1,
            }};
            let feature: MyStruct = serde_json::from_value(json).unwrap();
            assert!(feature.geometry.is_none())
        }

        #[test]
        fn test_from_feature() {
            #[derive(Debug, PartialEq, Deserialize)]
            struct MyStruct {
                #[serde(deserialize_with = "deserialize_geometry")]
                geometry: geo_types::Point<f64>,
                name: String,
                age: u64,
            }

            let feature = Feature {
                bbox: None,
                geometry: Some(crate::Geometry::new(crate::Value::Point(vec![125.6, 10.1]))),
                id: None,
                properties: Some(
                    json!({
                        "name": "Dinagat Islands",
                        "age": 123,
                    })
                    .as_object()
                    .unwrap()
                    .clone(),
                ),
                foreign_members: None,
            };

            let actual: MyStruct = from_feature(feature).unwrap();
            let expected = MyStruct {
                geometry: geo_types::Point::new(125.6, 10.1),
                name: "Dinagat Islands".to_string(),
                age: 123,
            };
            assert_eq!(actual, expected);
        }
    }

    #[cfg(feature = "geo-types")]
    #[test]
    fn roundtrip() {
        use crate::ser::serialize_geometry;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            #[serde(
                serialize_with = "serialize_geometry",
                deserialize_with = "deserialize_geometry"
            )]
            geometry: geo_types::Point<f64>,
            name: String,
            age: u64,
        }

        let feature_collection_string = feature_collection().to_string();
        let bytes_reader = feature_collection_string.as_bytes();

        let mut elements = deserialize_feature_collection_to_vec::<MyStruct>(bytes_reader).unwrap();
        for element in &mut elements {
            element.age += 1;
            element.geometry.set_x(element.geometry.x() + 1.0);
        }
        let actual_output = crate::ser::to_feature_collection_string(&elements).unwrap();

        use std::str::FromStr;
        let actual_output_json = JsonValue::from_str(&actual_output).unwrap();
        let expected_output_json = json!({
            "type": "FeatureCollection",
            "features": [
                {
                  "type": "Feature",
                  "geometry": {
                    "type": "Point",
                    "coordinates": [126.6, 10.1]
                  },
                  "properties": {
                    "name": "Dinagat Islands",
                    "age": 124
                  }
                },
                {
                  "type": "Feature",
                  "geometry": {
                    "type": "Point",
                    "coordinates": [3.3, 4.5]
                  },
                  "properties": {
                    "name": "Neverland",
                    "age": 457
                  }
                }
            ]
        });

        assert_eq!(actual_output_json, expected_output_json);
    }

    #[cfg(feature = "geo-types")]
    #[test]
    fn all_props() {
        use crate::ser::serialize_geometry;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct MyStruct {
            #[serde(
                serialize_with = "serialize_geometry",
                deserialize_with = "deserialize_geometry"
            )]
            geometry: geo_types::Point<f64>,

            #[serde(flatten)]
            properties: JsonObject,
        }

        let feature_collection_string = feature_collection().to_string();
        let bytes_reader = feature_collection_string.as_bytes();

        let mut elements = deserialize_feature_collection_to_vec::<MyStruct>(bytes_reader).unwrap();
        for element in &mut elements {
            // dbg!(&element.properties);
            // => [src/de.rs:615] &element.properties = {
            //    "age": Number(123),
            //    "name": String("Dinagat Islands"),
            // }
            let mut age = element
                .properties
                .get("age")
                .expect("key to exist")
                .as_u64()
                .expect("numeric");
            age += 1;
            *element.properties.get_mut("age").expect("key to exist") = age.into();
            element.geometry.set_x(element.geometry.x() + 1.0);
        }
        let actual_output = crate::ser::to_feature_collection_string(&elements).unwrap();

        use std::str::FromStr;
        let actual_output_json = JsonValue::from_str(&actual_output).unwrap();
        let expected_output_json = json!({
            "type": "FeatureCollection",
            "features": [
                {
                  "type": "Feature",
                  "geometry": {
                    "type": "Point",
                    "coordinates": [126.6, 10.1]
                  },
                  "properties": {
                    "name": "Dinagat Islands",
                    "age": 124
                  }
                },
                {
                  "type": "Feature",
                  "geometry": {
                    "type": "Point",
                    "coordinates": [3.3, 4.5]
                  },
                  "properties": {
                    "name": "Neverland",
                    "age": 457
                  }
                }
            ]
        });

        assert_eq!(actual_output_json, expected_output_json);
    }
}
