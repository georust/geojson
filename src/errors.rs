//! Module for all GeoJSON-related errors
use crate::geometry::deserialize::GeometryType;
use crate::Feature;
use thiserror::Error;

/// Errors which can occur when encoding, decoding, and converting GeoJSON
#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid writer state: {0}")]
    InvalidWriterState(&'static str),
    #[error("IO Error: {0}")]
    Io(std::io::Error),
    #[error("Expected type: `{expected_type}`, but found `{found_type}`")]
    InvalidGeometryConversion {
        expected_type: &'static str,
        found_type: &'static str,
    },
    #[error(
        "Attempted to a convert a feature without a geometry into a geo_types::Geometry: `{0}`"
    )]
    FeatureHasNoGeometry(Box<Feature>),
    #[error("Encountered geometry type: `{geometry_type}` with unexpected coordinates dimensions: {dimensions}")]
    InvalidGeometryDimensions {
        geometry_type: GeometryType,
        dimensions: u8,
    },
    #[error("Encountered geometry type: `{geometry_type}` with no `coordinates` key")]
    GeometryWithoutCoordinatesKey { geometry_type: GeometryType },
    #[error("Encountered GeometryCollection with no `geometries` key")]
    GeometryCollectionWithoutGeometriesKey,
    #[error("Error while deserializing GeoJSON: {0}")]
    MalformedGeoJson(serde_json::Error),
    #[error("Expected GeoJSON type `{expected}`, found `{actual}`")]
    ExpectedType { expected: String, actual: String },
    #[error("A position must contain two or more elements, but got `{0}`")]
    PositionTooShort(usize),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::MalformedGeoJson(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
