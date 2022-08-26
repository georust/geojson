use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Country {
    // see the geo_types example if you want to store
    // geotypes in your struct
    geometry: geojson::Geometry,
    name: String,
}

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn main() -> Result<(), Box<dyn Error>> {
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
