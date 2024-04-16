pub mod algos;
use algos::sorts;

// print the function name
fn println_fn_name<T>(func: T) {
    println!("Position at: ({})", std::any::type_name::<T>());
}

fn main() {
    // import_test();
    bubble_sort_test();
}

fn import_test() {
    println_fn_name(import_test);
    sorts::hello_sorts();
}

fn bubble_sort_test() {
    println_fn_name(bubble_sort_test);
    let mut arr = vec![1, 3, 4, 8, 2, 7, 4, 0];
    println!("Input -> {:?}", arr);
    sorts::bubble_sort(&mut arr);
    println!("Result-> {:?}", arr);
}
