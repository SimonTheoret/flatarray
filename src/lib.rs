/// Small crate containing a custom datastructure. This datastructure
/// is called a `FlatArray` and is a cache-friendly alternative to a
/// Vec<Vec<T>>. Instead, a `FlatArray` is of fixed size but can be
/// iterated over as if it was a `Vec<Vec<T>>`, with the help of the
/// `iter_arrays` and `iter_arrays_mut` methods.
///
/// This crate is intended to be used by other crates, such as `named_entity` and
/// `rusev`, which explains why its API surface is so small.
mod array;
pub use self::array::FlatArray;
mod vector;
pub use self::vector::FlatVec;
mod builder;
pub use self::builder::FlatBuilder; // re-export the builder
mod iterator;
pub use self::iterator::{FlattenedCollection, Iter, IterMut};
/// This method allocates. It should only be used in the testing environment.

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;
    fn build_vecs() -> Vec<Vec<&'static str>> {
        vec![
            vec!["O", "O", "O", "B-MISC", "I-MISC", "I-MISC", "O"],
            vec!["B-PER", "I-PER", "O"],
        ]
    }
    fn setup_flattened_iter<T>(
        input: Vec<Vec<T>>,
        flattened_typ: &'static str,
    ) -> (
        Box<dyn FlattenedCollection<&'static str>>,
        Vec<Vec<&'static str>>,
    ) {
        let input = vec![
            vec!["this", "is", "the", "first", "sentence"],
            vec!["this", "is", "the", "second", "sentence"],
            vec!["this", "is", "the", "second", "sentence"],
        ];
        let mut builder = FlatBuilder::default();
        builder.push(input);
        if flattened_typ == "vec" {
            return (Box::new(builder.build_flatvec()), input);
        } else if flattened_typ == "array" {
            return (Box::new(builder.build_flatarray()), input);
        } else {
            panic!("Wrong flattened collection type");
        }
    }

    #[test]
    fn collect_usizes() {
        let builder = FlatBuilder::push_exact_sized(&mut self, item);
    }
}
