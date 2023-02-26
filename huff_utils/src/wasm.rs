cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
        /// Implements a shim for the `into_par_iter` which wasm32 does not support
        pub trait IntoParallelIterator {
            type Item;
            type IntoIter: Iterator<Item = Self::Item>;

            /// Returns a non-parallel iterator
            fn into_par_iter(self) -> Self::IntoIter;
        }

        impl<T> IntoParallelIterator for Vec<T> {
            type Item = T;
            type IntoIter = std::vec::IntoIter<T>;

            fn into_par_iter(self) -> Self::IntoIter {
                self.into_iter()
            }
        }

        impl<'data, T: Sync + 'data> IntoParallelIterator for &'data Vec<T> {
            type Item = &'data T;
            type IntoIter = std::slice::Iter<'data, T>;

            fn into_par_iter(self) -> Self::IntoIter {
                <&[T]>::into_iter(self)
            }
        }
    }
}
