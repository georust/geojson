use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_group(c: &mut Criterion) {
    let geojson_str = include_str!("../tests/fixtures/countries.geojson");
    let geojson = geojson_str.parse::<geojson::GeoJson>().unwrap();

    #[cfg(feature="geo-types")]
    c.bench_function("quick_collection", move |b| {
        b.iter(|| {
            let _: Result<geo_types::GeometryCollection<f64>, _> =
                black_box(geojson::quick_collection(&geojson));
        });
    });
}

criterion_group!(benches, benchmark_group);
criterion_main!(benches);
