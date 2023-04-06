// Copyright 2015 The GeoRust Developers
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
#![allow(deprecated)]

use crate::{Feature, Result};

use serde::Deserialize;
use std::io;
use std::marker::PhantomData;

// TODO: Eventually make this private - and expose only FeatureReader.
#[deprecated(
    since = "0.24.0",
    note = "use FeatureReader::from_reader(io).features() instead"
)]
/// Iteratively deserialize individual features from a stream containing a
/// GeoJSON [`FeatureCollection`](struct@crate::FeatureCollection)
///
/// This has the benefit of not having to wait until the end of the
/// stream to get results, and avoids having to allocate memory for the complete collection.
///
/// Based on example code found at <https://github.com/serde-rs/serde/issues/903#issuecomment-297488118>.
///
/// [GeoJSON Format Specification § 3.3](https://datatracker.ietf.org/doc/html/rfc7946#section-3.3)
pub struct FeatureIterator<'de, R, D = Feature> {
    reader: R,
    state: State,
    output: PhantomData<D>,
    lifetime: PhantomData<&'de ()>,
}

#[derive(Debug, Copy, Clone)]
enum State {
    BeforeFeatures,
    DuringFeatures,
    AfterFeatures,
}

impl<'de, R, D> FeatureIterator<'de, R, D> {
    pub fn new(reader: R) -> Self {
        FeatureIterator {
            reader,
            state: State::BeforeFeatures,
            output: PhantomData,
            lifetime: PhantomData,
        }
    }
}

impl<'de, R, D> FeatureIterator<'de, R, D>
where
    R: io::Read,
{
    fn seek_to_next_feature(&mut self) -> Result<bool> {
        let mut next_bytes = [0];
        loop {
            self.reader.read_exact(&mut next_bytes)?;
            let next_byte = next_bytes[0] as char;
            if next_byte.is_whitespace() {
                continue;
            }

            match (self.state, next_byte) {
                (State::BeforeFeatures, '[') => {
                    self.state = State::DuringFeatures;
                    return Ok(true);
                }
                (State::BeforeFeatures, _) => {
                    continue;
                }
                (State::DuringFeatures, ',') => {
                    return Ok(true);
                }
                (State::DuringFeatures, ']') => {
                    self.state = State::AfterFeatures;
                    return Ok(false);
                }
                (State::AfterFeatures, _) => {
                    unreachable!("should not seek if we've already finished processing features")
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("next byte: {}", next_byte),
                    )
                    .into());
                }
            }
        }
    }
}

impl<'de, R, D> Iterator for FeatureIterator<'de, R, D>
where
    R: io::Read,
    D: Deserialize<'de>,
{
    type Item = Result<D>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.seek_to_next_feature() {
            Ok(true) => {}
            Ok(false) => return None,
            Err(err) => {
                return Some(Err(err));
            }
        }

        let de = serde_json::Deserializer::from_reader(&mut self.reader);
        match de.into_iter().next() {
            Some(Ok(v)) => Some(Ok(v)),
            Some(Err(err)) => Some(Err(err.into())),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Geometry, Position, Value};

    use std::io::BufReader;

    fn fc() -> &'static str {
        r#"
      {
        "type": "FeatureCollection",
        "features": [
          {
            "type": "Feature",
            "geometry": {
              "type": "Point",
              "coordinates": [102.0, 0.5]
            },
            "properties": {
              "prop0": "value0"
            }
          },
          {
            "type": "Feature",
            "geometry": {
              "type": "LineString",
              "coordinates": [
                [102.0, 0.0], [103.0, 1.0], [104.0, 0.0], [105.0, 1.0]
              ]
            },
            "properties": {
              "prop0": "value0",
              "prop1": 0.0
            }
          },
          {
            "type": "Feature",
            "geometry": {
              "type": "Polygon",
              "coordinates": [
                [
                  [100.0, 0.0], [101.0, 0.0], [101.0, 1.0],
                  [100.0, 1.0], [100.0, 0.0]
                ]
              ]
            },
            "properties": {
              "prop0": "value0",
              "prop1": { "this": "that" }
            }
          }
        ]
      }"#
    }

    #[test]
    fn stream_read_test() {
        let mut fi = FeatureIterator::<_, Feature>::new(BufReader::new(fc().as_bytes()));
        assert_eq!(
            Geometry {
                bbox: None,
                value: Value::Point(Position::from([102.0, 0.5])),
                foreign_members: None,
            },
            fi.next().unwrap().unwrap().geometry.unwrap()
        );
        assert_eq!(
            Geometry {
                bbox: None,
                value: Value::LineString(vec![
                    Position::from([102.0, 0.0]),
                    Position::from([103.0, 1.0]),
                    Position::from([104.0, 0.0]),
                    Position::from([105.0, 1.0])
                ]),
                foreign_members: None,
            },
            fi.next().unwrap().unwrap().geometry.unwrap()
        );
        assert_eq!(
            Geometry {
                bbox: None,
                value: Value::Polygon(vec![vec![
                    Position::from([100.0, 0.0]),
                    Position::from([101.0, 0.0]),
                    Position::from([101.0, 1.0]),
                    Position::from([100.0, 1.0]),
                    Position::from([100.0, 0.0])
                ]]),
                foreign_members: None,
            },
            fi.next().unwrap().unwrap().geometry.unwrap()
        );
        assert!(fi.next().is_none());
    }

    mod field_ordering {
        use super::*;
        use crate::Feature;

        #[test]
        fn type_field_before_features_field() {
            let type_first = r#"
              {
                type: "FeatureCollection",
                features: [
                  {
                    "type": "Feature",
                    "geometry": {
                      "type": "Point",
                      "coordinates": [1.1, 1.2]
                    },
                    "properties": { }
                  },
                  {
                    "type": "Feature",
                    "geometry": {
                      "type": "Point",
                      "coordinates": [2.1, 2.2]
                    },
                    "properties": { }
                  }
                ]
              }
            "#;
            let features: Vec<Feature> =
                FeatureIterator::new(BufReader::new(type_first.as_bytes()))
                    .map(Result::unwrap)
                    .collect();
            assert_eq!(features.len(), 2);
        }

        #[test]
        fn features_field_before_type_field() {
            let type_first = r#"
              {
                features: [
                  {
                    "type": "Feature",
                    "geometry": {
                      "type": "Point",
                      "coordinates": [1.1, 1.2]
                    },
                    "properties": {}
                  },
                  {
                    "type": "Feature",
                    "geometry": {
                      "type": "Point",
                      "coordinates": [2.1, 2.2]
                    },
                    "properties": { }
                  }
                ],
                type: "FeatureCollection"
              }
            "#;
            let features: Vec<Feature> =
                FeatureIterator::new(BufReader::new(type_first.as_bytes()))
                    .map(Result::unwrap)
                    .collect();
            assert_eq!(features.len(), 2);
        }
    }
}
