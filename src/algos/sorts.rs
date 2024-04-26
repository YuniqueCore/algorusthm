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

pub mod heap_sort {

    /* 堆排序 */
    pub fn sort(nums: &mut [i32]) {
        // 建堆操作：堆化除叶节点以外的其他所有节点
        for i in (0..=nums.len() / 2 - 1).rev() {
            sift_down(nums, nums.len(), i);
        }
        // 从堆中提取最大元素，循环 n-1 轮
        for i in (1..=nums.len() - 1).rev() {
            // 交换根节点与最右叶节点（交换首元素与尾元素）
            let tmp = nums[0];
            nums[0] = nums[i];
            nums[i] = tmp;
            // 以根节点为起点，从顶至底进行堆化
            sift_down(nums, i, 0);
        }
    }

    /* 堆的长度为 n，从节点 i 开始，从顶至底堆化 */
    fn sift_down(nums: &mut [i32], n: usize, mut i: usize) {
        loop {
            // 判断节点 i, l, r 中值最大的节点，记为 ma
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            let mut ma = i;
            if l < n && nums[l] > nums[ma] {
                ma = l;
            }
            if r < n && nums[r] > nums[ma] {
                ma = r;
            }
            // 若节点 i 最大或索引 l, r 越界，则无须继续堆化，跳出
            if ma == i {
                break;
            }
            // 交换两节点
            let temp = nums[i];
            nums[i] = nums[ma];
            nums[ma] = temp;
            // 循环向下堆化
            i = ma;
        }
    }
    pub fn sort_1(arr: &mut [i32]) {
        for i in (0..=(arr.len() / 2 - 1)).rev() {
            shift_down_1(arr, arr.len(), i);
        }

        for i in (1..=(arr.len() - 1)).rev() {
            let tmp = arr[0];
            arr[0] = arr[i];
            arr[i] = tmp;

            shift_down_1(arr, i, 0);
        }
    }

    fn shift_down_1(arr: &mut [i32], n: usize, mut i: usize) {
        loop {
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            let mut max = i;
            if l < n && arr[l] > arr[max] {
                max = l;
            }
            if r < n && arr[r] > arr[max] {
                max = r;
            }

            if max == i {
                break;
            }
            let tmp = arr[i];
            arr[i] = arr[max];
            arr[max] = tmp;
            // arr.swap(i, max);
            i = max;
        }
    }
}

pub mod buket_sort {

    pub fn sort(nums: &mut [i32]) {
        println!("normlization: {:?}", min_max_normalization(nums));
        let k = nums.len() / 2;

        let mut buckets = vec![vec![]; k];
        let len = nums.len();

        for i in 0..len {
            let h = (normalization(nums, nums[i]).unwrap() * k as f64) as usize;
            println!("i is {}, h is {}", i, h);
            buckets[h].push(nums[i]);
        }

        for i in 0..buckets.len() {
            buckets[i].sort_by(|a, b| a.partial_cmp(b).unwrap());
        }

        let mut num_index = 0;
        for bucket in &mut buckets {
            for &mut num in bucket {
                nums[num_index] = num;
                num_index += 1;
            }
        }
    }

    fn normalization(data: &mut [i32], num: i32) -> Option<f64> {
        if data.is_empty() {
            return None; // 返回空向量
        }

        let (min_val, max_val) = data.iter().fold((i32::MAX, i32::MIN), |(min, max), &x| {
            (min.min(x), max.max(x))
        });

        // if min_val == max_val {
        //     // 如果最小值和最大值相等，无法进行归一化
        //     return None; // 或者返回包含单个元素的向量，取决于您的需求
        // }

        let normalized = (num - min_val) as f64 / max_val as f64;
        Some(normalized)
    }

    fn min_max_normalization(data: &mut [i32]) -> Option<Vec<f64>> {
        if data.is_empty() {
            return None; // 或者返回空向量，取决于您的需求
        }

        let (min_val, max_val) = data.iter().fold((i32::MAX, i32::MIN), |(min, max), &x| {
            (min.min(x), max.max(x))
        });

        // if min_val == max_val {
        //     // 如果最小值和最大值相等，无法进行归一化
        //     return None; // 或者返回包含单个元素的向量，取决于您的需求
        // }

        Some(
            data.iter()
                .map(|&x| {
                    let normalized = (x - min_val) as f64 / max_val as f64;
                    // 如果需要更高的精度，可以使用 `f64::from(x - min_val) / f64::from(max_val - min_val)`
                    normalized
                })
                .collect(),
        )
    }
}
