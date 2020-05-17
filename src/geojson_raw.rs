use serde;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Position(Vec<f64>);

#[derive(Debug, Deserialize)]
pub struct Point {
    coordinates: Position,
}

#[derive(Debug, Deserialize)]
pub struct MultiPoint {
    coordinates: Vec<Position>,
}

#[derive(Debug, Deserialize)]
pub struct LineString {
    coordinates: Vec<Position>,
}

#[derive(Debug, Deserialize)]
pub struct MultiLineString {
    coordinates: Vec<Vec<Position>>,
}

#[derive(Debug, Deserialize)]
pub struct Polygon {
    coordinates: Vec<Vec<Position>>,
}

#[derive(Debug, Deserialize)]
pub struct MultiPolygon {
    coordinates: Vec<Vec<Vec<Position>>>,
}

#[derive(Debug, Deserialize)]
pub struct GeometryCollection {
    geometries: Vec<Geometry>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Geometry {
    Point(Point),
    MultiPoint(MultiPoint),
    LineString(LineString),
    MultiLineString(MultiLineString),
    Polygon(Polygon),
    MultiPolygon(MultiPolygon),
    GeometryCollection(GeometryCollection),
}

#[derive(Debug, Deserialize)]
pub struct Feature {
    geometry: Geometry,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum FeatureWithTag {
    Feature(Feature),
}

#[derive(Debug, Deserialize)]
pub struct FeatureCollection {
    features: Vec<FeatureWithTag>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum GeoJson {
    Point(Point),
    MultiPoint(MultiPoint),
    LineString(LineString),
    MultiLineString(MultiLineString),
    Polygon(Polygon),
    MultiPolygon(MultiPolygon),
    GeometryCollection(GeometryCollection),
    Feature(Feature),
    FeatureCollection(FeatureCollection),
}

#[cfg(test)]
mod test {
    use super::GeoJson;

    #[test]
    fn test_parse_line_string() {
        let input = r#"{
            "type": "LineString",
            "coordinates": [[-100, 40], [-105, 45], [-110, 55]]
        }"#;

        let parsed: GeoJson = serde_json::from_str(&input).unwrap();

        println!("parsed: {:?}", parsed);
    }

    #[test]
    fn test_parse_feature_collection() {
        let input = r#"{
            "type": "FeatureCollection",
            "features": [
              {
                "type": "Feature",
                "properties": {
                  "population": 200
                },
                "geometry": {
                  "type": "Point",
                  "coordinates": [-112.0372, 46.608058]
                }
              }
            ]
        }"#;

        let parsed: GeoJson = serde_json::from_str(&input).unwrap();

        println!("parsed: {:?}", parsed);
    }
}
