use std::fmt::Debug;
use std::time::Duration;
use std::time::Instant;

use crate::testarrays;

// Function to format duration into a human-readable string
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

// print the function name
pub fn println_fn_name<T>(func: T) {
    println!("Position at: ({})", std::any::type_name::<T>());
}

pub trait ToUsize: Sized {
    fn to_usize(&self) -> usize;
}

impl ToUsize for i32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

pub trait FromUsize: Sized {
    fn from_usize(usize_val: usize) -> Self;
}

impl FromUsize for i32 {
    fn from_usize(usize_val: usize) -> Self {
        usize_val as i32
    }
}

pub enum InParams<T: Debug + Ord> {
    Params(Vec<T>),
}

pub fn test_sort(sort_fn: &dyn Fn(&mut [i32]), vec_len: usize) -> bool {
    // Generate a random vector
    let mut arr = testarrays::generate_0_100_vec(vec_len);
    // Clone the generated vector for comparison after sorting
    let mut arr_sorted = arr.clone();
    // Print the generated vector
    println!("Input -> {:?}", arr);

    // Start timing
    let start = Instant::now();
    // Use the provided sort function to sort the vector
    sort_fn(&mut arr);
    // Stop timing
    let duration = start.elapsed();

    // Use Vec's sort method to sort the cloned vector
    arr_sorted.sort();
    // Print the expected sorted result
    println!("Expect-> {:?}", arr_sorted);
    // Print the result after sorting
    println!("Result-> {:?}", arr);

    // Print sorting duration
    println!("{}", format_duration(duration));

    // Check if the two vectors are equal
    if arr == arr_sorted {
        println!("√√√ Sorting is correct.");
        true
    } else {
        println!("XXX Sorting is incorrect.");
        false
    }
}

pub fn test_sort_u32(sort_fn: &dyn Fn(&mut [u32]), vec_len: usize) -> bool {
    // 生成随机向量
    let mut arr: Vec<u32> = testarrays::generate_0_100_vec(vec_len)
        .iter()
        .map(|&x| x as u32)
        .collect();
    // 克隆生成的向量以用于排序比较
    let mut arr_sorted = arr.clone();
    // 打印生成的向量
    println!("Input -> {:?}", arr);

    // Start timing
    let start = Instant::now();
    // Use the provided sort function to sort the vector
    sort_fn(&mut arr);
    // Stop timing
    let duration = start.elapsed();

    // Use Vec's sort method to sort the cloned vector
    arr_sorted.sort();
    // Print the expected sorted result
    println!("Expect-> {:?}", arr_sorted);
    // Print the result after sorting
    println!("Result-> {:?}", arr);

    // Print sorting duration
    println!("{}", format_duration(duration));

    // 检查两个向量是否相等
    if arr == arr_sorted {
        println!("√√√ Sorting is correct.");
        true
    } else {
        println!("XXX Sorting is incorrect.");
        false
    }
}

/// Test the sort function with any number of Input parameters
pub fn test_sort_with_i32<T: Debug + Ord>(
    sort_fn: &dyn Fn(&mut [i32], InParams<T>),
    params: InParams<T>,
    vec_len: usize,
) -> bool {
    // 生成随机向量
    let mut arr = testarrays::generate_0_100_vec(vec_len);
    // 克隆生成的向量以用于排序比较
    let mut arr_sorted = arr.clone();

    println!("Input -> {:?}", arr);

    // Start timing
    let start = Instant::now();
    // Use the provided sort function to sort the vector
    sort_fn(&mut arr, params);
    // Stop timing
    let duration = start.elapsed();

    // Use Vec's sort method to sort the cloned vector
    arr_sorted.sort();
    // Print the expected sorted result
    println!("Expect-> {:?}", arr_sorted);
    // Print the result after sorting
    println!("Result-> {:?}", arr);

    // Print sorting duration
    println!("{}", format_duration(duration));

    // 检查两个向量是否相等
    if arr == arr_sorted {
        println!("√√√ Sorting is correct.");
        true
    } else {
        println!("XXX Sorting is incorrect.");
        false
    }
}

pub fn test_sort_with_u32<T: Debug + Ord>(
    sort_fn: &dyn Fn(&mut [u32], InParams<T>),
    params: InParams<T>,
    vec_len: usize,
) -> bool {
    // 生成随机向量
    let mut arr: Vec<u32> = testarrays::generate_0_100_vec(vec_len)
        .iter()
        .map(|&x| x as u32)
        .collect();
    // 克隆生成的向量以用于排序比较
    let mut arr_sorted = arr.clone();
    println!("Input -> {:?}", arr);

    // Start timing
    let start = Instant::now();
    // Use the provided sort function to sort the vector
    sort_fn(&mut arr, params);
    // Stop timing
    let duration = start.elapsed();

    // Use Vec's sort method to sort the cloned vector
    arr_sorted.sort();
    // Print the expected sorted result
    println!("Expect-> {:?}", arr_sorted);
    // Print the result after sorting
    println!("Result-> {:?}", arr);

    // Print sorting duration
    println!("{}", format_duration(duration));

    // 检查两个向量是否相等
    if arr == arr_sorted {
        println!("√√√ Sorting is correct.");
        true
    } else {
        println!("XXX Sorting is incorrect.");
        false
    }
}

pub mod global_counter {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DEPTH: AtomicUsize = AtomicUsize::new(0);

    fn get_depth() -> usize {
        DEPTH.fetch_add(1, Ordering::Relaxed)
    }

    pub fn init_depth() {
        DEPTH.store(0, Ordering::Relaxed);
    }
}
