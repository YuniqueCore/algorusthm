pub fn hello_sorts() {
    println!("Hello Sorts");
}

pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let mut flag = false;
        for j in 0..n - i {
            let next = arr[j + 1];
            let cur = arr[j];
            // println!(
            //     "j.({}) -> old arr[j+1] = {}, arr[j] = {} -> arr: {:?}",
            //     j, next, cur, arr
            // );
            if cur > next {
                let tmp = cur;
                arr[j] = next;
                arr[j + 1] = tmp;
                flag = true;
            }
            // println!(
            //     "  new arr[j+1] = {}, arr[j] = {} -> arr: {:?}",
            //     arr[j + 1],
            //     arr[j],
            //     arr
            // );
        }
        println!("i.({}) -> {:?}", i, arr);
        // flag is false means no exchange event happened
        // so break current inner iteration
        if !flag {
            break;
        }
    }
}
