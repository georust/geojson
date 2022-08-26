use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use geojson::FeatureCollection;

fn main() -> Result<(), Box<dyn Error>> {
    let file_reader = BufReader::new(File::open("tests/fixtures/countries.geojson")?);

    let countries: FeatureCollection = serde_json::from_reader(file_reader)?;
    assert_eq!(countries.features.len(), 180);

    // Write the structs back to GeoJSON
    let file_writer = BufWriter::new(File::create("example-output-countries.geojson")?);
    serde_json::to_writer(file_writer, &countries)?;

    Ok(())
}
