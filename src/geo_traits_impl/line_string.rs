use super::{LineStringType, PointType};
use bytemuck::TransparentWrapper;
use geo_traits::{Dimensions, LineStringTrait, PointTrait};

impl LineStringTrait for LineStringType {
    type T = f64;
    type CoordType<'b>
        = &'b PointType
    where
        Self: 'b;

    fn num_coords(&self) -> usize {
        self.0.len()
    }

    fn coord(&self, i: usize) -> Option<Self::CoordType<'_>> {
        Some(PointType::wrap_ref(&self.0[i]))
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        self.coord(i).unwrap()
    }

    fn dim(&self) -> Dimensions {
        self.coord(0).map_or(Dimensions::Unknown(0), |p| p.dim())
    }
}

impl geo_traits::LineStringTrait for &LineStringType {
    type T = f64;
    type CoordType<'b>
        = &'b PointType
    where
        Self: 'b;

    fn num_coords(&self) -> usize {
        LineStringType::num_coords(self)
    }

    fn coord(&self, i: usize) -> Option<Self::CoordType<'_>> {
        LineStringType::coord(self, i)
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        LineStringType::coord_unchecked(self, i)
    }

    fn dim(&self) -> Dimensions {
        LineStringType::dim(self)
    }
}
