use geojson::{de::deserialize_geometry, ser::serialize_geometry};

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[cfg(not(feature = "geo-types"))]
fn main() -> Result<(), Box<dyn Error>> {
    panic!("this example requires geo-types")
}

#[cfg(feature = "geo-types")]
fn main() -> Result<(), Box<dyn Error>> {
    #[derive(Serialize, Deserialize)]
    struct Country {
        #[serde(
            serialize_with = "serialize_geometry",
            deserialize_with = "deserialize_geometry"
        )]
        geometry: geo_types::Geometry,
        name: String,
    }

    let file_reader = BufReader::new(File::open("tests/fixtures/countries.geojson")?);

    // Create a Vec of Country structs from the GeoJSON
    let countries: Vec<Country> =
        geojson::de::deserialize_feature_collection_to_vec::<Country>(file_reader)?;
    assert_eq!(countries.len(), 180);

    // Write the structs back to GeoJSON
    let file_writer = BufWriter::new(File::create("example-output-countries.geojson")?);
    geojson::ser::to_feature_collection_writer(file_writer, &countries)?;

    Ok(())
}
