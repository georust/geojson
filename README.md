# geojson

[Documentation](https://docs.rs/geojson/)

Library for serializing the [GeoJSON](http://geojson.org) vector GIS file format

## Minimum Rust Version

This library requires a minimum Rust version of 1.34 (released April 11 2019)

## Examples

### Reading

```rust
use geojson::GeoJson;

let geojson_str = r#"
{
    "type": "Feature",
    "properties": {
        "name": "Firestone Grill"
    },
    "geometry": {
        "type": "Point",
        "coordinates": [-120.66029,35.2812]
    }
}
"#;

let geojson = geojson_str.parse::<GeoJson>().unwrap();
```

### Writing

```rust
use geojson::{Feature, GeoJson, Geometry, Value, JsonObject, JsonValue};

let geometry = Geometry::new(
    Value::Point(Position::from([-120.66029,35.2812]))
);

let mut properties = JsonObject::new();
properties.insert(
    String::from("name"),
    JsonValue::from("Firestone Grill"),
);

let geojson = GeoJson::Feature(Feature {
    bbox: None,
    geometry: Some(geometry),
    id: None,
    properties: Some(properties),
    foreign_members: None,
});

let geojson_string = geojson.to_string();
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
