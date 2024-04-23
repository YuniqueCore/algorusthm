use std::fmt::Debug;

use crate::testarrays;

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
    // 生成随机向量
    let mut arr = testarrays::generate_0_100_vec(vec_len);
    // 克隆生成的向量以用于排序比较
    let mut arr_sorted = arr.clone();
    // 打印生成的向量
    println!("Input -> {:?}", arr);
    // 使用提供的排序函数对向量进行排序
    sort_fn(&mut arr);
    // 使用 Vec 的排序方法对克隆的向量进行排序
    arr_sorted.sort();
    // 打印预期的排序结果
    println!("Expect-> {:?}", arr_sorted);
    // 打印排序后的结果
    println!("Result-> {:?}", arr);

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
pub fn test_sort_with<T: Debug + Ord>(
    sort_fn: &dyn Fn(&mut [i32], InParams<T>),
    params: InParams<T>,
    vec_len: usize,
) -> bool {
    // 生成随机向量
    let mut arr = testarrays::generate_0_100_vec(vec_len);
    // 克隆生成的向量以用于排序比较
    let mut arr_sorted = arr.clone();
    // 打印生成的向量
    println!("Input -> {:?}", arr);
    // 使用提供的排序函数对向量进行排序
    sort_fn(&mut arr, params);
    // 使用 Vec 的排序方法对克隆的向量进行排序
    arr_sorted.sort();
    // 打印预期的排序结果
    println!("Expect-> {:?}", arr_sorted);
    // 打印排序后的结果
    println!("Result-> {:?}", arr);

    // 检查两个向量是否相等
    if arr == arr_sorted {
        println!("Sorting is correct.");
        true
    } else {
        println!("Sorting is incorrect.");
        false
    }
}
