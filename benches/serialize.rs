use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn serialize_feature_collection_benchmark(c: &mut Criterion) {
    let geojson_str = include_str!("../tests/fixtures/countries.geojson");

    c.bench_function(
        "serialize geojson::FeatureCollection struct (countries.geojson)",
        |b| {
            let geojson = geojson_str.parse::<geojson::GeoJson>().unwrap();

            b.iter(|| {
                black_box({
                    let geojson_string = serde_json::to_string(&geojson).unwrap();
                    // Sanity check that we serialized a long string of some kind.
                    assert_eq!(geojson_string.len(), 256890);
                });
            });
        },
    );

    c.bench_function("serialize custom struct (countries.geojson)", |b| {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Country {
            geometry: geojson::Geometry,
            name: String,
        }
        let features =
            geojson::de::deserialize_feature_collection_str_to_vec::<Country>(geojson_str).unwrap();
        assert_eq!(features.len(), 180);

        b.iter(|| {
            black_box({
                let geojson_string = geojson::ser::to_feature_collection_string(&features).unwrap();
                // Sanity check that we serialized a long string of some kind.
                //
                // Note this is slightly shorter than the GeoJson round-trip above because we drop
                // some fields, like foreign members
                assert_eq!(geojson_string.len(), 254908);
            });
        });
    });
}

criterion_group!(benches, serialize_feature_collection_benchmark);
criterion_main!(benches);
