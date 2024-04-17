pub fn swap_num(a: &mut i32, b: &mut i32) {
    println!("a = a + b = {} + {}", a, b);
    *a = *a + *b;
    println!("b = a - b  = {} - {}", a, b);
    *b = *a - *b;
    println!("a = a - b = {} - {}", a, b);
    *a = *a - *b;
}
