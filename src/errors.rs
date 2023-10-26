//! Module for all GeoJSON-related errors
use crate::Feature;
use serde_json::value::Value;
use thiserror::Error;

/// Errors which can occur when encoding, decoding, and converting GeoJSON
#[derive(Error, Debug)]
pub enum Error<T: geo_types::CoordFloat + serde::Serialize> {
    #[error("Encountered non-array value for a 'bbox' object: `{0}`")]
    BboxExpectedArray(Value),
    #[error("Encountered non-numeric value within 'bbox' array")]
    BboxExpectedNumericValues(Value),
    #[error("Encountered a non-object type for GeoJSON: `{0}`")]
    GeoJsonExpectedObject(Value),
    /// This was previously `GeoJsonUnknownType`, but has been split for clarity
    #[error("Expected a Feature, FeatureCollection, or Geometry, but got an empty type")]
    EmptyType,
    #[error("invalid writer state: {0}")]
    InvalidWriterState(&'static str),
    #[error("IO Error: {0}")]
    Io(std::io::Error),
    /// This was previously `GeoJsonUnknownType`, but has been split for clarity
    #[error("Expected a Feature mapping, but got a `{0}`")]
    NotAFeature(String),
    #[error("Expected type: `{expected_type}`, but found `{found_type}`")]
    InvalidGeometryConversion {
        expected_type: &'static str,
        found_type: &'static str,
    },
    #[error(
        "Attempted to a convert a feature without a geometry into a geo_types::Geometry: `{0}`"
    )]
    FeatureHasNoGeometry(Feature<T>),
    #[error("Encountered an unknown 'geometry' object type: `{0}`")]
    GeometryUnknownType(String),
    #[error("Error while deserializing JSON: {0}")]
    MalformedJson(serde_json::Error),
    #[error("Encountered neither object type nor null type for 'properties' object: `{0}`")]
    PropertiesExpectedObjectOrNull(Value),
    #[error("Encountered neither object type nor null type for 'geometry' field on 'feature' object: `{0}`")]
    FeatureInvalidGeometryValue(Value),
    #[error(
        "Encountered neither number type nor string type for 'id' field on 'feature' object: `{0}`"
    )]
    FeatureInvalidIdentifierType(Value),
    #[error("Expected GeoJSON type `{expected}`, found `{actual}`")]
    ExpectedType { expected: String, actual: String },
    #[error("Expected a String value, but got a `{0}`")]
    ExpectedStringValue(Value),
    #[error("Expected a GeoJSON property for `{0}`, but got None")]
    ExpectedProperty(String),
    #[error("Expected a floating-point value, but got None")]
    ExpectedFloatValue,
    #[error("Expected an Array value, but got `{0}`")]
    ExpectedArrayValue(String),
    #[error("Expected an owned Object, but got `{0}`")]
    ExpectedObjectValue(Value),
    #[error("A position must contain two or more elements, but got `{0}`")]
    PositionTooShort(usize),
}

pub type Result<T, U = f64> = std::result::Result<T, Error<U>>;

impl<T> From<serde_json::Error> for Error<T>
where
    T: geo_types::CoordFloat + serde::Serialize,
{
    fn from(error: serde_json::Error) -> Self {
        Self::MalformedJson(error)
    }
}

impl<T> From<std::io::Error> for Error<T>
where
    T: geo_types::CoordFloat+ serde::Serialize,
{
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
