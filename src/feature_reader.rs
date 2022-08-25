use crate::de::deserialize_feature_collection;
use crate::{Feature, Result};

use serde::de::DeserializeOwned;

use std::io::Read;

/// Enumerates individual Features from a GeoJSON FeatureCollection
pub struct FeatureReader<R> {
    reader: R,
}

impl<R: Read> FeatureReader<R> {
    /// Create a FeatureReader from the given `reader`.
    pub fn from_reader(reader: R) -> Self {
        Self { reader }
    }

    /// Iterate over the individual [`Feature`s](Feature) of a FeatureCollection.
    ///
    /// If instead you'd like to deserialize directly to your own struct, see [`FeatureReader::deserialize`].
    ///
    /// # Examples
    ///
    /// ```
    /// let feature_collection_string = r#"{
    ///      "type": "FeatureCollection",
    ///      "features": [
    ///          {
    ///            "type": "Feature",
    ///            "geometry": { "type": "Point", "coordinates": [125.6, 10.1] },
    ///            "properties": {
    ///              "name": "Dinagat Islands",
    ///              "age": 123
    ///            }
    ///          },
    ///          {
    ///            "type": "Feature",
    ///            "geometry": { "type": "Point", "coordinates": [2.3, 4.5] },
    ///            "properties": {
    ///              "name": "Neverland",
    ///              "age": 456
    ///            }
    ///          }
    ///      ]
    /// }"#
    /// .as_bytes();
    /// let io_reader = std::io::BufReader::new(feature_collection_string);
    ///
    /// use geojson::FeatureReader;
    /// let feature_reader = FeatureReader::from_reader(io_reader);
    /// for feature in feature_reader.features() {
    ///     let feature = feature.expect("valid geojson feature");
    ///
    ///     let name = feature.property("name").unwrap().as_str().unwrap();
    ///     let age = feature.property("age").unwrap().as_u64().unwrap();
    ///
    ///     if name == "Dinagat Islands" {
    ///         assert_eq!(123, age);
    ///     } else if name == "Neverland" {
    ///         assert_eq!(456, age);
    ///     } else {
    ///         panic!("unexpected name: {}", name);
    ///     }
    /// }
    /// ```
    pub fn features(self) -> impl Iterator<Item = Result<Feature>> {
        #[allow(deprecated)]
        crate::FeatureIterator::new(self.reader)
    }

    /// Deserialize the features of FeatureCollection into your own custom
    /// struct using the [`serde`](../../serde) crate.
    ///
    /// # Examples
    ///
    /// Your struct must implement or derive [`serde::Deserialize`].
    ///
    /// If you have enabled the `geo-types` feature, which is enabled by default, you can
    /// deserialize directly to a useful geometry type.
    ///
    /// ```rust,ignore
    /// use geojson::{FeatureReader, de::deserialize_geometry};
    ///
    /// #[derive(serde::Deserialize)]
    /// struct MyStruct {
    ///     #[serde(deserialize_with = "deserialize_geometry")]
    ///     geometry: geo_types::Point<f64>,
    ///     name: String,
    ///     age: u64,
    /// }
    /// ```
    ///
    /// Then you can deserialize the FeatureCollection directly to your type.
    #[cfg_attr(feature = "geo-types", doc = "```")]
    #[cfg_attr(not(feature = "geo-types"), doc = "```ignore")]
    /// let feature_collection_string = r#"{
    ///     "type": "FeatureCollection",
    ///     "features": [
    ///         {
    ///            "type": "Feature",
    ///            "geometry": { "type": "Point", "coordinates": [125.6, 10.1] },
    ///            "properties": {
    ///              "name": "Dinagat Islands",
    ///              "age": 123
    ///            }
    ///         },
    ///         {
    ///            "type": "Feature",
    ///            "geometry": { "type": "Point", "coordinates": [2.3, 4.5] },
    ///            "properties": {
    ///              "name": "Neverland",
    ///              "age": 456
    ///            }
    ///          }
    ///    ]
    /// }"#.as_bytes();
    /// let io_reader = std::io::BufReader::new(feature_collection_string);
    /// #
    /// # use geojson::{FeatureReader, de::deserialize_geometry};
    /// #
    /// # #[derive(serde::Deserialize)]
    /// # struct MyStruct {
    /// #     #[serde(deserialize_with = "deserialize_geometry")]
    /// #     geometry: geo_types::Point<f64>,
    /// #     name: String,
    /// #     age: u64,
    /// # }
    ///
    /// let feature_reader = FeatureReader::from_reader(io_reader);
    /// for feature in feature_reader.deserialize::<MyStruct>().unwrap() {
    ///     let my_struct = feature.expect("valid geojson feature");
    ///
    ///     if my_struct.name == "Dinagat Islands" {
    ///         assert_eq!(123, my_struct.age);
    ///     } else if my_struct.name == "Neverland" {
    ///         assert_eq!(456, my_struct.age);
    ///     } else {
    ///         panic!("unexpected name: {}", my_struct.name);
    ///     }
    /// }
    /// ```
    ///
    /// If you're not using [`geo-types`](geo_types), you can deserialize to a `geojson::Geometry` instead.
    /// ```rust,ignore
    /// use serde::Deserialize;
    /// #[derive(Deserialize)]
    /// struct MyStruct {
    ///     geometry: geojson::Geometry,
    ///     name: String,
    ///     age: u64,
    /// }
    /// ```
    pub fn deserialize<D: DeserializeOwned>(self) -> Result<impl Iterator<Item = Result<D>>> {
        deserialize_feature_collection(self.reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::Deserialize;
    use serde_json::json;

    #[derive(Deserialize)]
    struct MyRecord {
        geometry: crate::Geometry,
        name: String,
        age: u64,
    }

    fn feature_collection_string() -> String {
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
        .to_string()
    }

    #[test]
    #[cfg(feature = "geo-types")]
    fn deserialize_into_type() {
        let feature_collection_string = feature_collection_string();
        let mut bytes_reader = feature_collection_string.as_bytes();
        let feature_reader = FeatureReader::from_reader(&mut bytes_reader);

        let records: Vec<MyRecord> = feature_reader
            .deserialize()
            .expect("a valid feature collection")
            .map(|result| result.expect("a valid feature"))
            .collect();

        assert_eq!(records.len(), 2);

        assert_eq!(
            records[0].geometry,
            (&geo_types::point!(x: 125.6, y: 10.1)).into()
        );
        assert_eq!(records[0].name, "Dinagat Islands");
        assert_eq!(records[0].age, 123);

        assert_eq!(
            records[1].geometry,
            (&geo_types::point!(x: 2.3, y: 4.5)).into()
        );
        assert_eq!(records[1].name, "Neverland");
        assert_eq!(records[1].age, 456);
    }
}
