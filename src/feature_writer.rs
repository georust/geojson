use crate::ser::to_feature_writer;
use crate::{Error, Feature, Result};

use serde::Serialize;
use std::io::Write;

#[derive(PartialEq)]
enum State {
    New,
    WritingFeatures,
    WritingForeignMembers,
    Finished,
}

/// Write Features to a FeatureCollection
pub struct FeatureWriter<W: Write> {
    writer: W,
    state: State,
}

impl<W: Write> FeatureWriter<W> {
    /// Create a FeatureWriter from the given `writer`.
    ///
    /// To append features from your custom structs, use [`FeatureWriter::serialize`].
    ///
    /// To append features from [`Feature`] use [`FeatureWriter::write_feature`].
    ///
    /// To write a foreign member, use [`FeatureWriter::write_foreign_member`] before appending any
    /// features.
    pub fn from_writer(writer: W) -> Self {
        Self {
            writer,
            state: State::New,
        }
    }

    /// Write a [`crate::Feature`] struct to the output stream. If you'd like to
    /// serialize your own custom structs, see [`FeatureWriter::serialize`] instead.
    pub fn write_feature(&mut self, feature: &Feature) -> Result<()> {
        match self.state {
            State::Finished => {
                return Err(Error::InvalidWriterState(
                    "cannot write another Feature when writer has already finished",
                ))
            }
            State::New => {
                self.write_prefix()?;
                self.state = State::WritingFeatures;
            }
            State::WritingFeatures => {
                self.write_str(",")?;
            }
            State::WritingForeignMembers => {
                self.write_str(r#" "features": ["#)?;
                self.state = State::WritingFeatures;
            }
        }
        serde_json::to_writer(&mut self.writer, feature)?;
        Ok(())
    }

    /// Serialize your own custom struct to the features of a FeatureCollection using the
    /// [`serde`] crate.
    ///
    /// # Examples
    ///
    /// Your struct must implement or derive [`serde::Serialize`].
    ///
    /// If you have enabled the `geo-types` feature, which is enabled by default, you can
    /// serialize directly from a useful geometry type.
    ///
    /// ```rust,ignore
    /// use geojson::{FeatureWriter, ser::serialize_geometry};
    ///
    /// #[derive(serde::Serialize)]
    /// struct MyStruct {
    ///     #[serde(serialize_with = "serialize_geometry")]
    ///     geometry: geo_types::Point<f64>,
    ///     name: String,
    ///     age: u64,
    /// }
    /// ```
    ///
    /// Then you can serialize the FeatureCollection directly from your type.
    #[cfg_attr(feature = "geo-types", doc = "```")]
    #[cfg_attr(not(feature = "geo-types"), doc = "```ignore")]
    /// #
    /// # use geojson::{FeatureWriter, ser::serialize_geometry};
    /// #
    /// # #[derive(serde::Serialize)]
    /// # struct MyStruct {
    /// #     #[serde(serialize_with = "serialize_geometry")]
    /// #     geometry: geo_types::Point<f64>,
    /// #     name: String,
    /// #     age: u64,
    /// # }
    ///
    /// let dinagat = MyStruct {
    ///     geometry: geo_types::point!(x: 125.6, y: 10.1),
    ///     name: "Dinagat Islands".to_string(),
    ///     age: 123
    /// };
    ///
    /// let neverland = MyStruct {
    ///     geometry: geo_types::point!(x: 2.3, y: 4.5),
    ///     name: "Neverland".to_string(),
    ///     age: 456
    /// };
    ///
    /// let mut output: Vec<u8> = vec![];
    /// {
    ///     let io_writer = std::io::BufWriter::new(&mut output);
    ///     let mut feature_writer = FeatureWriter::from_writer(io_writer);
    ///     feature_writer.serialize(&dinagat).unwrap();
    ///     feature_writer.serialize(&neverland).unwrap();
    /// }
    ///
    /// let expected_output = r#"{
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
    ///
    /// fn assert_eq_json(bytes_1: &[u8], bytes_2: &[u8]) {
    ///     // check for semantic equality, discarding any formatting/whitespace differences
    ///     let json_1: serde_json::Value = serde_json::from_slice(bytes_1).unwrap();
    ///     let json_2: serde_json::Value = serde_json::from_slice(bytes_2).unwrap();
    ///     assert_eq!(json_1, json_2);
    /// }
    ///
    /// assert_eq_json(expected_output, &output);
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
    pub fn serialize<S: Serialize>(&mut self, value: &S) -> Result<()> {
        match self.state {
            State::Finished => {
                return Err(Error::InvalidWriterState(
                    "cannot serialize another record when writer has already finished",
                ))
            }
            State::New => {
                self.write_prefix()?;
                self.state = State::WritingFeatures;
            }
            State::WritingFeatures => {
                self.write_str(",")?;
            }
            State::WritingForeignMembers => {
                self.write_str(r#" "features": ["#)?;
                self.state = State::WritingFeatures;
            }
        }
        to_feature_writer(&mut self.writer, value)
    }

    /// Write a [foreign member](https://datatracker.ietf.org/doc/html/rfc7946#section-6) to the
    /// output stream. This must be done before appending any features.
    pub fn write_foreign_member<T: ?Sized + Serialize>(
        &mut self,
        key: &str,
        value: &T,
    ) -> Result<()> {
        match self.state {
            State::Finished => {
                return Err(Error::InvalidWriterState(
                    "cannot write foreign member when writer has already finished",
                ))
            }
            State::New => {
                self.write_str(r#"{ "type": "FeatureCollection", "#)?;
                write!(self.writer, "\"{key}\": ")?;
                serde_json::to_writer(&mut self.writer, value)?;
                self.write_str(",")?;

                self.state = State::WritingForeignMembers;
                Ok(())
            }
            State::WritingFeatures => {
                return Err(Error::InvalidWriterState(
                    "must write foreign members before any features",
                ))
            }
            State::WritingForeignMembers => {
                write!(self.writer, "\"{key}\": ")?;
                serde_json::to_writer(&mut self.writer, value)?;
                self.write_str(",")?;
                Ok(())
            }
        }
    }

    /// Writes the closing syntax for the FeatureCollection.
    ///
    /// You shouldn't normally need to call this manually, as the writer will close itself upon
    /// being dropped.
    pub fn finish(&mut self) -> Result<()> {
        match self.state {
            State::Finished => {
                return Err(Error::InvalidWriterState(
                    "cannot finish writer - it's already finished",
                ))
            }
            State::New => {
                self.state = State::Finished;
                self.write_prefix()?;
                self.write_suffix()?;
            }
            State::WritingFeatures | State::WritingForeignMembers => {
                self.state = State::Finished;
                self.write_suffix()?;
            }
        }
        Ok(())
    }

    /// Flush the underlying writer buffer.
    ///
    /// You shouldn't normally need to call this manually, as the writer will flush itself upon
    /// being dropped.
    pub fn flush(&mut self) -> Result<()> {
        Ok(self.writer.flush()?)
    }

    fn write_prefix(&mut self) -> Result<()> {
        self.write_str(r#"{ "type": "FeatureCollection", "features": ["#)
    }

    fn write_suffix(&mut self) -> Result<()> {
        self.write_str("]}")
    }

    fn write_str(&mut self, text: &str) -> Result<()> {
        self.writer.write_all(text.as_bytes())?;
        Ok(())
    }
}

impl<W: Write> Drop for FeatureWriter<W> {
    fn drop(&mut self) {
        if self.state != State::Finished {
            _ = self.finish().map_err(|e| {
               log::error!("FeatureWriter errored while finishing in Drop impl. To handle errors like this, explicitly call `FeatureWriter::finish`. Error: {}", e);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JsonValue;

    use serde_json::json;

    // an example struct that we want to serialize
    #[derive(Serialize)]
    struct MyRecord {
        geometry: crate::Geometry,
        name: String,
        age: u64,
    }

    #[test]
    fn write_empty() {
        let mut buffer: Vec<u8> = vec![];
        {
            let mut writer = FeatureWriter::from_writer(&mut buffer);
            writer.finish().unwrap();
        }

        let expected = json!({
            "type": "FeatureCollection",
            "features": []
        });

        let actual_json: JsonValue = serde_json::from_slice(&buffer).unwrap();
        assert_eq!(actual_json, expected);
    }

    #[test]
    fn finish_on_drop() {
        let mut buffer: Vec<u8> = vec![];
        {
            _ = FeatureWriter::from_writer(&mut buffer);
        }

        let expected = json!({
            "type": "FeatureCollection",
            "features": []
        });

        let actual_json: JsonValue = serde_json::from_slice(&buffer).unwrap();
        assert_eq!(actual_json, expected);
    }

    #[test]
    fn write_feature() {
        let mut buffer: Vec<u8> = vec![];
        {
            let mut writer = FeatureWriter::from_writer(&mut buffer);

            let record_1 = {
                let mut props = serde_json::Map::new();
                props.insert("name".to_string(), "Mishka".into());
                props.insert("age".to_string(), 12.into());

                Feature {
                    bbox: None,
                    geometry: Some(crate::Geometry::from(crate::Value::Point(vec![1.1, 1.2]))),
                    id: None,
                    properties: Some(props),
                    foreign_members: None,
                }
            };

            let record_2 = {
                let mut props = serde_json::Map::new();
                props.insert("name".to_string(), "Jane".into());
                props.insert("age".to_string(), 22.into());

                Feature {
                    bbox: None,
                    geometry: Some(crate::Geometry::from(crate::Value::Point(vec![2.1, 2.2]))),
                    id: None,
                    properties: Some(props),
                    foreign_members: None,
                }
            };

            writer.write_feature(&record_1).unwrap();
            writer.write_feature(&record_2).unwrap();
            writer.flush().unwrap();
        }

        let expected = json!({
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "geometry": { "type": "Point", "coordinates": [1.1, 1.2] },
                    "properties": { "name": "Mishka", "age": 12
                    }
                },
                {
                    "type": "Feature",
                    "geometry": { "type": "Point", "coordinates": [2.1, 2.2] },
                    "properties": {
                        "name": "Jane",
                        "age": 22
                    }
                }
            ]
        });

        let actual_json: JsonValue = serde_json::from_slice(&buffer).expect("valid json");
        assert_eq!(actual_json, expected)
    }

    #[test]
    fn serialize() {
        let mut buffer: Vec<u8> = vec![];
        {
            let mut writer = FeatureWriter::from_writer(&mut buffer);
            let record_1 = MyRecord {
                geometry: crate::Geometry::from(crate::Value::Point(vec![1.1, 1.2])),
                name: "Mishka".to_string(),
                age: 12,
            };
            let record_2 = MyRecord {
                geometry: crate::Geometry::from(crate::Value::Point(vec![2.1, 2.2])),
                name: "Jane".to_string(),
                age: 22,
            };
            writer.serialize(&record_1).unwrap();
            writer.serialize(&record_2).unwrap();
            writer.flush().unwrap();
        }

        let expected = json!({
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "geometry": { "type": "Point", "coordinates": [1.1, 1.2] },
                    "properties": { "name": "Mishka", "age": 12
                    }
                },
                {
                    "type": "Feature",
                    "geometry": { "type": "Point", "coordinates": [2.1, 2.2] },
                    "properties": {
                        "name": "Jane",
                        "age": 22
                    }
                }
            ]
        });

        let actual_json: JsonValue = serde_json::from_slice(&buffer).expect("valid json");
        assert_eq!(actual_json, expected)
    }

    #[test]
    fn write_foreign_members() {
        let mut buffer: Vec<u8> = vec![];
        {
            let mut writer = FeatureWriter::from_writer(&mut buffer);

            writer.write_foreign_member("extra", "string").unwrap();
            writer.write_foreign_member("list", &[1, 2, 3]).unwrap();
            writer
                .write_foreign_member("nested", &json!({"foo": "bar"}))
                .unwrap();

            let record_1 = {
                let mut props = serde_json::Map::new();
                props.insert("name".to_string(), "Mishka".into());
                props.insert("age".to_string(), 12.into());

                Feature {
                    bbox: None,
                    geometry: Some(crate::Geometry::from(crate::Value::Point(vec![1.1, 1.2]))),
                    id: None,
                    properties: Some(props),
                    foreign_members: None,
                }
            };

            writer.write_feature(&record_1).unwrap();
            writer.flush().unwrap();
        }

        let expected = json!({
            "type": "FeatureCollection",
            "extra": "string",
            "list": [1, 2, 3],
            "nested": {
                "foo": "bar",
            },
            "features": [
                {
                    "type": "Feature",
                    "geometry": { "type": "Point", "coordinates": [1.1, 1.2] },
                    "properties": { "name": "Mishka", "age": 12
                    }
                },
            ]
        });

        println!("{}", String::from_utf8(buffer.clone()).unwrap());
        let actual_json: JsonValue = serde_json::from_slice(&buffer).expect("valid json");
        assert_eq!(actual_json, expected)
    }

    #[cfg(feature = "geo-types")]
    mod test_geo_types {
        use super::*;
        use crate::ser::serialize_geometry;

        // an example struct that we want to serialize
        #[derive(Serialize)]
        struct MyGeoRecord {
            #[serde(serialize_with = "serialize_geometry")]
            geometry: geo_types::Point,
            name: String,
            age: u64,
        }

        #[test]
        fn serialize_geo_types() {
            let mut buffer: Vec<u8> = vec![];
            {
                let mut writer = FeatureWriter::from_writer(&mut buffer);
                let record_1 = MyGeoRecord {
                    geometry: geo_types::point!(x: 1.1, y: 1.2),
                    name: "Mishka".to_string(),
                    age: 12,
                };
                let record_2 = MyGeoRecord {
                    geometry: geo_types::point!(x: 2.1, y: 2.2),
                    name: "Jane".to_string(),
                    age: 22,
                };
                writer.serialize(&record_1).unwrap();
                writer.serialize(&record_2).unwrap();
                writer.flush().unwrap();
            }

            let expected = json!({
                "type": "FeatureCollection",
                "features": [
                    {
                        "type": "Feature",
                        "geometry": { "type": "Point", "coordinates": [1.1, 1.2] },
                        "properties": {
                            "name": "Mishka",
                            "age": 12
                        }
                    },
                    {
                        "type": "Feature",
                        "geometry": { "type": "Point", "coordinates": [2.1, 2.2] },
                        "properties": {
                            "name": "Jane",
                            "age": 22
                        }
                    }
                ]
            });

            let actual_json: JsonValue = serde_json::from_slice(&buffer).expect("valid json");
            assert_eq!(actual_json, expected)
        }
    }
}
