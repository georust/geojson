use geo_types::{self, CoordFloat};

use crate::GeometryValue;

use crate::{Error, Result};
use crate::{Feature, FeatureCollection, GeoJson, LineStringType, PointType, PolygonType};
use std::convert::{TryFrom, TryInto};

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&GeometryValue> for geo_types::Point<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &GeometryValue) -> Result<Self> {
        match value {
            GeometryValue::Point { coordinates } => Ok(create_geo_point(coordinates)),
            other => Err(mismatch_geom_err("Point", other)),
        }
    }
}
try_from_owned_value!(geo_types::Point<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&GeometryValue> for geo_types::MultiPoint<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &GeometryValue) -> Result<Self> {
        match value {
            GeometryValue::MultiPoint { coordinates } => Ok(geo_types::MultiPoint(
                coordinates
                    .iter()
                    .map(|point_type| create_geo_point(point_type))
                    .collect(),
            )),
            other => Err(mismatch_geom_err("MultiPoint", other)),
        }
    }
}
try_from_owned_value!(geo_types::MultiPoint<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&GeometryValue> for geo_types::LineString<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &GeometryValue) -> Result<Self> {
        match value {
            GeometryValue::LineString { coordinates } => Ok(create_geo_line_string(coordinates)),
            other => Err(mismatch_geom_err("LineString", other)),
        }
    }
}
try_from_owned_value!(geo_types::LineString<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&GeometryValue> for geo_types::MultiLineString<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &GeometryValue) -> Result<Self> {
        match value {
            GeometryValue::MultiLineString { coordinates } => {
                Ok(create_geo_multi_line_string(coordinates))
            }
            other => Err(mismatch_geom_err("MultiLineString", other)),
        }
    }
}
try_from_owned_value!(geo_types::MultiLineString<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&GeometryValue> for geo_types::Polygon<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &GeometryValue) -> Result<Self> {
        match value {
            GeometryValue::Polygon { coordinates } => Ok(create_geo_polygon(coordinates)),
            other => Err(mismatch_geom_err("Polygon", other)),
        }
    }
}
try_from_owned_value!(geo_types::Polygon<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&GeometryValue> for geo_types::MultiPolygon<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &GeometryValue) -> Result<geo_types::MultiPolygon<T>> {
        match value {
            GeometryValue::MultiPolygon { coordinates } => {
                Ok(create_geo_multi_polygon(coordinates))
            }
            other => Err(mismatch_geom_err("MultiPolygon", other)),
        }
    }
}
try_from_owned_value!(geo_types::MultiPolygon<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&GeometryValue> for geo_types::GeometryCollection<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &GeometryValue) -> Result<Self> {
        match value {
            GeometryValue::GeometryCollection { geometries } => {
                let geojson_geometries = geometries
                    .iter()
                    .map(|geometry| (&geometry.value).try_into().unwrap())
                    .collect();

                Ok(geo_types::GeometryCollection(geojson_geometries))
            }
            other => Err(mismatch_geom_err("GeometryCollection", other)),
        }
    }
}
try_from_owned_value!(geo_types::GeometryCollection<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&GeometryValue> for geo_types::Geometry<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &GeometryValue) -> Result<Self> {
        match &value {
            GeometryValue::Point { coordinates } => {
                Ok(geo_types::Geometry::Point(create_geo_point(coordinates)))
            }
            GeometryValue::MultiPoint { coordinates } => {
                Ok(geo_types::Geometry::MultiPoint(geo_types::MultiPoint(
                    coordinates
                        .iter()
                        .map(|point_type| create_geo_point(point_type))
                        .collect(),
                )))
            }
            GeometryValue::LineString { coordinates } => Ok(geo_types::Geometry::LineString(
                create_geo_line_string(coordinates),
            )),
            GeometryValue::MultiLineString { coordinates } => Ok(
                geo_types::Geometry::MultiLineString(create_geo_multi_line_string(coordinates)),
            ),
            GeometryValue::Polygon { coordinates } => Ok(geo_types::Geometry::Polygon(
                create_geo_polygon(coordinates),
            )),
            GeometryValue::MultiPolygon { coordinates } => Ok(geo_types::Geometry::MultiPolygon(
                create_geo_multi_polygon(coordinates),
            )),
            GeometryValue::GeometryCollection { geometries } => {
                let gc = geo_types::Geometry::GeometryCollection(geo_types::GeometryCollection(
                    geometries
                        .iter()
                        .cloned()
                        .map(|geom| geom.try_into())
                        .collect::<Result<Vec<geo_types::Geometry<T>>>>()?,
                ));
                Ok(gc)
            }
        }
    }
}
try_from_owned_value!(geo_types::Geometry<T>);

