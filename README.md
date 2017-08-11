rust-geojson
============

[![Build Status](https://travis-ci.org/georust/rust-geojson.svg)](https://travis-ci.org/georust/rust-geojson)
[![geojson on Crates.io](https://meritbadge.herokuapp.com/geojson)](https://crates.io/crates/geojson)

[Documentation](https://docs.rs/geojson/)

Library for serializing the [GeoJSON](http://geojson.org) vector GIS file format

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
use std::collections::HashMap;
use rustc_serialize::json::ToJson;
use geojson::{Feature, GeoJson, Geometry, Value};

let geometry = Geometry::new(
    Value::Point(vec![-120.66029,35.2812])
);

let mut properties = HashMap::new();
properties.insert(
    String::from("name"),
    "Firestone Grill".to_json(),
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
