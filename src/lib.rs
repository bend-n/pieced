#![no_std]
/// Splits the slice into a slice of `N`-element arrays,
/// starting at the beginning of the slice,
/// and a rest slice with length strictly less than `N`.
///
/// # Panics
///
/// Panics if `N` is 0.
/// # Examples
///
/// ```
/// let slice = ['l', 'o', 'r', 'e', 'm'];
/// let (chunks, rest) = pieced::as_with_rest(&slice);
/// assert_eq!(chunks, &[['l', 'o'], ['r', 'e']]);
/// assert_eq!(rest, &['m']);
/// ```
///
/// If you expect the slice to be an exact multiple, you can combine
/// `let`-`else` with an empty slice pattern (or use [`as_exact`]):
/// ```
/// let slice = ['R', 'u', 's', 't'];
/// let (chunks, []) = pieced::as_with_rest(&slice) else {
///     panic!("slice didn't have even length")
/// };
/// assert_eq!(chunks, &[['R', 'u'], ['s', 't']]);
/// ```
pub const fn as_with_rest<T, const N: usize>(slice: &[T]) -> (&[[T; N]], &[T]) {
    assert!(N != 0, "chunk size must be non-zero");
    let len = slice.len() / N;
    let (multiple_of_n, remainder) = slice.split_at(len * N);
    let new_len = multiple_of_n.len() / N;
    // SAFETY: We cast a slice of `new_len * N` elements into
    // a slice of `new_len` many `N` elements chunks.
    (
        unsafe { core::slice::from_raw_parts(multiple_of_n.as_ptr().cast(), new_len) },
        remainder,
    )
}

/// Splits the slice into a slice of `N`-element arrays, assuming that there's no remainder.
///
/// # Panics
///
/// Panics when
/// - The slice splits exactly into `N`-element chunks (aka `self.len() % N == 0`).
/// - `N != 0`.
///
/// # Examples
///
/// ```
/// let slice: &[char] = &['l', 'o', 'r', 'e', 'm', '!'];
/// let chunks: &[[char; 1]] = pieced::as_exact(slice);
/// assert_eq!(chunks, &[['l'], ['o'], ['r'], ['e'], ['m'], ['!']]);
/// let chunks: &[[char; 3]] = pieced::as_exact(slice);
/// assert_eq!(chunks, &[['l', 'o', 'r'], ['e', 'm', '!']]);
/// ```
pub const fn as_exact<T, const N: usize>(slice: &[T]) -> &[[T; N]] {
    assert!(
        N != 0 && slice.len() % N == 0,
        "pieced::as_exact requires `N != 0` and the slice to split exactly into `N`-element chunks",
    );
    let new_len = slice.len() / N;
    // SAFETY: We cast a slice of `new_len * N` elements into
    // a slice of `new_len` many `N` elements chunks.
    unsafe { core::slice::from_raw_parts(slice.as_ptr().cast(), new_len) }
}
