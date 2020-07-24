#[cfg(test)]
mod roundtrip_tests {
    use geojson::GeoJson;
    use serde_json;
    use std::fs::File;
    use std::io::prelude::*;

    macro_rules! roundtrip_test {
        ($name:ident : $file_name:expr) => {
            #[test]
            fn $name() {
                let fixture_dir_path = "tests/fixtures/canonical/";
                let mut file_path = fixture_dir_path.to_owned();
                file_path.push_str($file_name.to_owned().as_str());

                test_round_trip(&file_path);
            }
        };
    }

    macro_rules! roundtrip_tests {
        ( $($name:ident: $file_name:expr,)* ) => {
            $(
                roundtrip_test!($name: $file_name);
             )*
        }
    }

    roundtrip_tests! {
        test_good_feature_with_id: "good-feature-with-id.geojson",
        test_good_feature_with_string_id: "good-feature-with-string-id.geojson",
        test_good_feature: "good-feature.geojson",
        test_good_feature_collection_bbox: "good-featurecollection-bbox.geojson",
        test_good_feature_collection_bbox3d: "good-featurecollection-bbox3d.geojson",
        test_good_feature_collection_extensions: "good-featurecollection-extensions.geojson",
        test_good_feature_collection: "good-featurecollection.geojson",
        test_good_geometry_collection: "good-geometrycollection.geojson",
        test_good_linestring: "good-linestring.geojson",
        test_good_multilinestring: "good-multilinestring.geojson",
        test_good_multipoint: "good-multipoint.geojson",
        test_good_point_3d: "good-point-3d.geojson",
        test_good_point: "good-point.geojson",
        test_good_polygon: "good-polygon.geojson",
        test_multipolygon: "multipolygon.geojson",
        test_null_geometry: "nullgeometry.geojson",
    }

    /// Verifies that we can parse and then reencode geojson back to the same representation
    /// without losing any data.
    fn test_round_trip(file_path: &str) {
        let mut file = File::open(&file_path).unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);

        // Read and parse the geojson from the file's contents
        let geojson = file_contents
            .parse::<GeoJson<Vec<f64>>>()
            .expect("unable to parse");

        // Now that we've successfully decoded the geojson, re-encode it and compare to the
        // original to make sure nothing was lost.
        let geojson_string = geojson.to_string();

        let original_json: serde_json::Value = serde_json::from_str(&file_contents).unwrap();
        let roundtrip_json: serde_json::Value = serde_json::from_str(&geojson_string).unwrap();

        assert_eq!(original_json, roundtrip_json)
    }
}
