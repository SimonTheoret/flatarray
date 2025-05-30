use crate::builder::FlatBuilder;
use crate::iterator::{Iter, StrIter};
use crate::vector::FlatVec;
use std::ops::Deref;

pub type FlatStr = FlatVec<u8>;

impl FlatStr {
    pub fn from_strings<S: Deref<Target = str>, I: IntoIterator<Item = S>>(
        strings_iter: I,
    ) -> Self {
        let mut builder = FlatBuilder::default();
        strings_iter.into_iter().for_each(|s| {
            let bytes_slice = s.bytes();
            builder.push(bytes_slice);
        });
        builder.build_flatvec()
    }
}

impl FlatStr {
    pub fn iter_strings<'a>(&'a self) -> StrIter<'a> {
        StrIter(Iter::new(self))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_flattened_str() -> (FlatStr, Vec<String>) {
        let input = vec![
            String::from("this is the first sentence"),
            String::from("this is the second sentence"),
            String::from("this is the third sentence"),
        ];
        let flat_str = FlatStr::from_strings(input.clone());
        (flat_str, input)
    }

    #[test]
    fn test_flatvec_len() {
        let (flat_str, expected) = setup_flattened_str();
        let vectored: Vec<_> = Iter::new(&flat_str).collect();
        assert!(expected.len() == vectored.len())
    }
}
