//! Module for all GeoJSON-related errors
use crate::geometry::Value as GValue;
use crate::Feature;
use serde_json::value::Value;
use thiserror::Error;

/// Errors which can occur when encoding, decoding, and converting GeoJSON
#[derive(Error, Debug)]
pub enum Error {
    #[error("Encountered non-array value for a 'bbox' object: `{0}`")]
    BboxExpectedArray(Value),
    #[error("Encountered non-numeric value within 'bbox' array")]
    BboxExpectedNumericValues(Value),
    #[error("Encountered a non-object type for GeoJSON: `{0}`")]
    GeoJsonExpectedObject(Value),
    /// This was previously `GeoJsonUnknownType`, but has been split for clarity
    #[error("Expected a Feature, FeatureCollection, or Geometry, but got an empty type")]
    EmptyType,
    /// This was previously `GeoJsonUnknownType`, but has been split for clarity
    #[error("Expected a Feature mapping, but got a `{0}`")]
    NotAFeature(String),
    #[error("Encountered a mismatch when converting to a Geo type: `{0}`")]
    InvalidGeometryConversion(GValue),
    #[error(
        "Attempted to a convert a feature without a geometry into a geo_types::Geometry: `{0}`"
    )]
    FeatureHasNoGeometry(Feature),
    #[error("Encountered an unknown 'geometry' object type: `{0}`")]
    GeometryUnknownType(String),
    #[error("Encountered malformed JSON: {0}")]
    MalformedJson(serde_json::error::Error),
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
    #[error("Expected an Array value, but got `{0}`")]
    ExpectedArrayValue(String),
    #[error("Expected an owned Object, but got `{0}`")]
    ExpectedObjectValue(Value),
}
