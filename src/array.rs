use crate::iterator::{FlattenedCollection, Iter, IterMut};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;

/// Custom datastructure built for reducing cache misses. This is a unmutable
/// datastructure
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct FlatArray<T> {
    pub(crate) content: Box<[T]>,
    pub(crate) indices: Box<[usize]>,
}

impl<T> FlattenedCollection<T> for FlatArray<T> {
    fn indices_len(&self) -> usize {
        self.indices.len()
    }
    unsafe fn get_indices(&self, index: usize) -> usize {
        unsafe { *self.indices.get_unchecked(index) }
    }
    unsafe fn get_content(&self, range: std::ops::Range<usize>) -> &[T] {
        unsafe { self.content.get_unchecked(range) }
    }
    unsafe fn get_mut_content(&mut self, range: std::ops::Range<usize>) -> &mut [T] {
        unsafe { self.content.get_unchecked_mut(range) }
    }
}

impl<T> Default for FlatArray<T> {
    fn default() -> Self {
        let content: Box<[T]> = Box::new([]);
        let indices: Box<[usize]> = Box::new([]);
        Self { content, indices }
    }
}

impl<T> FlatArray<T> {
    pub fn new(vecs: Vec<Vec<T>>) -> Self {
        Self::from(vecs)
    }
    pub fn from_raw(content: impl Into<Box<[T]>>, indices: impl Into<Box<[usize]>>) -> Self {
        Self {
            content: content.into(),
            indices: indices.into(),
        }
    }
}

impl<T> FlatArray<T> {
    /// Borrows the content of the FlatArray as a slice.
    pub fn get_content(&self) -> &[T] {
        self.content.as_ref()
    }
}

impl<'a, T> FlatArray<T> {
    /// Returns an iterator over the content. This iterator returns the
    /// individual elements.
    pub fn iter(&'a self) -> std::slice::Iter<'a, T> {
        self.content.iter()
    }
    /// Returns an iterator over the arrays/vectors used to build the
    /// `FlatArray`. The iterator will return a slice of type `&[T]`.
    pub fn iter_arrays(&'a self) -> Iter<'a, FlatArray<T>, T> {
        Iter::new(self)
    }
    /// Returns an iterator over the arrays/vectors used to build the
    /// `FlatArray`. The iterator will return a slice of type `&mut [T]`.
    pub fn iter_arrays_mut(&'a mut self) -> IterMut<'a, FlatArray<T>, T> {
        IterMut::new(self)
    }
}

impl<T> Deref for FlatArray<T> {
    type Target = [T];
    /// `FlatArray` implements the `Deref` trait. It allows users to
    /// treat a `FlatArray` as if it was a slice (e.g. `&[T]`).
    fn deref(&self) -> &Self::Target {
        self.content.as_ref()
    }
}

impl<E, I> FromIterator<I> for FlatArray<E>
where
    I: Iterator<Item = E> + ExactSizeIterator,
{
    /// Build the `FlatArray` from an iterator over vectors. Prefer to
    /// use the `From(Vec<Vec<T>>)` implementation over `from_iter`:
    /// they pre-allocate a minimal amount of memory before filling
    /// the `FlatArray`.
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut flattened = Vec::new();
        let mut indices = Vec::new();
        indices.push(0);
        for vec in iter.into_iter() {
            indices.push(indices.last().unwrap() + vec.len());
            for s in vec {
                flattened.push(s);
            }
        }
        let tokens = flattened.into_boxed_slice();
        let indices_boxed = indices.into_boxed_slice();
        Self {
            content: tokens,
            indices: indices_boxed,
        }
    }
}

impl<T> From<Vec<Vec<T>>> for FlatArray<T> {
    #[inline(always)]
    fn from(value: Vec<Vec<T>>) -> Self {
        let length: usize = value.iter().map(|v| v.len()).sum();
        let indices_length = value.len();
        let mut flattened = Vec::with_capacity(length);
        let mut indices = Vec::with_capacity(indices_length);
        indices.push(0);
        for vec in value.into_iter() {
            indices.push(indices.last().unwrap() + vec.len());
            for s in vec {
                flattened.push(s);
            }
        }
        let tokens = flattened.into_boxed_slice();
        let indices_boxed = indices.into_boxed_slice();
        Self {
            content: tokens,
            indices: indices_boxed,
        }
    }
}

impl<'a> From<Vec<Vec<&'a str>>> for FlatArray<Cow<'a, str>> {
    #[inline(always)]
    fn from(value: Vec<Vec<&'a str>>) -> Self {
        let length: usize = value.iter().map(|v| v.len()).sum();
        let indices_length = value.len();
        let mut flattened = Vec::with_capacity(length);
        let mut indices = Vec::with_capacity(indices_length);
        let mut current_indice = 0;
        indices.push(0);
        for vec in value.into_iter() {
            for s in &vec {
                current_indice += 1;
                flattened.push(Cow::from(*s));
            }
            if !vec.is_empty() {
                indices.push(current_indice);
            }
        }
        let tokens = flattened.into_boxed_slice();
        let indices_boxed = indices.into_boxed_slice();
        Self {
            content: tokens,
            indices: indices_boxed,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::borrow::Cow;

    #[test]
    #[allow(non_snake_case)]
    fn test_iter_TokenVec_len() {
        let vecs = build_vecs();
        let token_vecs = FlatArray::new(vecs);
        dbg!(token_vecs.clone());
        let expected = 2;
        let actual = token_vecs.iter_arrays().count();
        assert_eq!(expected, actual);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_iter_TokenVec() {
        let vecs = build_vecs();
        let token_vecs = FlatArray::new(vecs);
        dbg!(token_vecs.clone());
        for (i, actual) in token_vecs.iter_arrays().enumerate() {
            if i == 0 {
                dbg!(i);
                let expected = &[
                    Cow::from("O"),
                    Cow::from("O"),
                    Cow::from("O"),
                    Cow::from("B-MISC"),
                    Cow::from("I-MISC"),
                    Cow::from("I-MISC"),
                    Cow::from("O"),
                ];
                assert_eq!(expected, actual);
            } else if i == 1 {
                dbg!(i);
                let expected = &[Cow::from("B-PER"), Cow::from("I-PER"), Cow::from("O")];
                assert_eq!(expected, actual);
            } else if i > 1 {
                dbg!(i);
                dbg!(actual);
                dbg!(token_vecs.clone());
                panic!("Only two iterations possible")
            }
        }
    }

    impl<T> From<FlatArray<T>> for Vec<Vec<T>>
    where
        T: Clone,
    {
        fn from(value: FlatArray<T>) -> Self {
            value.iter_arrays().map(Vec::from).collect()
        }
    }
    fn build_vecs() -> Vec<Vec<&'static str>> {
        vec![
            vec!["O", "O", "O", "B-MISC", "I-MISC", "I-MISC", "O"],
            vec!["B-PER", "I-PER", "O"],
        ]
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_new_TokenVec() {
        let vecs = build_vecs();
        let actual = FlatArray::new(vecs);
        let expected_tokens = Box::new([
            "O", "O", "O", "B-MISC", "I-MISC", "I-MISC", "O", "B-PER", "I-PER", "O",
        ]);

        let expected_indices = Box::new([0, 7, 10]);
        let expected = FlatArray {
            content: expected_tokens,
            indices: expected_indices,
        };
        assert_eq!(expected, actual);
    }
}