macro_rules! impl_try_from_geom_value {
    ($($kind:ident),*) => {
        $(
            #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
            impl<T> TryFrom<&$crate::Geometry> for geo_types::$kind<T>
            where
                T: CoordFloat,
            {
                type Error = Error;

                fn try_from(geometry: &crate::Geometry) -> Result<Self> {
                    Self::try_from(&geometry.value)
                }
            }

            #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
            impl<T> TryFrom<$crate::Geometry> for geo_types::$kind<T>
            where
                T: CoordFloat,
            {
                type Error = Error;

                fn try_from(geometry: crate::Geometry) -> Result<Self> {
                    Self::try_from(geometry.value)
                }
            }

            #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
            impl<T> TryFrom<$crate::Feature> for geo_types::$kind<T>
            where
                T: CoordFloat,
            {
                type Error = Error;

                fn try_from(val: Feature) -> Result<Self> {
                    match val.geometry {
                        None => Err(Error::FeatureHasNoGeometry(Box::new(val))),
                        Some(geom) => geom.try_into(),
                    }
                }
            }
        )*
    }
}

impl_try_from_geom_value![
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    Geometry,
    GeometryCollection
];

impl<T: CoordFloat> TryFrom<&GeoJson> for geo_types::GeometryCollection<T> {
    type Error = Error;

