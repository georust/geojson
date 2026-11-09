#![cfg(feature = "preserve_order")]

use geojson::Feature;
use serde_json::{Value, json};

#[test]
fn round_trip_keeps_property_and_foreign_member_order() {
    let mut feature: Feature = r#"{
        "type":"Feature", "z-extra":1, "geometry":null, "a-extra":2,
        "properties":{"z":{"b":0,"a":1},"a":2,"m":3,"b":4}
    }"#
    .parse()
    .unwrap();
    feature.set_property("m", 5);
    feature.remove_property("a");

    let restored: Feature = feature.to_string().parse().unwrap();
    assert_eq!(
        serde_json::to_string(&restored.properties.unwrap()).unwrap(),
        r#"{"z":{"b":0,"a":1},"m":5,"b":4}"#
    );
    assert_eq!(
        serde_json::to_string(&restored.foreign_members.unwrap()).unwrap(),
        r#"{"z-extra":1,"a-extra":2}"#
    );
}

#[test]
fn extracting_geometry_and_id_keeps_property_order() {
    let input = json!({"geometry":null,"z":1,"id":"row","a":2,"m":3,"b":4});
    let output = geojson::ser::to_feature_string(&input).unwrap();
    let feature: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(feature["id"], "row");
    assert_eq!(
        feature["properties"].to_string(),
        r#"{"z":1,"a":2,"m":3,"b":4}"#
    );
}
