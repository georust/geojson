use bytemuck::TransparentWrapper;
use geo_traits::{
    CoordTrait, Dimensions, GeometryCollectionTrait, GeometryTrait, LineStringTrait,
    MultiLineStringTrait, MultiPointTrait, MultiPolygonTrait, PolygonTrait,
};
use geo_traits::{UnimplementedLine, UnimplementedRect, UnimplementedTriangle};

mod coord;
mod geometry;
mod geometry_collection;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
#[doc(hidden)]
pub struct PointType(crate::Position);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
#[doc(hidden)]
pub struct LineStringType(crate::LineStringType);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
#[doc(hidden)]
pub struct PolygonType(crate::PolygonType);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
#[doc(hidden)]
pub struct MultiPointType(Vec<crate::PointType>);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
#[doc(hidden)]
pub struct MultiLineStringType(Vec<crate::LineStringType>);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
#[doc(hidden)]
pub struct MultiPolygonType(Vec<crate::PolygonType>);

#[derive(bytemuck::TransparentWrapper)]
#[repr(transparent)]
#[doc(hidden)]
pub struct GeometryCollectionType(Vec<crate::Geometry>);
