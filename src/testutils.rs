use std::cmp::Ord;
use std::fmt::Debug;
use std::time::{Duration, Instant};

use crate::utils::InParams;

/// Function to format duration into a human-readable string
fn format_duration(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!(
            "Sorting time: {:.6} seconds ---> {:.6} seconds",
            duration.as_secs_f64(),
            duration.as_secs_f64()
        )
    } else if duration.as_millis() > 0 {
        format!(
            "Sorting time: {:.6} seconds ---> {} milliseconds",
            duration.as_secs_f64(),
            duration.as_millis()
        )
    } else if duration.as_micros() > 0 {
        format!(
            "Sorting time: {:.6} seconds ---> {} microseconds",
            duration.as_secs_f64(),
            duration.as_micros()
        )
    } else {
        format!(
            "Sorting time: {:.6} seconds ---> {} nanoseconds",
            duration.as_secs_f64(),
            duration.as_nanos()
        )
    }
}

/// Generic function to generate and sort vectors
pub fn test_generic_sort<T, F>(sort_fn: &F, vec_len: usize)
where
    T: Copy + Ord + From<u8> + Debug,
    F: Fn(&mut [T]),
{
    let mut arr: Vec<T> = (0..vec_len).map(|_| rand::random::<u8>().into()).collect();
    let mut arr_sorted = arr.clone();
    println!("Input -> {:?}", arr);

    let start = Instant::now();
    sort_fn(&mut arr);
    let duration = start.elapsed();

    arr_sorted.sort();
    println!("Expect-> {:?}", arr_sorted);
    println!("Result-> {:?}", arr);
    println!("{}", format_duration(duration));

    if arr == arr_sorted {
        println!("√√√ Sorting is correct.");
    } else {
        println!("XXX Sorting is incorrect.");
    }
}

/// Enhanced generic test function to handle additional parameters
pub fn test_generic_sort_with_params<T, U, F>(sort_fn: &F, params: InParams<U>, vec_len: usize)
where
    T: Copy + Ord + From<u8> + Debug,
    U: Debug + Ord,
    F: Fn(&mut [T], InParams<U>),
{
    let mut arr: Vec<T> = (0..vec_len).map(|_| rand::random::<u8>().into()).collect();
    let mut arr_sorted = arr.clone();
    println!("Input -> {:?}", arr);

    let start = Instant::now();
    sort_fn(&mut arr, params);
    let duration = start.elapsed();

    arr_sorted.sort();
    println!("Expect-> {:?}", arr_sorted);
    println!("Result-> {:?}", arr);
    println!("{}", format_duration(duration));

    if arr == arr_sorted {
        println!("√√√ Sorting is correct.");
    } else {
        println!("XXX Sorting is incorrect.");
    }
}
