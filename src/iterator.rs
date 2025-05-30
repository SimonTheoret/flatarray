use std::marker::PhantomData;
use std::ops::Range;
use std::ops::{Deref, DerefMut};

use crate::str::FlatStr;

pub trait FlattenedCollection<T> {
    fn indices_len(&self) -> usize;
    /// # Safety
    /// The index must be inside the bounds of the indices.
    unsafe fn get_indices(&self, index: usize) -> usize;
    /// # Safety
    /// The range must be inside the bounds of the indices.
    unsafe fn get_content(&self, range: Range<usize>) -> &[T];
    /// # Safety
    /// The range must be inside the bounds of the indices.
    unsafe fn get_mut_content(&mut self, range: Range<usize>) -> &mut [T];
    fn indices_empty(&self) -> bool {
        self.indices_len() == 0
    }
}

impl<T> FlattenedCollection<T> for Box<dyn FlattenedCollection<T>> {
    fn indices_len(&self) -> usize {
        self.deref().indices_len()
    }
    unsafe fn get_indices(&self, index: usize) -> usize {
        unsafe { self.deref().get_indices(index) }
    }
    unsafe fn get_content(&self, range: Range<usize>) -> &[T] {
        unsafe { self.deref().get_content(range) }
    }
    unsafe fn get_mut_content(&mut self, range: Range<usize>) -> &mut [T] {
        unsafe { self.deref_mut().get_mut_content(range) }
    }
}

impl<T, Flat: FlattenedCollection<T>> FlattenedCollection<T> for Box<Flat> {
    fn indices_len(&self) -> usize {
        self.deref().indices_len()
    }
    unsafe fn get_indices(&self, index: usize) -> usize {
        unsafe { self.deref().get_indices(index) }
    }
    unsafe fn get_content(&self, range: Range<usize>) -> &[T] {
        unsafe { self.deref().get_content(range) }
    }
    unsafe fn get_mut_content(&mut self, range: Range<usize>) -> &mut [T] {
        unsafe { self.deref_mut().get_mut_content(range) }
    }
}

#[derive(Debug)]
pub struct Iter<'a, Flat, T>
where
    Flat: FlattenedCollection<T>,
    T: 'a,
{
    indice_index: usize,
    token_vecs: &'a Flat,
    counter: usize,
    phantom_data: PhantomData<T>,
}

impl<'a, Flat, T> Clone for Iter<'a, Flat, T>
where
    Flat: FlattenedCollection<T>,
    T: 'a,
{
    fn clone(&self) -> Self {
        Self {
            indice_index: self.indice_index,
            token_vecs: self.token_vecs,
            counter: self.counter,
            phantom_data: PhantomData,
        }
    }
}

impl<'a, Flat, T> Iter<'a, Flat, T>
where
    Flat: FlattenedCollection<T>,
{
    pub(crate) fn new(token_vecs: &'a Flat) -> Self {
        Self {
            indice_index: 0,
            token_vecs,
            counter: 0,
            phantom_data: PhantomData,
        }
    }
}
impl<'a, Flat, T> Iterator for Iter<'a, Flat, T>
where
    Flat: FlattenedCollection<T>,
{
    type Item = &'a [T];
    // NOTE: Inlining this function seems to _reduce_ the performance
    // #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.token_vecs.indices_empty() || self.counter >= self.token_vecs.indices_len() - 1 {
            return None;
        }
        let start = unsafe { self.token_vecs.get_indices(self.indice_index) };
        let end = unsafe { self.token_vecs.get_indices(self.indice_index + 1) };
        self.counter += 1;
        self.indice_index += 1;
        unsafe { Some(self.token_vecs.get_content(start..end)) }
    }
}

#[derive(Debug)]
pub struct IterMut<'a, Flat, T>
where
    Flat: FlattenedCollection<T>,
{
    indice_index: usize,
    token_vecs: &'a mut Flat,
    counter: usize,
    phantom_data: PhantomData<&'a T>,
}

impl<'a, Flat, T> IterMut<'a, Flat, T>
where
    Flat: FlattenedCollection<T>,
{
    pub(crate) fn new(token_vecs: &'a mut Flat) -> Self {
        Self {
            indice_index: 0,
            token_vecs,
            counter: 0,
            phantom_data: PhantomData,
        }
    }
}

impl<'a, Flat, T> Iterator for IterMut<'a, Flat, T>
where
    Flat: FlattenedCollection<T>,
{
    type Item = &'a mut [T];
    // NOTE: Inlining this function seems to reduce the performance
    // #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter >= self.token_vecs.indices_len() - 1 {
            return None;
        }
        let start = unsafe { self.token_vecs.get_indices(self.indice_index) };
        let end = unsafe { self.token_vecs.get_indices(self.indice_index + 1) };
        self.counter += 1;
        self.indice_index += 1;
        unsafe { Some(&mut *(self.token_vecs.get_mut_content(start..end) as *mut _)) }
    }
}

pub struct StrIter<'a>(pub(crate) Iter<'a, FlatStr, u8>);

impl<'a> Iterator for StrIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(bytes) => unsafe { Some(str::from_utf8_unchecked(bytes)) },
        }
    }
}
