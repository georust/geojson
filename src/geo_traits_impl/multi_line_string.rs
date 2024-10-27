use super::{LineStringType, MultiLineStringType};
use bytemuck::TransparentWrapper;
use geo_traits::{Dimensions, LineStringTrait};

impl geo_traits::MultiLineStringTrait for MultiLineStringType {
    type T = f64;
    type LineStringType<'b>
        = &'b LineStringType
    where
        Self: 'b;

    fn num_line_strings(&self) -> usize {
        self.0.len()
    }

    fn dim(&self) -> Dimensions {
        self.line_string(0).unwrap().dim()
    }

    fn line_string(&self, i: usize) -> Option<Self::LineStringType<'_>> {
        self.0.get(i).map(LineStringType::wrap_ref)
    }

    unsafe fn line_string_unchecked(&self, i: usize) -> Self::LineStringType<'_> {
        LineStringType::wrap_ref(self.0.get_unchecked(i))
    }

    fn line_strings(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::LineStringType<'_>> {
        self.0.iter().map(LineStringType::wrap_ref)
    }
}

impl<'a> geo_traits::MultiLineStringTrait for &'a MultiLineStringType {
    type T = f64;
    type LineStringType<'b>
        = &'b LineStringType
    where
        Self: 'b;

    fn num_line_strings(&self) -> usize {
        MultiLineStringType::num_line_strings(self)
    }

    fn dim(&self) -> Dimensions {
        MultiLineStringType::dim(self)
    }

    fn line_string(&self, i: usize) -> Option<Self::LineStringType<'_>> {
        MultiLineStringType::line_string(self, i)
    }

    unsafe fn line_string_unchecked(&self, i: usize) -> Self::LineStringType<'_> {
        MultiLineStringType::line_string_unchecked(self, i)
    }

    fn line_strings(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = Self::LineStringType<'_>> {
        MultiLineStringType::line_strings(self)
    }
}
