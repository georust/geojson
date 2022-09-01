use geojson::{de::deserialize_geometry, ser::serialize_geometry, FeatureReader, FeatureWriter};

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

    let reader = {
        let file_reader = BufReader::new(File::open("tests/fixtures/countries.geojson")?);
        FeatureReader::from_reader(file_reader)
    };

    let mut writer = {
        let file_writer = BufWriter::new(File::create("example-output-contries.geojson")?);
        FeatureWriter::from_writer(file_writer)
    };

    let mut country_count = 0;
    for country in reader.deserialize::<Country>()? {
        let country = country?;
        country_count += 1;

        writer.serialize(&country)?;
    }

    assert_eq!(country_count, 180);
    Ok(())
}
