mod coord;
mod geometry;
mod geometry_collection;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;

// These structures are needed because we can't implement traits on types like
// `geojson::PointType` because they are just type aliases of raw types like
// `Vec<f64>`.

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct PointType(crate::Position);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct LineStringType(crate::LineStringType);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct PolygonType(crate::PolygonType);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct MultiPointType(Vec<crate::PointType>);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct MultiLineStringType(Vec<crate::LineStringType>);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct MultiPolygonType(Vec<crate::PolygonType>);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
pub struct GeometryCollectionType(Vec<crate::Geometry>);
