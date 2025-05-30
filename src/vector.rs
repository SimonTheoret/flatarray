use crate::iterator::{FlattenedCollection, Iter, IterMut};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub struct FlatVec<T> {
    pub(crate) content: Vec<T>,
    pub(crate) indices: Vec<usize>,
}

impl<T> FlattenedCollection<T> for FlatVec<T> {
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

impl<'a, T> FlatVec<T> {
    /// Returns an iterator over the content. This iterator returns the
    /// individual elements.
    pub fn iter(&'a self) -> std::slice::Iter<'a, T> {
        self.content.iter()
    }
    /// Returns an iterator over the arrays/vectors used to build the
    /// `FlatVec`. The iterator will return a slice of type `&[T]`.
    pub fn iter_arrays(&'a self) -> Iter<'a, FlatVec<T>, T> {
        Iter::new(self)
    }
    /// Returns an iterator over the arrays/vectors used to build the
    /// `FlatVec`. The iterator will return a slice of type `&mut [T]`.
    pub fn iter_arrays_mut(&'a mut self) -> IterMut<'a, FlatVec<T>, T> {
        IterMut::new(self)
    }
}

impl<T> FlatVec<T> {
    pub fn new(vecs: Vec<Vec<T>>) -> Self {
        Self::from(vecs)
    }
    pub fn from_raw(content: impl Into<Vec<T>>, indices: impl Into<Vec<usize>>) -> Self {
        Self {
            content: content.into(),
            indices: indices.into(),
        }
    }
}

impl<E, I> FromIterator<I> for FlatVec<E>
where
    I: Iterator<Item = E> + ExactSizeIterator,
{
    /// Build the `FlatArray` from an iterator over vectors. Prefer to
    /// use the `From(Vec<Vec<T>>)` implementation over `from_iter`:
    /// it pre-allocate a minimal amount of memory before filling
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
        let tokens = flattened;
        let indices_boxed = indices;
        Self {
            content: tokens,
            indices: indices_boxed,
        }
    }
}

impl<T> From<Vec<Vec<T>>> for FlatVec<T> {
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
        let tokens = flattened;
        let indices_boxed = indices;
        Self {
            content: tokens,
            indices: indices_boxed,
        }
    }
}

impl<'a> From<Vec<Vec<&'a str>>> for FlatVec<Cow<'a, str>> {
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
        let tokens = flattened;
        let indices_boxed = indices;
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
        let token_vecs = FlatVec::new(vecs);
        dbg!(token_vecs.clone());
        let expected = 2;
        let actual = token_vecs.iter_arrays().count();
        assert_eq!(expected, actual);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_iter_TokenVec() {
        let vecs = build_vecs();
        let token_vecs = FlatVec::new(vecs);
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

    impl<T> From<FlatVec<T>> for Vec<Vec<T>>
    where
        T: Clone,
    {
        fn from(value: FlatVec<T>) -> Self {
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
        let actual = FlatVec::new(vecs);
        let expected_tokens = vec![
            "O", "O", "O", "B-MISC", "I-MISC", "I-MISC", "O", "B-PER", "I-PER", "O",
        ];

        let expected_indices = vec![0, 7, 10];
        let expected = FlatVec {
            content: expected_tokens,
            indices: expected_indices,
        };
        assert_eq!(expected, actual);
    }
}
