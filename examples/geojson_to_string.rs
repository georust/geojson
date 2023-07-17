use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use geojson::{Feature, GeoJson};

fn main() -> Result<(), Box<dyn Error>> {
    let file_reader = BufReader::new(File::open("tests/fixtures/canonical/good-feature.geojson")?);

    let feature: Feature = serde_json::from_reader(file_reader)?;
    
    let geojson: GeoJson = feature.into();

    println!("{}", &geojson.to_string());
    println!("{}", &geojson.to_string_pretty()?);

    Ok(())
}
