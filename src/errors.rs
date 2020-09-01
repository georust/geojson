//! Module for all GeoJSON-related errors
use serde_json::value::Value;
use thiserror::Error;
use crate::geometry::Value as GValue;

/// Errors which can occur when encoding, decoding, and converting GeoJSON
#[derive(Error, Debug)]
pub enum Error {
    #[error("Encountered non-array value for a 'bbox' object: `{0}`.")]
    BboxExpectedArray(Value),
    #[error("Encountered non-numeric value within 'bbox' array.")]
    BboxExpectedNumericValues(Value),
    #[error("Encountered a non-object type for GeoJSON: `{0}`.")]
    GeoJsonExpectedObject(Value),
    #[error("Expected a Feature, FeatureCollection, or Geometry, but got an empty type.")]
    EmptyType,
    #[error("Expected a Feature mapping, but got a `{0}`.")]
    NotAFeature(String),
    #[error("Encountered a mismatch when converting to a Geo type: `{0}`.")]
    InvalidGeometryConversion(GValue),
    #[error("Encountered an unknown 'geometry' object type: `{0}`")]
    GeometryUnknownType(String),
    // Fixme: can we detail the error?
    #[error("Encountered malformed JSON")]
    MalformedJson,
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
    ExpectedF64Value,
    #[error("Expected an array value, but got None")]
    ExpectedArrayValue,
    #[error("Expected an owned array value, but got `{0}`")]
    ExpectedOwnedArrayValue(Value),
    #[error("Expected an owned Object, but got `{0}`")]
    ExpectedObjectValue(Value),
}
