rust-geojson
============

[![Build Status](https://travis-ci.org/georust/rust-geojson.svg)](https://travis-ci.org/georust/rust-geojson)
[![geojson on Crates.io](https://meritbadge.herokuapp.com/geojson)](https://crates.io/crates/geojson)

[Documentation](https://georust.github.io/rust-geojson/)

Library for serializing the [GeoJSON](http://geojson.org) vector GIS file format

# Examples

## Reading

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

## Writing

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
    crs: None,
    bbox: None,
    geometry: geometry,
    id: None,
    properties: Some(properties),
});

let geojson_string = geojson.to_string();
```

Licensed under version two of the Apache License.
