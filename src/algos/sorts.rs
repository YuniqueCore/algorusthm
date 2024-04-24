pub fn hello_sorts() {
    println!("Hello Sorts");
}

pub mod selection_sort {

    pub fn sort(arr: &mut [i32]) {
        for l in 0..arr.len() {
            let mut min_index = l;
            for next in l + 1..arr.len() {
                if arr[next] < arr[min_index] {
                    min_index = next;
                }
            }
            arr.swap(l, min_index);
        }
    }
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
    use std::fmt::Debug;

    use crate::utils::{FromUsize, InParams, ToUsize};

    pub fn sort_with_params<T: Debug + Ord + ToUsize + FromUsize + Copy>(
        arr: &mut [T],
        in_params: InParams<T>,
    ) {
        let InParams::Params(params) = in_params;
        let (left, right) = (params[0], params[1]);
        // terminate sort when the partial array has only one item
        if left >= right {
            return;
        }

        // Sentinel division
        let pivot = self::partition(arr, left.to_usize(), right.to_usize());

        // Recursively sort the left and right part of the array
        sort_with_params(arr, InParams::Params(vec![left, T::from_usize(pivot - 1)]));
        sort_with_params(arr, InParams::Params(vec![T::from_usize(pivot + 1), right]));
    }

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

    fn get_mid_value_index<T: Debug + Ord + Copy>(
        arr: &mut [T],
        left: usize,
        mid: usize,
        right: usize,
    ) -> usize {
        let (l, m, r) = (arr[left], arr[mid], arr[right]);
        if m <= l && l <= r || r <= l && l <= m {
            left
        } else if l <= m && m <= r || r <= m && m <= l {
            mid
        } else {
            right
        }
    }

    fn partition<T: Debug + Ord + Copy>(arr: &mut [T], left: usize, right: usize) -> usize {
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

pub mod insertion_sort {

    pub fn sort(arr: &mut [i32]) {
        let mut iter_times = 0;
        for i in 1..arr.len() {
            let current = arr[i];
            for j in 0..i {
                if arr[i] < arr[j] {
                    let mut last = i;
                    while last > j {
                        arr[last] = arr[last - 1];
                        last -= 1;
                        iter_times += 1;
                    }
                    arr[j] = current;
                }
            }
        }
        println!("sort total iteration times: {}", iter_times);
    }

    pub fn sort_example(arr: &mut [i32]) {
        let mut iter_times = 0;
        for i in 1..arr.len() {
            let (base, mut j) = (arr[i], (i - 1) as i32);
            while j >= 0 && arr[j as usize] > base {
                arr[(j + 1) as usize] = arr[j as usize]; // move the arr[j]=arr[i-1] to next arr[i]
                j -= 1;
                iter_times += 1;
            }
            arr[(j + 1) as usize] = base;
        }
        println!("sort example total iteration times: {}", iter_times);
    }
}

pub mod merge_sort {
    use std::vec;

    pub fn sort(arr: &mut [i32], left: usize, right: usize, depth: usize) {
        // End recursive at only one item
        if left >= right {
            return;
        }

        // Split stage
        let middle = (left + right) / 2;
        self::sort(arr, left, middle, depth + 1);
        self::sort(arr, middle + 1, right, depth + 1);
        // merge stage
        self::merge(arr, left, middle, right);
        println!(
            "Depth: {} --- left: {} --- right: {} --- Array: {:?}",
            depth, left, right, arr
        );
    }

    fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize) {
        let (mut i, mut j, mut k) = (left, mid + 1, 0);
        let tmp_len = right - left + 1;
        let mut tmp = vec![0; tmp_len];

        while i <= mid && j <= right {
            if arr[i] <= arr[j] {
                tmp[k] = arr[i];
                i += 1;
            } else {
                tmp[k] = arr[j];
                j += 1;
            }
            k += 1;
        }

        while i <= mid {
            tmp[k] = arr[i];
            i += 1;
            k += 1;
        }

        while j <= right {
            tmp[k] = arr[j];
            j += 1;
            k += 1;
        }

        // move the tmp to the original array
        for k in 0..tmp_len {
            arr[left + k] = tmp[k];
        }
        println!("tmp in merge: {:?}", tmp);
    }
}
