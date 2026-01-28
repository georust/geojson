use geo_types::{self, CoordFloat};

use crate::{Feature, FeatureCollection, GeometryValue, Position};

use crate::{LineStringType, PointType, PolygonType};
use std::convert::From;

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::Point<T>> for GeometryValue
where
    T: CoordFloat,
{
    fn from(point: &geo_types::Point<T>) -> Self {
        let coords = create_point_type(point);
        GeometryValue::new_point(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::MultiPoint<T>> for GeometryValue
where
    T: CoordFloat,
{
    fn from(multi_point: &geo_types::MultiPoint<T>) -> Self {
        let coords: Vec<_> = multi_point
            .0
            .iter()
            .map(|point| create_point_type(point))
            .collect();
        GeometryValue::new_multi_point(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::LineString<T>> for GeometryValue
where
    T: CoordFloat,
{
    fn from(line_string: &geo_types::LineString<T>) -> Self {
        let coords = create_line_string_type(line_string);
        GeometryValue::new_line_string(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::Line<T>> for GeometryValue
where
    T: CoordFloat,
{
    fn from(line: &geo_types::Line<T>) -> Self {
        let coords = create_from_line_type(line);
        GeometryValue::new_line_string(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::Triangle<T>> for GeometryValue
where
    T: CoordFloat,
{
    fn from(triangle: &geo_types::Triangle<T>) -> Self {
        let coords = create_from_triangle_type(triangle);
        GeometryValue::new_polygon(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::Rect<T>> for GeometryValue
where
    T: CoordFloat,
{
    fn from(rect: &geo_types::Rect<T>) -> Self {
        let coords = create_from_rect_type(rect);
        GeometryValue::new_polygon(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::MultiLineString<T>> for GeometryValue
where
    T: CoordFloat,
{
    fn from(multi_line_string: &geo_types::MultiLineString<T>) -> Self {
        let coords = create_multi_line_string_type(multi_line_string);
        GeometryValue::new_multi_line_string(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::Polygon<T>> for GeometryValue
where
    T: CoordFloat,
{
    fn from(polygon: &geo_types::Polygon<T>) -> Self {
        let coords = create_polygon_type(polygon);
        GeometryValue::new_polygon(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::MultiPolygon<T>> for GeometryValue
where
    T: CoordFloat,
{
    fn from(multi_polygon: &geo_types::MultiPolygon<T>) -> Self {
        let coords = create_multi_polygon_type(multi_polygon);
        GeometryValue::new_multi_polygon(coords)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::GeometryCollection<T>> for GeometryValue
where
    T: CoordFloat,
{
    fn from(geometry_collection: &geo_types::GeometryCollection<T>) -> Self {
        let values = geometry_collection
            .0
            .iter()
            .map(|geometry| crate::Geometry::new(GeometryValue::from(geometry)));
        GeometryValue::new_geometry_collection(values)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&geo_types::GeometryCollection<T>> for FeatureCollection
where
    T: CoordFloat,
{
    fn from(geometry_collection: &geo_types::GeometryCollection<T>) -> Self {
        let values: Vec<Feature> = geometry_collection
            .0
            .iter()
            .map(|geometry| crate::Geometry::new(GeometryValue::from(geometry)).into())
            .collect();

        FeatureCollection {
            bbox: None,
            features: values,
            foreign_members: None,
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<'a, T> From<&'a geo_types::Geometry<T>> for GeometryValue
where
    T: CoordFloat,
{
    /// Convert from `geo_types::Geometry` enums
    fn from(geometry: &'a geo_types::Geometry<T>) -> Self {
        match *geometry {
            geo_types::Geometry::Point(ref point) => GeometryValue::from(point),
            geo_types::Geometry::MultiPoint(ref multi_point) => GeometryValue::from(multi_point),
            geo_types::Geometry::LineString(ref line_string) => GeometryValue::from(line_string),
            geo_types::Geometry::Line(ref line) => GeometryValue::from(line),
            geo_types::Geometry::Triangle(ref triangle) => GeometryValue::from(triangle),
            geo_types::Geometry::Rect(ref rect) => GeometryValue::from(rect),
            geo_types::Geometry::GeometryCollection(ref gc) => GeometryValue::from(gc),
            geo_types::Geometry::MultiLineString(ref multi_line_string) => {
                GeometryValue::from(multi_line_string)
            }
            geo_types::Geometry::Polygon(ref polygon) => GeometryValue::from(polygon),
            geo_types::Geometry::MultiPolygon(ref multi_polygon) => {
                GeometryValue::from(multi_polygon)
            }
        }
    }
}

fn create_point_type<T>(point: &geo_types::Point<T>) -> PointType
where
    T: CoordFloat,
{
    let x: f64 = point.x().to_f64().unwrap();
    let y: f64 = point.y().to_f64().unwrap();
    crate::Position::from([x, y])
}

fn create_line_string_type<T>(line_string: &geo_types::LineString<T>) -> LineStringType
where
    T: CoordFloat,
{
    line_string
        .points()
        .map(|point| create_point_type(&point))
        .collect()
}

fn create_from_line_type<T>(line_string: &geo_types::Line<T>) -> LineStringType
where
    T: CoordFloat,
{
    vec![
        create_point_type(&line_string.start_point()),
        create_point_type(&line_string.end_point()),
    ]
}

fn create_from_triangle_type<T>(triangle: &geo_types::Triangle<T>) -> PolygonType
where
    T: CoordFloat,
{
    create_polygon_type(&triangle.to_polygon())
}

fn create_from_rect_type<T>(rect: &geo_types::Rect<T>) -> PolygonType
where
    T: CoordFloat,
{
    create_polygon_type(&rect.to_polygon())
}

fn create_multi_line_string_type<T>(
    multi_line_string: &geo_types::MultiLineString<T>,
) -> Vec<LineStringType>
where
    T: CoordFloat,
{
    multi_line_string
        .0
        .iter()
        .map(|line_string| create_line_string_type(line_string))
        .collect()
}

fn create_polygon_type<T>(polygon: &geo_types::Polygon<T>) -> PolygonType
where
    T: CoordFloat,
{
    let exterior: Vec<Position> = polygon
        .exterior()
        .points()
        .map(|point| create_point_type(&point))
        .collect();

    // If exterior is empty, return early to avoid creating [[]]
    if exterior.is_empty() {
        return vec![];
    }

    let mut coords = vec![exterior];
    coords.extend(
        polygon
            .interiors()
            .iter()
            .map(|line_string| create_line_string_type(line_string)),
    );

    coords
}

fn create_multi_polygon_type<T>(multi_polygon: &geo_types::MultiPolygon<T>) -> Vec<PolygonType>
where
    T: CoordFloat,
{
    multi_polygon
        .0
        .iter()
        .map(|polygon| create_polygon_type(polygon))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{GeoJson, Geometry, GeometryValue};
    use geo_types::{point, wkt, Point, Polygon};

    #[test]
    fn geo_point_conversion_test() {
        // Test with f32 coordinates
        let geo_point = Point::new(1.0f32, 2.0f32);
        let geojson_point = GeometryValue::from(&geo_point);
        assert_eq!(geojson_point, GeometryValue::new_point([1.0, 2.0]));

        // Test with f64 coordinates
        let geo_point = Point::new(1.0f64, 2.0f64);
        let geojson_point = GeometryValue::from(&geo_point);
        assert_eq!(geojson_point, GeometryValue::new_point([1.0, 2.0]));
    }

    #[test]
    fn geo_multi_point_conversion_test() {
        let geo_multi_point = wkt!(MULTIPOINT(1.0 2.0,3.0 4.0));
        let geojson_multi_point = GeometryValue::from(&geo_multi_point);
        assert_eq!(
            geojson_multi_point,
            GeometryValue::new_multi_point([[1.0, 2.0], [3.0, 4.0]])
        );
    }

    #[test]
    fn geo_line_string_conversion_test() {
        let geo_line_string = wkt!(LINESTRING(1.0 2.0,3.0 4.0));
        let geojson_line_string = GeometryValue::from(&geo_line_string);
        assert_eq!(
            geojson_line_string,
            GeometryValue::new_line_string([[1.0, 2.0], [3.0, 4.0]])
        );
    }

    #[test]
    fn geo_line_conversion_test() {
        let geo_line = wkt!(LINE(1.0 2.0,3.0 4.0));
        let geojson_line = GeometryValue::from(&geo_line);
        assert_eq!(
            geojson_line,
            GeometryValue::new_line_string([[1.0, 2.0], [3.0, 4.0]])
        );
    }

    #[test]
    fn geo_triangle_conversion_test() {
        let triangle = wkt!(TRIANGLE(0.0 0.0,2.0 0.0,1.0 1.0));
        let geojson_polygon = GeometryValue::from(&triangle);
        // Triangle closes the ring by repeating the first vertex
        assert_eq!(
            geojson_polygon,
            GeometryValue::new_polygon([[[0.0, 0.0], [2.0, 0.0], [1.0, 1.0], [0.0, 0.0]]])
        );
    }

    #[test]
    fn geo_rect_conversion_test() {
        // Same rect as geo_types::geometry::Rect::to_polygon doctest
        let rect = wkt!(RECT(0. 0.,1. 2.));
        let geojson_polygon = GeometryValue::from(&rect);
        let expected = GeometryValue::new_polygon([[
            [1.0, 0.0],
            [1.0, 2.0],
            [0.0, 2.0],
            [0.0, 0.0],
            [1.0, 0.0],
        ]]);
        assert_eq!(geojson_polygon, expected);
    }

    #[test]
    fn geo_multi_line_string_conversion_test() {
        let geo_multi_line_string = wkt!(MULTILINESTRING(
            (1.0 2.0,3.0 4.0),
            (5.0 6.0,7.0 8.0)
        ));
        let geojson_multi_line_point = GeometryValue::from(&geo_multi_line_string);
        let expected = GeometryValue::new_multi_line_string([
            [[1.0, 2.0], [3.0, 4.0]],
            [[5.0, 6.0], [7.0, 8.0]],
        ]);

        assert_eq!(geojson_multi_line_point, expected);
    }

    #[test]
    fn geo_polygon_conversion_test() {
        // Polygon with exterior ring and one interior hole
        let geo_polygon = wkt!(POLYGON(
            (0.0 0.0,4.0 0.0,4.0 4.0,0.0 4.0,0.0 0.0),
            (1.0 1.0,2.0 1.0,2.0 2.0,1.0 2.0,1.0 1.0)
        ));
        let geojson_polygon = GeometryValue::from(&geo_polygon);
        assert_eq!(
            geojson_polygon,
            GeometryValue::new_polygon([
                [[0.0, 0.0], [4.0, 0.0], [4.0, 4.0], [0.0, 4.0], [0.0, 0.0]],
                [[1.0, 1.0], [2.0, 1.0], [2.0, 2.0], [1.0, 2.0], [1.0, 1.0]],
            ])
        );
    }

    #[test]
    fn geo_empty_polygon_conversion_test() {
        // Test that an empty polygon serializes to coordinates: [] instead of coordinates: [[]]
        let geo_polygon: Polygon = Polygon::empty();
        let geojson_polygon = GeometryValue::from(&geo_polygon);

        let geometry = Geometry::new(geojson_polygon.clone());
        let json = serde_json::to_string(&geometry).unwrap();
        assert!(
            json.contains(r#""coordinates":[]"#),
            "Empty polygon should serialize to coordinates: [], got: {}",
            json
        );
    }

    #[test]
    fn geo_multi_polygon_conversion_test() {
        let geo_multi_polygon = wkt!(MULTIPOLYGON(
            ((0.0 0.0,1.0 0.0,1.0 1.0,0.0 0.0)),
            ((2.0 2.0,3.0 2.0,3.0 3.0,2.0 2.0))
        ));
        let geojson_multi_polygon = GeometryValue::from(&geo_multi_polygon);
        assert_eq!(
            geojson_multi_polygon,
            GeometryValue::new_multi_polygon([
                [[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 0.0]]],
                [[[2.0, 2.0], [3.0, 2.0], [3.0, 3.0], [2.0, 2.0]]],
            ])
        );
    }

    #[test]
    fn geo_geometry_collection_conversion_test() {
        let geo_geometry_collection = wkt!(GEOMETRYCOLLECTION(
            MULTIPOINT(0.0 0.0,1.0 1.0),
            MULTILINESTRING((0.0 0.0,1.0 1.0),(2.0 2.0,3.0 3.0)),
            MULTIPOLYGON(((0.0 0.0,1.0 0.0,1.0 1.0,0.0 0.0)))
        ));
        let geojson_geometry_collection = GeometryValue::from(&geo_geometry_collection);

        let GeometryValue::GeometryCollection { geometries } = geojson_geometry_collection else {
            panic!("Not valid geometry {:?}", geojson_geometry_collection);
        };

        assert_eq!(geometries[0].value.type_name(), "MultiPoint");
        assert_eq!(geometries[1].value.type_name(), "MultiLineString");
        assert_eq!(geometries[2].value.type_name(), "MultiPolygon");
    }

    #[test]
    fn test_from_geo_type_to_geojson() {
        let point = point!(x: 1.0, y: 2.0);
        let actual = serde_json::Value::from(GeoJson::from(&point));
        let expected = serde_json::json!({"coordinates": [1.0, 2.0], "type": "Point"});
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_iter_geo_type_to_geojson() {
        let p1 = point!(x: 1.0, y: 2.0);
        let p2 = point!(x: 3.0, y: 4.0);
        let points = [p1, p2];

        use std::iter::FromIterator;

        let actual = GeoJson::from_iter(points.iter());
        let actual2 = points.iter().collect::<GeoJson>();
        assert_eq!(actual, actual2);

        let expected = serde_json::json!({
            "type": "GeometryCollection",
            "geometries": [
                {"coordinates": [1.0, 2.0], "type": "Point"},
                {"coordinates": [3.0, 4.0], "type": "Point"},
            ]
        });
        assert_eq!(serde_json::Value::from(actual), expected);
    }
}
