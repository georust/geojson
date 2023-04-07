use geojson::de::deserialize_geometry;
use geojson::GeoJson;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::io::BufReader;

fn parse_feature_collection_benchmark(c: &mut Criterion) {
    let geojson_str = include_str!("../tests/fixtures/countries.geojson");

    c.bench_function("parse (countries.geojson)", |b| {
        b.iter(|| {
            let fc = geojson_str.parse::<geojson::FeatureCollection>().unwrap();
            assert_eq!(fc.features.len(), 180);
            black_box(fc);
        })
    });

    c.bench_function("FeatureReader::features (countries.geojson)", |b| {
        b.iter(|| {
            let feature_reader =
                geojson::FeatureReader::from_reader(BufReader::new(geojson_str.as_bytes()));
            let mut count = 0;
            for feature in feature_reader.features() {
                let feature = feature.unwrap();
                black_box(feature);
                count += 1;
            }
            assert_eq!(count, 180);
        });
    });

    c.bench_function("FeatureReader::deserialize (countries.geojson)", |b| {
        b.iter(|| {
            #[allow(unused)]
            #[derive(serde::Deserialize)]
            struct Country {
                geometry: geojson::Geometry,
                name: String,
            }
            let feature_reader =
                geojson::FeatureReader::from_reader(BufReader::new(geojson_str.as_bytes()));

            let mut count = 0;
            for feature in feature_reader.deserialize::<Country>().unwrap() {
                let feature = feature.unwrap();
                black_box(feature);
                count += 1;
            }
            assert_eq!(count, 180);
        });
    });

    #[cfg(feature = "geo-types")]
    c.bench_function(
        "FeatureReader::deserialize to geo-types (countries.geojson)",
        |b| {
            b.iter(|| {
                #[allow(unused)]
                #[derive(serde::Deserialize)]
                struct Country {
                    #[serde(deserialize_with = "deserialize_geometry")]
                    geometry: geo_types::Geometry,
                    name: String,
                }
                let feature_reader =
                    geojson::FeatureReader::from_reader(BufReader::new(geojson_str.as_bytes()));

                let mut count = 0;
                for feature in feature_reader.deserialize::<Country>().unwrap() {
                    let feature = feature.unwrap();
                    black_box(feature);
                    count += 1;
                }
                assert_eq!(count, 180);
            });
        },
    );
}

fn parse_geometry_collection_benchmark(c: &mut Criterion) {
    c.bench_function("parse (geometry_collection.geojson)", |b| {
        let geojson_str = include_str!("../tests/fixtures/geometry_collection.geojson");

        b.iter(|| {
            let _ = black_box(geojson_str.parse::<geojson::GeoJson>());
        });
    });
}

criterion_group!(
    benches,
    parse_feature_collection_benchmark,
    parse_geometry_collection_benchmark
);
criterion_main!(benches);
