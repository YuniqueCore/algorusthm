use std::usize;

pub fn hello_sorts() {
    println!("Hello Sorts");
}

pub mod bubble_sort {

    pub fn sort(arr: &mut [i32]) {
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
}

pub mod quick_sort {
    pub fn sort(arr: &mut [i32], left: i32, right: i32) {
        // terminate sort when the partial array has only one item
        if left >= right {
            return;
        }

        // Sentinel division
        let pivot = self::partition(arr, left as usize, right as usize) as i32;

        // Recursively sort the left and right part of the array
        sort(arr, left, pivot - 1);
        sort(arr, pivot + 1, right);
    }

    fn get_mid_value_index(arr: &mut [i32], left: usize, mid: usize, right: usize) -> usize {
        let (l, m, r) = (arr[left], arr[mid], arr[right]);
        if m <= l && l <= r || r <= l && l <= m {
            left
        } else if l <= m && m <= r || r <= m && m <= l {
            mid
        } else {
            right
        }
    }

    fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
        let m = get_mid_value_index(arr, left, (left + right) / 2, right);
        // swap the median to the left of array
        arr.swap(left, m);
        let (mut l, mut r) = (left, right);
        let flag = arr[left];
        while l < r {
            // find the first right item less than flag value
            // Due to the flag is left one, MUST find the bigger one firstly
            while l < r && arr[r] >= flag {
                r -= 1;
            }
            // find the first left item less than flag value
            while l < r && arr[l] <= flag {
                l += 1;
            }
            arr.swap(l, r);
        }
        arr.swap(l, left); // swap the left index value to l index value
        l // return flag index
    }
}