    /// Process top-level `GeoJSON` items, returning a geo_types::GeometryCollection or an Error
    fn try_from(gj: &GeoJson) -> Result<geo_types::GeometryCollection<T>>
    where
        T: CoordFloat,
    {
        match gj {
            GeoJson::FeatureCollection(collection) => Ok(geo_types::GeometryCollection(
                collection
                    .features
                    .iter()
                    // Only pass on non-empty geometries
                    .filter_map(|feature| feature.geometry.as_ref())
                    .map(|geometry| geometry.clone().try_into())
                    .collect::<Result<_>>()?,
            )),
            GeoJson::Feature(feature) => {
                if let Some(geometry) = &feature.geometry {
                    Ok(geo_types::GeometryCollection(vec![geometry
                        .clone()
                        .try_into()?]))
                } else {
                    Ok(geo_types::GeometryCollection(vec![]))
                }
            }
            GeoJson::Geometry(geometry) => Ok(geo_types::GeometryCollection(vec![geometry
                .clone()
                .try_into()?])),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<FeatureCollection> for geo_types::Geometry<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(val: FeatureCollection) -> Result<geo_types::Geometry<T>> {
        Ok(geo_types::Geometry::GeometryCollection(
            geo_types::GeometryCollection::try_from(&GeoJson::FeatureCollection(val))?,
        ))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<GeoJson> for geo_types::Geometry<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(val: GeoJson) -> Result<geo_types::Geometry<T>> {
        match val {
            GeoJson::Geometry(geom) => geom.try_into(),
            GeoJson::Feature(feat) => feat.try_into(),
            GeoJson::FeatureCollection(fc) => fc.try_into(),
        }
    }
}

fn create_geo_coordinate<T>(point_type: &PointType) -> geo_types::Coord<T>
where
    T: CoordFloat,
{
    geo_types::Coord {
        x: T::from(point_type[0]).unwrap(),
        y: T::from(point_type[1]).unwrap(),
    }
}

fn create_geo_point<T>(point_type: &PointType) -> geo_types::Point<T>
where
    T: CoordFloat,
{
    geo_types::Point::new(
        T::from(point_type[0]).unwrap(),
        T::from(point_type[1]).unwrap(),
    )
}

fn create_geo_line_string<T>(line_type: &LineStringType) -> geo_types::LineString<T>
where
    T: CoordFloat,
{
    geo_types::LineString(
        line_type
            .iter()
            .map(|point_type| create_geo_coordinate(point_type))
            .collect(),
    )
}

fn create_geo_multi_line_string<T>(
    multi_line_type: &[LineStringType],
) -> geo_types::MultiLineString<T>
where
    T: CoordFloat,
{
    geo_types::MultiLineString(
        multi_line_type
            .iter()
            .map(|point_type| create_geo_line_string(point_type))
            .collect(),
    )
}

fn create_geo_polygon<T>(polygon_type: &PolygonType) -> geo_types::Polygon<T>
where
    T: CoordFloat,
{
    let exterior = polygon_type
        .first()
        .map(|e| create_geo_line_string(e))
        .unwrap_or_else(|| create_geo_line_string(&vec![]));

    let interiors = if polygon_type.len() < 2 {
        vec![]
    } else {
        polygon_type[1..]
            .iter()
            .map(|line_string_type| create_geo_line_string(line_string_type))
            .collect()
    };

    geo_types::Polygon::new(exterior, interiors)
}

fn create_geo_multi_polygon<T>(multi_polygon_type: &[PolygonType]) -> geo_types::MultiPolygon<T>
where
    T: CoordFloat,
{
    geo_types::MultiPolygon(
        multi_polygon_type
            .iter()
            .map(|polygon_type| create_geo_polygon(polygon_type))
            .collect(),
    )
}

fn mismatch_geom_err(expected_type: &'static str, found: &GeometryValue) -> Error {
    Error::InvalidGeometryConversion {
        expected_type,
        found_type: found.type_name(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Geometry, GeometryValue};
    use serde_json::json;

    use geo_types::{coord, point};
    use std::convert::TryInto;

    #[test]
    fn geojson_point_conversion_test() {
        let geojson_point = GeometryValue::new_point([100.0, 0.2]);
        let geo_point: geo_types::Point<f64> = geojson_point.try_into().unwrap();

        assert_eq!(geo_point, point!(x: 100.0,  y: 0.2));
    }

    #[test]
    fn geojson_multi_point_conversion_test() {
        let geojson_multi_point = GeometryValue::new_multi_point([[100.0, 0.2], [101.0, 1.0]]);
        let geo_multi_point: geo_types::MultiPoint<f64> = geojson_multi_point.try_into().unwrap();

        assert_eq!(geo_multi_point.0[0], point!(x: 100.0, y: 0.2));
        assert_eq!(geo_multi_point.0[1], point!(x: 101.0, y: 1.0));
    }

    #[test]
    fn geojson_line_string_conversion_test() {
        let geojson_line_string = GeometryValue::new_line_string([[100.0, 0.2], [101.0, 1.0]]);
        let geo_line_string: geo_types::LineString<f64> = geojson_line_string.try_into().unwrap();

        assert_eq!(geo_line_string[0], coord!(x: 100.0, y: 0.2));
        assert_eq!(geo_line_string[1], coord!(x: 101.0, y: 1.0));
    }

    #[test]
    fn geojson_multi_line_string_conversion_test() {
        let geojson_multi_line_string = GeometryValue::new_multi_line_string([
            [[100.0, 0.2], [101.0, 1.0]],
            [[101.0, 1.0], [102.0, 0.8]],
        ]);
        let geo_multi_line_string: geo_types::MultiLineString<f64> =
            geojson_multi_line_string.try_into().unwrap();

        let geo_line_string1 = &geo_multi_line_string.0[0];
        assert_eq!(geo_line_string1[0], coord!(x: 100.0, y: 0.2));
        assert_eq!(geo_line_string1[1], coord!(x: 101.0, y: 1.0));

        let geo_line_string2 = &geo_multi_line_string.0[1];
        assert_eq!(geo_line_string2[0], coord!(x: 101.0, y: 1.0));
        assert_eq!(geo_line_string2[1], coord!(x: 102.0, y: 0.8));
    }

    #[test]
    fn geojson_polygon_conversion_test() {
        let geojson_polygon = GeometryValue::new_polygon([
            [[100.0, 0.0], [101.0, 1.0], [101.0, 1.0], [100.0, 0.0]],
            [[104.0, 0.2], [100.9, 0.2], [100.9, 0.7], [104.0, 0.2]],
        ]);
        let geo_polygon: geo_types::Polygon<f64> = geojson_polygon.try_into().unwrap();

        let exterior = geo_polygon.exterior();
        assert_eq!(exterior[0], coord!(x: 100.0, y: 0.0));
        assert_eq!(exterior[1], coord!(x: 101.0, y: 1.0));
        assert_eq!(exterior[2], coord!(x: 101.0, y: 1.0));
        assert_eq!(exterior[3], coord!(x: 100.0, y: 0.0));

        let interior = &geo_polygon.interiors()[0];
        assert_eq!(interior[0], coord!(x: 104.0, y: 0.2));
        assert_eq!(interior[1], coord!(x: 100.9, y: 0.2));
        assert_eq!(interior[2], coord!(x: 100.9, y: 0.7));
        assert_eq!(interior[3], coord!(x: 104.0, y: 0.2));
    }

    #[test]
    fn geojson_empty_polygon_conversion_test() {
        let geojson_polygon = GeometryValue::new_polygon(Vec::<Vec<[f64; 2]>>::new());
        let geo_polygon: geo_types::Polygon<f64> = geojson_polygon.try_into().unwrap();

        assert!(geo_polygon.exterior().0.is_empty());
    }

    #[test]
    fn geojson_polygon_without_interiors_conversion_test() {
        let geojson_polygon =
            GeometryValue::new_polygon([[[100.0, 0.0], [101.0, 1.0], [101.0, 1.0], [100.0, 0.0]]]);
        let geo_polygon: geo_types::Polygon<f64> = geojson_polygon.try_into().unwrap();

        let exterior = geo_polygon.exterior();
        assert_eq!(exterior[0], coord!(x: 100.0, y: 0.0));
        assert_eq!(exterior[1], coord!(x: 101.0, y: 1.0));
        assert_eq!(exterior[2], coord!(x: 101.0, y: 1.0));
        assert_eq!(exterior[3], coord!(x: 100.0, y: 0.0));

        assert_eq!(0, geo_polygon.interiors().len());
    }

    #[test]
    fn geojson_multi_polygon_conversion_test() {
        let geojson_multi_polygon = GeometryValue::new_multi_polygon([
            [[[100.0, 0.0], [101.0, 1.0], [101.0, 1.0], [100.0, 0.0]]],
            [[[104.0, 0.2], [100.9, 0.2], [100.9, 0.7], [104.0, 0.2]]],
        ]);
        let geo_multi_polygon: geo_types::MultiPolygon<f64> =
            geojson_multi_polygon.try_into().unwrap();

        let exterior1 = geo_multi_polygon.0[0].exterior();
        assert_eq!(exterior1[0], coord!(x: 100.0, y: 0.0));
        assert_eq!(exterior1[1], coord!(x: 101.0, y: 1.0));
        assert_eq!(exterior1[2], coord!(x: 101.0, y: 1.0));
        assert_eq!(exterior1[3], coord!(x: 100.0, y: 0.0));

        let exterior2 = geo_multi_polygon.0[1].exterior();
        assert_eq!(exterior2[0], coord!(x: 104.0, y: 0.2));
        assert_eq!(exterior2[1], coord!(x: 100.9, y: 0.2));
        assert_eq!(exterior2[2], coord!(x: 100.9, y: 0.7));
        assert_eq!(exterior2[3], coord!(x: 104.0, y: 0.2));
    }

    #[test]
    fn geojson_geometry_collection_conversion_test() {
        let geojson_multi_point = GeometryValue::new_multi_point([[100.0, 0.0], [100.0, 1.0]]);
        let geojson_multi_line_string = GeometryValue::new_multi_line_string([
            [[100.0, 0.0], [100.0, 1.0]],
            [[100.0, 1.0], [101.0, 1.0]],
        ]);
        let geojson_multi_polygon = GeometryValue::new_multi_polygon([
            [[[101.0, 1.0], [102.0, 0.0], [101.0, 0.0], [101.0, 1.0]]],
            [[[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 0.0]]],
        ]);

        let geojson_geometry_collection = GeometryValue::new_geometry_collection([
            geojson_multi_point,
            geojson_multi_line_string,
            geojson_multi_polygon,
        ]);

        let geo_geometry_collection: geo_types::GeometryCollection<f64> =
            geojson_geometry_collection.try_into().unwrap();

        assert_eq!(3, geo_geometry_collection.0.len());
    }

    #[test]
    fn geojson_geometry_conversion() {
        let geojson_geometry = Geometry::from(GeometryValue::new_point([100.0, 0.2]));
        let geo_geometry: geo_types::Geometry<f64> = geojson_geometry
            .try_into()
            .expect("Should be able to convert to geo_types::Geometry");
        let geo_point: geo_types::Point<_> =
            geo_geometry.try_into().expect("this should be a point");
        assert_eq!(geo_point, point!(x: 100.0, y: 0.2));
    }

    #[test]
    fn geojson_mismatch_geometry_conversion_test() {
        let geojson_line_string = GeometryValue::new_line_string([[100.0, 0.2], [101.0, 1.0]]);
        use std::convert::TryFrom;
        let error = geo_types::Point::<f64>::try_from(geojson_line_string).unwrap_err();
        assert_eq!(
            "Expected type: `Point`, but found `LineString`",
            format!("{}", error)
        )
    }

    #[test]
    fn feature_collection_with_geom_collection() {
        let geojson_str = json!({
            "type": "FeatureCollection",
            "features": [
            {
                "type": "Feature",
                "geometry": {
                    "type": "GeometryCollection",
                    "geometries": [
                    {
                        "type": "Polygon",
                        "coordinates": [
                            [
                                [1.0, 1.0],
                                [2.0, 2.0],
                                [3.0, 1.0],
                                [1.0, 1.0]
                            ]
                        ]
                    }
                    ]
                },
                "properties": {}
            }
            ]
        })
        .to_string();
        let geojson: crate::GeoJson = geojson_str.parse().unwrap();
        let mut geojson_feature_collection: crate::FeatureCollection = geojson.try_into().unwrap();
        let feature: crate::Feature = geojson_feature_collection.features.remove(0);

        use std::convert::TryFrom;
        let geo_geom = geo_types::Geometry::try_from(feature).unwrap();

        let expected =
            geo_types::Geometry::GeometryCollection(geo_types::GeometryCollection(vec![
                geo_types::Geometry::Polygon(geo_types::Polygon::new(
                    geo_types::LineString::new(vec![
                        coord!(x: 1.0, y: 1.0),
                        coord!(x: 2.0, y: 2.0),
                        coord!(x: 3.0, y: 1.0),
                        coord!(x: 1.0, y: 1.0),
                    ]),
                    vec![],
                )),
            ]));
        assert_eq!(geo_geom, expected);
    }

    #[test]
    fn borrowed_value_conversions_test() -> crate::Result<()> {
        let geojson_point = GeometryValue::new_point([100.0, 0.2]);
        let _: geo_types::Point<f64> = (&geojson_point).try_into()?;

        let geojson_multi_point = GeometryValue::new_multi_point([[100.0, 0.2], [101.0, 1.0]]);
        let _: geo_types::MultiPoint<f64> = (&geojson_multi_point).try_into()?;

        let geojson_line_string = GeometryValue::new_line_string([[100.0, 0.2], [101.0, 1.0]]);
        let _: geo_types::LineString<f64> = (&geojson_line_string).try_into()?;

        let geojson_multi_line_string = GeometryValue::new_multi_line_string([
            [[100.0, 0.2], [101.0, 1.0]],
            [[101.0, 1.0], [102.0, 0.8]],
        ]);
        let _: geo_types::MultiLineString<f64> = (&geojson_multi_line_string).try_into()?;

        let geojson_polygon = GeometryValue::new_polygon([
            [[100.0, 0.2], [101.0, 1.0], [102.0, 0.8], [100.0, 0.2]],
            [[104.0, 0.2], [100.0, 0.2], [101.0, 1.0], [104.0, 0.2]],
        ]);
        let _: geo_types::Polygon<f64> = (&geojson_polygon).try_into()?;

        let geojson_multi_polygon = GeometryValue::new_multi_polygon([
            [[[100.0, 0.2], [101.0, 1.0], [102.0, 0.8], [100.0, 0.2]]],
            [[[104.0, 0.2], [102.0, 0.8], [101.0, 1.0], [104.0, 0.2]]],
        ]);
        let _: geo_types::MultiPolygon<f64> = (&geojson_multi_polygon).try_into()?;

        let geojson_geometry_collection = GeometryValue::new_geometry_collection([
            geojson_multi_point,
            geojson_multi_line_string,
            geojson_multi_polygon,
        ]);
        let _: geo_types::GeometryCollection<f64> = (&geojson_geometry_collection).try_into()?;

        Ok(())
    }
}
