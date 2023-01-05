use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::convert::TryFrom;

fn benchmark_group(c: &mut Criterion) {
    let geojson_str = include_str!("../tests/fixtures/countries.geojson");
    let geojson = geojson_str.parse::<geojson::GeoJson>().unwrap();

    #[cfg(feature = "geo-types")]
    c.bench_function("Convert to geo-types", move |b| {
        b.iter(|| black_box(geo_types::GeometryCollection::<f64>::try_from(&geojson).unwrap()));
    });
}

criterion_group!(benches, benchmark_group);
criterion_main!(benches);
