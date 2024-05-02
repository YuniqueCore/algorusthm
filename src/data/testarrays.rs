use rand::seq::SliceRandom;
use rand::thread_rng;
// For the shuffle method
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use std::fmt::Debug;

pub const TEST_I32_VEC_1: &[i32] = &[1, 3, 4, 8, 2, 7, 4, 0];
pub const TEST_I32_VEC_1_SORTED: &[i32] = &[0, 1, 2, 3, 4, 4, 7, 8];

/// Generates a vector containing random integers between 0 and 100 (inclusive) of the specified length.
///
/// # Parameters
/// - `length`: The length of the vector.
///
/// # Returns
/// Returns a `Vec<i32>` containing random integers between 0 and 100 (inclusive) of the specified length.
pub fn generate_0_100_vec(length: usize) -> Vec<i32> {
    generate_random_vec(length, 0, 100)
}

/// Generates a vector containing random integers of the specified length.
///
/// # Parameters
/// - `length`: The length of the vector.
/// - `min_number`: The minimum value (inclusive) for generated random numbers.
/// - `max_number`: The maximum value (inclusive) for generated random numbers.
///
/// # Returns
/// Returns a `Vec<i32>` containing random integers of the specified length.
pub fn generate_random_vec(length: usize, min_number: i32, max_number: i32) -> Vec<i32> {
    let mut rng = thread_rng();
    // Initialize a vector of length `length`, with each element being a random number between `min_number` and `max_number`.
    let mut vec = (0..length)
        .map(|_| rng.gen_range(min_number..=max_number))
        .collect::<Vec<i32>>();
    // Shuffle the elements within the vector to ensure their order is randomized.
    vec.shuffle(&mut rng);
    vec
}

/// Generic function to generate a random vector supporting different numeric types.
pub fn generate_random_vec_within<T>(length: usize, min_number: T, max_number: T) -> Vec<T>
where
    T: Copy + PartialOrd + Debug + rand::distributions::uniform::SampleUniform,
{
    let mut rng = rand::thread_rng();
    // Create a uniform distribution within the specified range.
    let range = Uniform::new_inclusive(min_number, max_number);

    // Initialize a vector of `length` with each element being a random number from the range.
    let mut vec = (0..length).map(|_| rng.sample(&range)).collect::<Vec<T>>();

    // Shuffle the elements within the vector to ensure their order is randomized.
    vec.shuffle(&mut rng);

    vec
}
