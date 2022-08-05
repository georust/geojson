use criterion::{black_box, criterion_group, criterion_main, Criterion};
use geojson::GeoJson;
use std::io::BufReader;

fn parse_feature_collection_benchmark(c: &mut Criterion) {
    c.bench_function("parse (countries.geojson)", |b| {
        let geojson_str = include_str!("../tests/fixtures/countries.geojson");

        b.iter(|| {
            let _ = black_box({
                match geojson_str.parse::<geojson::GeoJson>() {
                    Ok(GeoJson::FeatureCollection(fc)) => {
                        assert_eq!(fc.features.len(), 180);
                    }
                    _ => panic!("unexpected result"),
                }
            });
        })
    });

    c.bench_function("FeatureIter (countries.geojson)", |b| {
        let geojson_str = include_str!("../tests/fixtures/countries.geojson");

        b.iter(|| {
            let feature_iter =
                geojson::FeatureIterator::new(BufReader::new(geojson_str.as_bytes()));
            let _ = black_box({
                let mut count = 0;
                for _ in feature_iter {
                    count += 1;
                }
                assert_eq!(count, 184);
            });
        });
    });
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
