use std::mem::take;

use serde::{Deserialize, Serialize};

use crate::FlatArray;
use crate::FlatStr;
use crate::FlatVec;

#[derive(Debug, Deserialize, Serialize)]
/// This struct can be used to dynamically build a `FlatArray` or a FlatVec by pushing
/// element into it.
pub struct FlatBuilder<T> {
    content: Vec<T>,
    indices: Vec<usize>,
}

impl<T> FlatBuilder<T> {
    pub fn push_exact_sized<I: IntoIterator<Item = T> + ExactSizeIterator>(&mut self, item: I) {
        unsafe {
            self.indices
                .push(self.indices.last().unwrap_unchecked() + item.len())
        };
        for s in item {
            self.content.push(s);
        }
    }
    pub fn push<I: IntoIterator<Item = T>>(&mut self, item: I) {
        let mut current_indice = unsafe { *self.indices.last().unwrap_unchecked() };
        for s in item {
            self.content.push(s);
            current_indice += 1;
        }
        self.indices.push(current_indice)
    }

    pub fn push_owned<I, B>(&mut self, item: I)
    where
        I: IntoIterator<Item = B>,
        B: ToOwned<Owned = T>,
    {
        let mut current_indice = unsafe { *self.indices.last().unwrap_unchecked() };
        for s in item {
            self.content.push(s.to_owned());
            current_indice += 1;
        }
        self.indices.push(current_indice)
    }
    // pub fn push_str<I, S>(&mut self, item: I)
    // where
    //     I: IntoIterator<Item = S>,
    //     S: Deref<Target = str>,
    // {
    //     let mut current_indice = unsafe { *self.indices.last().unwrap_unchecked() };
    //     for s in item {
    //         self.content.push(s.deref().as_bytes());
    //         current_indice += 1;
    //     }
    //     self.indices.push(current_indice)
    // }

    pub fn build_flatvec(self) -> FlatVec<T> {
        FlatVec {
            content: self.content,
            indices: self.indices,
        }
    }
    pub fn build_flatarray(self) -> FlatArray<T> {
        FlatArray {
            content: self.content.into_boxed_slice(),
            indices: self.indices.into_boxed_slice(),
        }
    }
}
impl FlatBuilder<u8> {
    pub fn build_flatstr(self) -> FlatStr {
        FlatStr {
            content: self.content,
            indices: self.indices,
        }
    }
}

impl<T: Default> FlatBuilder<T> {
    pub fn push_take<I>(&mut self, item: I)
    where
        I: IntoIterator<Item = T>,
    {
        let mut current_indice = unsafe { *self.indices.last().unwrap_unchecked() };
        for mut s in item {
            self.content.push(take(&mut s));
            current_indice += 1;
        }
        self.indices.push(current_indice)
    }
}

impl<T> Default for FlatBuilder<T> {
    fn default() -> Self {
        Self {
            content: vec![],
            indices: vec![0],
        }
    }
}

impl<T> Clone for FlatBuilder<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            indices: self.indices.clone(),
        }
    }
}
