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

    fn coords(&self) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::CoordType<'_>> {
        self.0.iter().map(PointType::wrap_ref)
    }

    fn dim(&self) -> Dimensions {
        self.coord(0).unwrap().dim() // TODO: is this okay?
    }
}

impl<'a> geo_traits::LineStringTrait for &'a LineStringType {
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

    fn coords(&self) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::CoordType<'_>> {
        LineStringType::coords(self)
    }

    fn dim(&self) -> Dimensions {
        LineStringType::dim(self)
    }
}
