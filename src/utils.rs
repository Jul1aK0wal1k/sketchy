#[cfg(debug_assertions)]
pub(crate) fn debug_bounds_check<T>(index: usize, arr: &[T]) {
    if index >= arr.len() {
        let panic_msg = format!(
            "Attempted to access index {:?} of array of size {:?}",
            index.to_string(),
            arr.len().to_string()
        );
        panic!("{}", panic_msg);
    }
}

#[cfg(not(debug_assertions))]
#[inline(always)]
pub(crate) fn debug_bounds_check<T>(index: usize, arr: &[T]) {}

#[inline(always)]
pub(crate) fn real_index(i: usize, j: usize, n_columns: usize) -> usize {
    i * (n_columns - 1) + j
}
