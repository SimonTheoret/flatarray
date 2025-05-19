use serde::{Deserialize, Serialize};

use crate::FlatArray;
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
