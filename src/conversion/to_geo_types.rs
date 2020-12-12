use crate::geo_types;

use crate::{geometry, Position};

use crate::Error as GJError;
use crate::{quick_collection, Feature, FeatureCollection, GeoJson, Geometry};
use num_traits::Float;
use std::convert::{TryFrom, TryInto};

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryInto<geo_types::Point<T>> for geometry::Value<Pos>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_into(self) -> Result<geo_types::Point<T>, Self::Error> {
        match self {
            geometry::Value::Point(point_type) => Ok(create_geo_point(point_type)),
            _ => Err(GJError::InvalidGeometryConversion(self)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryInto<geo_types::MultiPoint<T>> for geometry::Value<Pos>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_into(self) -> Result<geo_types::MultiPoint<T>, Self::Error> {
        match self {
            geometry::Value::MultiPoint(multi_point_type) => Ok(geo_types::MultiPoint(
                multi_point_type
                    .into_iter()
                    .map(|point_type| create_geo_point(point_type))
                    .collect(),
            )),
            _ => Err(GJError::InvalidGeometryConversion(self)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryInto<geo_types::LineString<T>> for geometry::Value<Pos>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_into(self) -> Result<geo_types::LineString<T>, Self::Error> {
        match self {
            geometry::Value::LineString(multi_point_type) => {
                Ok(create_geo_line_string(multi_point_type))
            }
            _ => Err(GJError::InvalidGeometryConversion(self)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryInto<geo_types::MultiLineString<T>> for geometry::Value<Pos>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_into(self) -> Result<geo_types::MultiLineString<T>, Self::Error> {
        match self {
            geometry::Value::MultiLineString(multi_line_string_type) => {
                Ok(create_geo_multi_line_string(multi_line_string_type))
            }
            _ => Err(GJError::InvalidGeometryConversion(self)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryInto<geo_types::Polygon<T>> for geometry::Value<Pos>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_into(self) -> Result<geo_types::Polygon<T>, Self::Error> {
        match self {
            geometry::Value::Polygon(polygon_type) => Ok(create_geo_polygon(polygon_type)),
            _ => Err(GJError::InvalidGeometryConversion(self)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryInto<geo_types::MultiPolygon<T>> for geometry::Value<Pos>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_into(self) -> Result<geo_types::MultiPolygon<T>, Self::Error> {
        match self {
            geometry::Value::MultiPolygon(multi_polygon_type) => {
                Ok(create_geo_multi_polygon(multi_polygon_type))
            }
            _ => Err(GJError::InvalidGeometryConversion(self)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryInto<geo_types::GeometryCollection<T>> for geometry::Value<Pos>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_into(self) -> Result<geo_types::GeometryCollection<T>, Self::Error> {
        match self {
            geometry::Value::GeometryCollection(geometries) => {
                let geojson_geometries = geometries
                    .into_iter()
                    .map(|geometry| geometry.value.try_into().unwrap())
                    .collect();

                Ok(geo_types::GeometryCollection(geojson_geometries))
            }
            _ => Err(GJError::InvalidGeometryConversion(self)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryInto<geo_types::Geometry<T>> for geometry::Value<Pos>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_into(self) -> Result<geo_types::Geometry<T>, Self::Error> {
        match self {
            geometry::Value::Point(point_type) => {
                Ok(geo_types::Geometry::Point(create_geo_point(point_type)))
            }
            geometry::Value::MultiPoint(multi_point_type) => Ok(geo_types::Geometry::MultiPoint(
                create_geo_multi_point(multi_point_type),
            )),
            geometry::Value::LineString(line_string_type) => Ok(geo_types::Geometry::LineString(
                create_geo_line_string(line_string_type),
            )),
            geometry::Value::MultiLineString(multi_line_string_type) => {
                Ok(geo_types::Geometry::MultiLineString(
                    create_geo_multi_line_string(multi_line_string_type),
                ))
            }
            geometry::Value::Polygon(polygon_type) => Ok(geo_types::Geometry::Polygon(
                create_geo_polygon(polygon_type),
            )),
            geometry::Value::MultiPolygon(multi_polygon_type) => Ok(
                geo_types::Geometry::MultiPolygon(create_geo_multi_polygon(multi_polygon_type)),
            ),
            _ => Err(GJError::InvalidGeometryConversion(self)),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryFrom<Geometry<Pos>> for geo_types::Geometry<T>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_from(val: Geometry<Pos>) -> Result<geo_types::Geometry<T>, Self::Error> {
        val.value.try_into()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryFrom<Feature<Pos>> for geo_types::Geometry<T>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_from(val: Feature<Pos>) -> Result<geo_types::Geometry<T>, Self::Error> {
        match val.geometry {
            None => Err(GJError::FeatureHasNoGeometry(val)),
            Some(geom) => geom.try_into(),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryFrom<FeatureCollection<Pos>> for geo_types::Geometry<T>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_from(val: FeatureCollection<Pos>) -> Result<geo_types::Geometry<T>, Self::Error> {
        Ok(geo_types::Geometry::GeometryCollection(quick_collection(
            &GeoJson::FeatureCollection(val),
        )?))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T, Pos: Position> TryFrom<GeoJson<Pos>> for geo_types::Geometry<T>
where
    T: Float,
{
    type Error = GJError<Pos>;

    fn try_from(val: GeoJson<Pos>) -> Result<geo_types::Geometry<T>, Self::Error> {
        match val {
            GeoJson::Geometry(geom) => geom.try_into(),
            GeoJson::Feature(feat) => feat.try_into(),
            GeoJson::FeatureCollection(fc) => fc.try_into(),
        }
    }
}

fn create_geo_coordinate<T, Pos: Position>(point_type: Pos) -> geo_types::Coordinate<T>
where
    T: Float,
{
    geo_types::Coordinate {
        x: T::from(point_type.x()).unwrap(),
        y: T::from(point_type.y()).unwrap(),
    }
}

fn create_geo_point<T, Pos: Position>(point_type: Pos) -> geo_types::Point<T>
where
    T: Float,
{
    geo_types::Point::new(
        T::from(point_type.x()).unwrap(),
        T::from(point_type.y()).unwrap(),
    )
}

fn create_geo_multi_point<T, Pos: Position>(multi_point: Vec<Pos>) -> geo_types::MultiPoint<T>
where
    T: Float,
{
    geo_types::MultiPoint(
        multi_point
            .into_iter()
            .map(|point_type| create_geo_point(point_type))
            .collect(),
    )
}

fn create_geo_line_string<T, Pos: Position>(line_type: Vec<Pos>) -> geo_types::LineString<T>
where
    T: Float,
{
    geo_types::LineString(
        line_type
            .into_iter()
            .map(|point_type| create_geo_coordinate(point_type))
            .collect(),
    )
}

fn create_geo_multi_line_string<T, Pos: Position>(
    multi_line_type: Vec<Vec<Pos>>,
) -> geo_types::MultiLineString<T>
where
    T: Float,
{
    geo_types::MultiLineString(
        multi_line_type
            .into_iter()
            .map(|point_type| create_geo_line_string(point_type))
            .collect(),
    )
}

fn create_geo_polygon<T, Pos: Position>(mut polygon_type: Vec<Vec<Pos>>) -> geo_types::Polygon<T>
where
    T: Float,
{
    let exterior: geo_types::LineString<T> = if polygon_type.is_empty() {
        geo_types::LineString(vec![])
    } else {
        create_geo_line_string(polygon_type.remove(0))
    };

    let interiors = polygon_type
        .into_iter()
        .map(|line_string_type| create_geo_line_string(line_string_type))
        .collect();

    geo_types::Polygon::new(exterior, interiors)
}

fn create_geo_multi_polygon<T, Pos: Position>(
    multi_polygon_type: Vec<Vec<Vec<Pos>>>,
) -> geo_types::MultiPolygon<T>
where
    T: Float,
{
    geo_types::MultiPolygon(
        multi_polygon_type
            .into_iter()
            .map(|polygon_type| create_geo_polygon(polygon_type))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::{Geometry, Value};
    use geo_types;

    use std::convert::TryInto;

    #[test]
    fn geojson_point_conversion_test() {
        let coords = vec![100.0, 0.2];
        let geojson_point = Value::Point(coords.clone());
        let geo_point: geo_types::Point<f64> = geojson_point.try_into().unwrap();

        assert_almost_eq!(geo_point.x(), coords[0], 1e-6);
        assert_almost_eq!(geo_point.y(), coords[1], 1e-6);
    }

    #[test]
    fn geojson_multi_point_conversion_test() {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let geojson_multi_point = Value::MultiPoint(vec![coord1.clone(), coord2.clone()]);
        let geo_multi_point: geo_types::MultiPoint<f64> = geojson_multi_point.try_into().unwrap();

        assert_almost_eq!(geo_multi_point.0[0].x(), coord1[0], 1e-6);
        assert_almost_eq!(geo_multi_point.0[0].y(), coord1[1], 1e-6);
        assert_almost_eq!(geo_multi_point.0[1].x(), coord2[0], 1e-6);
        assert_almost_eq!(geo_multi_point.0[1].y(), coord2[1], 1e-6);
    }

    #[test]
    fn geojson_line_string_conversion_test() {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let geojson_line_string = Value::LineString(vec![coord1.clone(), coord2.clone()]);
        let geo_line_string: geo_types::LineString<f64> = geojson_line_string.try_into().unwrap();

        assert_almost_eq!(geo_line_string.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string.0[1].y, coord2[1], 1e-6);
    }

    #[test]
    fn geojson_multi_line_string_conversion_test() {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![102.0, 0.8];
        let geojson_multi_line_string = Value::MultiLineString(vec![
            vec![coord1.clone(), coord2.clone()],
            vec![coord2.clone(), coord3.clone()],
        ]);
        let geo_multi_line_string: geo_types::MultiLineString<f64> =
            geojson_multi_line_string.try_into().unwrap();

        let ref geo_line_string1 = geo_multi_line_string.0[0];
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);

        let ref geo_line_string2 = geo_multi_line_string.0[1];
        assert_almost_eq!(geo_line_string2.0[0].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[0].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].y, coord3[1], 1e-6);
    }

    #[test]
    fn geojson_polygon_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![101.0, 1.0];
        let coord4 = vec![104.0, 0.2];
        let coord5 = vec![100.9, 0.2];
        let coord6 = vec![100.9, 0.7];

        let geojson_multi_line_string_type1 = vec![
            vec![
                coord1.clone(),
                coord2.clone(),
                coord3.clone(),
                coord1.clone(),
            ],
            vec![
                coord4.clone(),
                coord5.clone(),
                coord6.clone(),
                coord4.clone(),
            ],
        ];
        let geojson_polygon = Value::Polygon(geojson_multi_line_string_type1);
        let geo_polygon: geo_types::Polygon<f64> = geojson_polygon.try_into().unwrap();

        let ref geo_line_string1 = geo_polygon.exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        let ref geo_line_string2 = geo_polygon.interiors()[0];
        assert_almost_eq!(geo_line_string2.0[0].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[0].y, coord4[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].x, coord5[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].y, coord5[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].x, coord6[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].y, coord6[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].y, coord4[1], 1e-6);
    }

    #[test]
    fn geojson_empty_polygon_conversion_test() {
        let geojson_polygon = Value::<(f64, f64)>::Polygon(vec![]);
        let geo_polygon: geo_types::Polygon<f64> = geojson_polygon.try_into().unwrap();

        assert!(geo_polygon.exterior().0.is_empty());
    }

    #[test]
    fn geojson_polygon_without_interiors_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![101.0, 1.0];

        let geojson_multi_line_string_type1 = vec![vec![
            coord1.clone(),
            coord2.clone(),
            coord3.clone(),
            coord1.clone(),
        ]];
        let geojson_polygon = Value::Polygon(geojson_multi_line_string_type1);
        let geo_polygon: geo_types::Polygon<f64> = geojson_polygon.try_into().unwrap();

        let ref geo_line_string1 = geo_polygon.exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        assert_eq!(0, geo_polygon.interiors().len());
    }

    #[test]
    fn geojson_multi_polygon_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![101.0, 1.0];
        let coord4 = vec![104.0, 0.2];
        let coord5 = vec![100.9, 0.2];
        let coord6 = vec![100.9, 0.7];

        let geojson_line_string_type1 = vec![
            coord1.clone(),
            coord2.clone(),
            coord3.clone(),
            coord1.clone(),
        ];

        let geojson_line_string_type2 = vec![
            coord4.clone(),
            coord5.clone(),
            coord6.clone(),
            coord4.clone(),
        ];
        let geojson_multi_polygon = Value::MultiPolygon(vec![
            vec![geojson_line_string_type1],
            vec![geojson_line_string_type2],
        ]);
        let geo_multi_polygon: geo_types::MultiPolygon<f64> =
            geojson_multi_polygon.try_into().unwrap();

        let ref geo_line_string1 = geo_multi_polygon.0[0].exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        let ref geo_line_string2 = geo_multi_polygon.0[1].exterior();
        assert_almost_eq!(geo_line_string2.0[0].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[0].y, coord4[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].x, coord5[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].y, coord5[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].x, coord6[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].y, coord6[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].y, coord4[1], 1e-6);
    }

    #[test]
    fn geojson_geometry_collection_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![100.0, 1.0];
        let coord3 = vec![101.0, 1.0];
        let coord4 = vec![102.0, 0.0];
        let coord5 = vec![101.0, 0.0];

        let geojson_multi_point = Value::MultiPoint(vec![coord1.clone(), coord2.clone()]);
        let geojson_multi_line_string = Value::MultiLineString(vec![
            vec![coord1.clone(), coord2.clone()],
            vec![coord2.clone(), coord3.clone()],
        ]);
        let geojson_multi_polygon = Value::MultiPolygon(vec![
            vec![vec![
                coord3.clone(),
                coord4.clone(),
                coord5.clone(),
                coord3.clone(),
            ]],
            vec![vec![
                coord1.clone(),
                coord5.clone(),
                coord3.clone(),
                coord1.clone(),
            ]],
        ]);

        let geojson_geometry_collection = Value::GeometryCollection(vec![
            Geometry::new(geojson_multi_point),
            Geometry::new(geojson_multi_line_string),
            Geometry::new(geojson_multi_polygon),
        ]);

        let geo_geometry_collection: geo_types::GeometryCollection<f64> =
            geojson_geometry_collection.try_into().unwrap();

        assert_eq!(3, geo_geometry_collection.0.len());
    }

    #[test]
    fn geojson_geometry_conversion() {
        let coords = vec![100.0, 0.2];
        let geojson_geometry = Geometry::from(Value::Point(coords.clone()));
        let geo_geometry: geo_types::Geometry<f64> = geojson_geometry
            .try_into()
            .expect("Shoudl be able to convert to geo_types::Geometry");
        let geo_point: geo_types::Point<_> =
            geo_geometry.try_into().expect("this should be a point");
        assert_almost_eq!(geo_point.x(), coords[0], 1e-6);
        assert_almost_eq!(geo_point.y(), coords[1], 1e-6);
    }
}
