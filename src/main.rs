pub mod algos;
pub mod data;
pub mod utils;
use std::vec;

use algos::exchanges;
use algos::sorts;
use data::testarrays;

fn main() {
    // import_test();
    // bubble_sort_test();
    // swap_num();
    // quick_sort_test();
    // selection_sort_test();
    insertion_sort_test();
}

fn import_test() {
    utils::println_fn_name(import_test);
    sorts::hello_sorts();
}

fn selection_sort_test() {
    utils::println_fn_name(selection_sort_test);
    utils::test_sort(&sorts::selection_sort::sort, 8);
}

fn quick_sort_test() {
    utils::println_fn_name(quick_sort_test);
    let in_params = utils::InParams::Params(vec![0, 8]);
    utils::test_sort_with(&sorts::quick_sort::sort_with_params, in_params, 9);
    // let mut arr = Vec::from(testarrays::TEST_I32_VEC_1);
    // println!("Input -> {:?}", arr);
    // let l = arr.len() - 1;
    // sorts::quick_sort::sort(&mut arr, 0, l as i32);
    // println!("Result-> {:?}", arr);

    // let mut arr = testarrays::generate_0_100_vec(8);
    // println!("Input -> {:?}", arr);
    // let l = arr.len() - 1;
    // sorts::quick_sort::sort(&mut arr, 0, l as i32);
    // println!("Result-> {:?}", arr);
}

fn bubble_sort_test() {
    utils::println_fn_name(bubble_sort_test);
    let mut arr = Vec::from(testarrays::TEST_I32_VEC_1);
    println!("Input -> {:?}", arr);
    sorts::bubble_sort::sort(&mut arr);
    println!("Result-> {:?}", arr);
}

fn insertion_sort_test() {
    utils::println_fn_name(insertion_sort_test);
    let mut arr = Vec::from(testarrays::TEST_I32_VEC_1);
    println!("Input -> {:?}", arr);
    sorts::insertion_sort::sort(&mut arr);
    let mut arr = Vec::from(testarrays::TEST_I32_VEC_1);
    sorts::insertion_sort::sort_example(&mut arr);
    println!("Result-> {:?}", arr);

    let mut arr = Vec::from(testarrays::generate_0_100_vec(10));
    let mut arr_example = arr.clone();
    println!("Input -> {:?}", arr);
    sorts::insertion_sort::sort(&mut arr);
    sorts::insertion_sort::sort_example(&mut arr_example);
    println!("Result-> {:?}", arr);
}

#[allow(unused_variables)]
fn swap_num() {
    utils::println_fn_name(swap_num);
    let mut a = 109;
    let mut b = -2;

    println!("Before exchange:");
    println!("a: {}, address: {:p}", a, &a);
    println!("b: {}, address: {:p}", b, &b);

    let a_address = &a as *const _;
    let b_address = &b as *const _;

    exchanges::swap_num(&mut a, &mut b);

    println!("After exchange:");
    println!("a: {}, address: {:p}", a, &a);
    println!("b: {}, address: {:p}", b, &b);

    // println!("After exchange:");
    // println!("a: {}, address: {:p}", *new_a, new_a);
    // println!("b: {}, address: {:p}", *new_b, new_b);

    // // Compare the address
    // assert_eq!(a_address, new_a as *const _);
    // assert_eq!(b_address, new_b as *const _);

    // The right way to compare the address
    // println!(
    //     "address a is new_a? {}",
    //     &a as *const i32 == &new_a as *const i32
    // );
    // println!(
    //     "address b is new_b? {}",
    //     &b as *const i32 == &new_b as *const i32
    // );
}
