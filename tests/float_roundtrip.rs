use geojson::{GeoJson, GeometryValue};

fn coordinate_bits(value: &GeoJson) -> u64 {
    let GeoJson::Geometry(geometry) = value else {
        panic!("expected a geometry");
    };
    let GeometryValue::Point { coordinates } = &geometry.value else {
        panic!("expected a point");
    };
    coordinates[0].to_bits()
}

// Correctly rounded binary64 bits, including signed zero and subnormals.
const CASES: [(&str, u64); 12] = [
    ("51.248178375505404", 0x40499fc44f1b2f60_u64),
    ("-93.31137037688033", 0xc05753ed7e04693b_u64),
    ("-36.573994842753436", 0xc0424978a9bad96e_u64),
    ("2.0030397744267762e-253", 0x0b777f245544ff83_u64),
    ("7.101215824554616e260", 0x761717c13ccbd1b8_u64),
    ("5e-324", 0x0000000000000001_u64),
    ("-5e-324", 0x8000000000000001_u64),
    ("2.2250738585072014e-308", 0x0010000000000000_u64),
    ("1e-300", 0x01a56e1fc2f8f359_u64),
    ("1e300", 0x7e37e43c8800759c_u64),
    ("0.0", 0x0000000000000000_u64),
    ("-0.0", 0x8000000000000000_u64),
];

#[test]
#[cfg(feature = "float_roundtrip")]
fn precise_coordinates_from_str_and_reader() {
    for (coordinate, expected) in CASES {
        let json = format!(r#"{{"type":"Point","coordinates":[{coordinate},0.0]}}"#);
        let from_str: GeoJson = json.parse().unwrap();
        let from_reader = GeoJson::from_reader(json.as_bytes()).unwrap();
        for parsed in [from_str, from_reader] {
            assert_eq!(coordinate_bits(&parsed), expected, "{coordinate}");
            let serialized = parsed.to_string();
            let reparsed: GeoJson = serialized.parse().unwrap();
            assert_eq!(coordinate_bits(&reparsed), expected, "{serialized}");
        }
    }
}

#[test]
#[cfg(not(feature = "float_roundtrip"))]
fn coordinates_follow_serde_json_without_the_feature() {
    // Cargo can unify serde_json/float_roundtrip through another dependency.
    // Without our feature, follow that parser rather than assume it loses bits.
    for (coordinate, _) in CASES {
        let json = format!(r#"{{"type":"Point","coordinates":[{coordinate},0.0]}}"#);
        let expected_str: serde_json::Value = serde_json::from_str(&json).unwrap();
        let expected_reader: serde_json::Value = serde_json::from_reader(json.as_bytes()).unwrap();
        let from_str: GeoJson = json.parse().unwrap();
        let from_reader = GeoJson::from_reader(json.as_bytes()).unwrap();
        for (parsed, expected) in [(from_str, expected_str), (from_reader, expected_reader)] {
            assert_eq!(
                coordinate_bits(&parsed),
                expected["coordinates"][0].as_f64().unwrap().to_bits(),
                "{coordinate}"
            );
            let serialized = parsed.to_string();
            let reparsed: GeoJson = serialized.parse().unwrap();
            let expected: serde_json::Value = serde_json::from_str(&serialized).unwrap();
            assert_eq!(
                coordinate_bits(&reparsed),
                expected["coordinates"][0].as_f64().unwrap().to_bits(),
                "{serialized}"
            );
        }
    }
}
