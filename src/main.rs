pub mod algos;
pub mod data;
pub mod testutils;
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
    // insertion_sort_test();
    // merge_sort_test();
    // heap_sort_test();
    // bucket_sort_test();
    // radix_sort_test();
    couting_sort_test();
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
    utils::test_sort_with_i32(&sorts::quick_sort::sort_with_params, in_params, 9);
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

fn merge_sort_test() {
    utils::println_fn_name(merge_sort_test);
    let mut arr = Vec::from(testarrays::TEST_I32_VEC_1);
    let right = arr.len() - 1;
    println!("Input -> {:?}", arr);
    sorts::merge_sort::sort(&mut arr, 0, right, 0);
    println!("Result-> {:?}", arr);

    let mut arr = Vec::from(testarrays::generate_0_100_vec(10));
    let right = arr.len() - 1;
    println!("Input -> {:?}", arr);
    sorts::merge_sort::sort(&mut arr, 0, right, 0);
    println!("Result-> {:?}", arr);
}

fn heap_sort_test() {
    utils::println_fn_name(heap_sort_test);
    let mut arr = Vec::from(testarrays::TEST_I32_VEC_1);
    println!("Input -> {:?}", arr);
    sorts::heap_sort::sort_1(&mut arr);
    println!("Result-> {:?}", arr);

    let mut arr = Vec::from(testarrays::generate_0_100_vec(10));
    println!("Input -> {:?}", arr);
    sorts::heap_sort::sort_1(&mut arr);
    println!("Result-> {:?}", arr);
}

fn bucket_sort_test() {
    utils::println_fn_name(bucket_sort_test);
    utils::test_sort(&sorts::buket_sort::sort, 10);
}

fn radix_sort_test() {
    utils::println_fn_name(bucket_sort_test);
    utils::test_sort_with_u32(
        &sorts::radix_sort::sort_with_inparam,
        utils::InParams::Params(vec![10]),
        10,
    );
}

fn couting_sort_test() {
    utils::println_fn_name(couting_sort_test);
    utils::test_sort_u32(&sorts::counting_sort::sort_simple_by_buckets, 10);
    // utils::test_sort_u32(&sorts::counting_sort::sort_simple_by_counter, 10);
    testutils::test_generic_sort(&sorts::counting_sort::sort_simple_by_counter, 10);
    testutils::test_generic_sort(&sorts::counting_sort::sort, 10);
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
