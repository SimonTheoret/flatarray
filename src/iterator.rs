use std::marker::PhantomData;
use std::ops::Range;

pub trait FlattenedCollection<T> {
    fn indices_len(&self) -> usize;
    unsafe fn get_indices(&self, index: usize) -> usize;
    unsafe fn get_content<'a>(&'a self, range: Range<usize>) -> &'a [T];
    unsafe fn get_mut_content<'a>(&'a mut self, range: Range<usize>) -> &'a mut [T];
}

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
        if self.token_vecs.indices_len() == 0 || self.counter >= self.token_vecs.indices_len() - 1 {
            return None;
        }
        let start = unsafe { self.token_vecs.get_indices(self.indice_index) };
        let end = unsafe { self.token_vecs.get_indices(self.indice_index + 1) };
        self.counter += 1;
        self.indice_index += 1;
        unsafe { Some(self.token_vecs.get_content(start..end)) }
    }
}

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
