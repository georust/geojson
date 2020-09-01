use serde_json::value::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GJError {
    // Fixme: can we detail the value?
    #[error("Encountered non-array type for a 'bbox' object")]
    BboxExpectedArray,
    // Fixme: can we detail the value?
    #[error("Encountered non-numeric value within 'bbox' array")]
    BboxExpectedNumericValues,
    // Fixme: can we detail the value?
    #[error("Encountered non-object type for GeoJSON")]
    GeoJsonExpectedObject,
    // Fixme: can we detail the value?
    #[error("Encountered unknown GeoJSON object type")]
    GeoJsonUnknownType,
    // Fixme: can we detail the value?
    #[error("Encountered unknown 'geometry' object type")]
    GeometryUnknownType,
    // Fixme: can we detail the error?
    #[error("Encountered malformed JSON")]
    MalformedJson,
    #[error("Encountered neither object type nor null type for 'properties' object: `{0}`")]
    PropertiesExpectedObjectOrNull(Value),
    #[error("Encountered neither object type nor null type for 'geometry' field on 'feature' object: `{0}`")]
    FeatureInvalidGeometryValue(Value),
    #[error("Encountered neither number type nor string type for 'id' field on 'feature' object")]
    FeatureInvalidIdentifierType,
    #[error("Expected GeoJSON type `{expected}`, found `{actual}`")]
    ExpectedType { expected: String, actual: String },

    // FIXME: make these types more specific
    #[error("Expected a String value")]
    ExpectedStringValue,
    #[error("Expected a GeoJSON property: `{0}`")]
    ExpectedProperty(String),
    #[error("Expected a floating-point value")]
    ExpectedF64Value,
    #[error("Expected an array value")]
    ExpectedArrayValue,
    #[error("Expected an object")]
    ExpectedObjectValue,
}
