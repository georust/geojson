use serde::de::SeqAccess;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;
use tinyvec::TinyVec;

/// Positions
///
/// [GeoJSON Format Specification § 3.1.1](https://tools.ietf.org/html/rfc7946#section-3.1.1)
///
/// ## Examples
/// ```
/// use geojson::Position;
/// let position_1 = Position::from([1.0, 2.0]);
/// assert_eq!(position_1[0], 1.0);
/// assert_eq!(position_1.as_slice(), &[1.0, 2.0]);
///
/// let position_2 = Position::from(vec![3.0, 4.0]);
/// assert_eq!(position_2[1], 4.0);
/// assert_eq!(position_2.as_slice(), &[3.0, 4.0]);
/// ```
///
/// As always, an out of bound access will panic.
/// ```
/// use geojson::Position;
/// let position_2d = Position::from([1.0, 2.0]);
/// // panics!
/// // let z = position_2d[2];
/// let position_3d = Position::from(vec![1.0, 2.0, 3.0]);
/// let z = position_3d[2];
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Position<PB: PositionBuffer = TinyVec<[f64; 2]>>(PB);

impl<PB: PositionBuffer> Position<PB> {
    pub fn as_slice(&self) -> &[f64] {
        self.0.as_slice()
    }

    pub fn as_slice_mut(&mut self) -> &mut [f64] {
        self.0.as_slice_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn from_values(floats: PB) -> Self {
        Self(floats)
    }
}

impl<I: SliceIndex<[f64]>, PB: PositionBuffer> Index<I> for Position<PB> {
    type Output = <I as SliceIndex<[f64]>>::Output;
    #[inline(always)]
    fn index(&self, index: I) -> &Self::Output {
        &self.0.as_slice()[index]
    }
}

impl<I: SliceIndex<[f64]>, PB: PositionBuffer> IndexMut<I> for Position<PB> {
    #[inline(always)]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0.as_slice_mut()[index]
    }
}

impl From<TinyVec<[f64; 2]>> for Position {
    fn from(value: TinyVec<[f64; 2]>) -> Self {
        Self(value)
    }
}

impl From<Vec<f64>> for Position {
    fn from(value: Vec<f64>) -> Self {
        Self(TinyVec::Heap(value))
    }
}

impl From<[f64; 2]> for Position {
    fn from(value: [f64; 2]) -> Self {
        Self(TinyVec::Inline(value.into()))
    }
}

impl From<(f64, f64)> for Position {
    fn from(value: (f64, f64)) -> Self {
        Self::from([value.0, value.1])
    }
}

impl From<[f64; 3]> for Position {
    fn from(value: [f64; 3]) -> Self {
        Self(TinyVec::Heap(value.into()))
    }
}

impl From<(f64, f64, f64)> for Position {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::from([value.0, value.1, value.2])
    }
}

impl From<[f64; 4]> for Position {
    fn from(value: [f64; 4]) -> Self {
        Self(TinyVec::Heap(value.into()))
    }
}

impl From<(f64, f64, f64, f64)> for Position {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self::from([value.0, value.1, value.2, value.3])
    }
}

pub trait PositionBuffer: private::Sealed {
    fn as_slice(&self) -> &[f64];
    fn as_slice_mut(&mut self) -> &mut [f64];

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn from_seq<'de, S>(first: f64, seq: S) -> Result<Self, S::Error>
    where
        Self: Sized,
        S: SeqAccess<'de>;
}

impl<const INLINE_SIZE: usize> PositionBuffer for TinyVec<[f64; INLINE_SIZE]> {
    fn as_slice(&self) -> &[f64] {
        TinyVec::as_slice(self)
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        TinyVec::as_mut_slice(self)
    }

    fn from_seq<'de, S>(first: f64, mut seq: S) -> Result<Self, S::Error>
    where
        Self: Sized,
        S: SeqAccess<'de>,
    {
        let mut floats = TinyVec::<[f64; INLINE_SIZE]>::new();
        floats.push(first);
        while let Some(next) = seq.next_element::<f64>()? {
            floats.push(next);
        }
        Ok(floats)
    }
}

impl<const INLINE_SIZE: usize> private::Sealed for TinyVec<[f64; INLINE_SIZE]> {}

impl<const N: usize> PositionBuffer for [f64; N] {
    fn as_slice(&self) -> &[f64] {
        self
    }

    fn as_slice_mut(&mut self) -> &mut [f64] {
        self
    }

    fn from_seq<'de, S>(first: f64, mut seq: S) -> Result<Self, S::Error>
    where
        Self: Sized,
        S: SeqAccess<'de>,
    {
        use serde::de::Error;

        let mut out = [0.; N];
        out[0] = first;
        let mut counter = 1;
        while let Some(next) = seq.next_element::<f64>()? {
            if counter >= out.len() {
                let mut additional_count = 0;
                while seq.next_element::<f64>()?.is_some() {
                    additional_count += 1;
                }
                return Err(S::Error::custom(format!(
                    "Received more than {N} elements, got {additional_count} additional elements"
                )));
            }
            out[counter] = next;
            counter += 1;
        }
        if counter < out.len() {
            return Err(S::Error::custom(format!("Received less than {N} elements, got {counter} elements only")));
        }
        Ok(out)
    }
}

impl<const N: usize> private::Sealed for [f64; N] {}

mod private {
    pub trait Sealed {}
}
