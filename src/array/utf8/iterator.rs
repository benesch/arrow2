use crate::array::{ArrayAccessor, ArrayValuesIter, Offset};
use crate::bitmap::utils::ZipValidity;

use super::{MutableUtf8Array, MutableUtf8ValuesArray, Utf8Array};

unsafe impl<'a, O: Offset> ArrayAccessor<'a> for Utf8Array<O> {
    type Item = &'a str;

    #[inline]
    unsafe fn value_unchecked(&'a self, index: usize) -> Self::Item {
        self.value_unchecked(index)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

/// Iterator of values of an [`Utf8Array`].
pub type Utf8ValuesIter<'a, O> = ArrayValuesIter<'a, Utf8Array<O>>;

impl<'a, O: Offset> IntoIterator for &'a Utf8Array<O> {
    type Item = Option<&'a str>;
    type IntoIter = ZipValidity<'a, &'a str, Utf8ValuesIter<'a, O>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

unsafe impl<'a, O: Offset> ArrayAccessor<'a> for MutableUtf8Array<O> {
    type Item = &'a str;

    #[inline]
    unsafe fn value_unchecked(&'a self, index: usize) -> Self::Item {
        self.value_unchecked(index)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

/// Iterator of values of an [`MutableUtf8ValuesArray`].
pub type MutableUtf8ValuesIter<'a, O> = ArrayValuesIter<'a, MutableUtf8ValuesArray<O>>;

impl<'a, O: Offset> IntoIterator for &'a MutableUtf8Array<O> {
    type Item = Option<&'a str>;
    type IntoIter = ZipValidity<'a, &'a str, MutableUtf8ValuesIter<'a, O>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

unsafe impl<'a, O: Offset> ArrayAccessor<'a> for MutableUtf8ValuesArray<O> {
    type Item = &'a str;

    #[inline]
    unsafe fn value_unchecked(&'a self, index: usize) -> Self::Item {
        self.value_unchecked(index)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<'a, O: Offset> IntoIterator for &'a MutableUtf8ValuesArray<O> {
    type Item = &'a str;
    type IntoIter = ArrayValuesIter<'a, MutableUtf8ValuesArray<O>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
