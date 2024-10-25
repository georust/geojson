use super::{LineStringType, PolygonType};
use bytemuck::TransparentWrapper;
use geo_traits::{Dimensions, LineStringTrait, PolygonTrait};

impl geo_traits::PolygonTrait for PolygonType {
    type T = f64;
    type RingType<'b>
        = &'b LineStringType
    where
        Self: 'b;

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        self.0.first().map(LineStringType::wrap_ref)
    }

    fn num_interiors(&self) -> usize {
        self.0.len() - 1
    }

    fn interior(&self, i: usize) -> Option<Self::RingType<'_>> {
        self.0.get(i + 1).map(LineStringType::wrap_ref)
    }

    fn dim(&self) -> Dimensions {
        self.exterior().unwrap().dim()
    }

    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_> {
        LineStringType::wrap_ref(self.0.get_unchecked(i + 1))
    }

    fn interiors(&self) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::RingType<'_>> {
        self.0.iter().skip(1).map(LineStringType::wrap_ref)
    }
}

impl<'a> geo_traits::PolygonTrait for &'a PolygonType {
    type T = f64;
    type RingType<'b>
        = &'b LineStringType
    where
        Self: 'b;

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        PolygonType::exterior(self)
    }

    fn num_interiors(&self) -> usize {
        PolygonType::num_interiors(self)
    }

    fn interior(&self, i: usize) -> Option<Self::RingType<'_>> {
        PolygonType::interior(self, i)
    }

    fn dim(&self) -> Dimensions {
        PolygonType::dim(self)
    }

    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_> {
        PolygonType::interior_unchecked(self, i)
    }

    fn interiors(&self) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::RingType<'_>> {
        PolygonType::interiors(self)
    }
}
