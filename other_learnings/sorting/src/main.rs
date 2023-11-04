use rand::Rng;
use std::time::SystemTime;
macro_rules! is_sorted {
    ($data:expr) => {
        is_sorted_pred($data, |x, y| x < y)
    };
    ($data:expr, $f:expr) => {
        is_sorted_pred($data, $f)
    };
}

fn main() {
    let size = 25000;
    println!("n size is {size}");

    let mut my_vec1 = gen_rand_vec(size);
    let mut my_vec2 = my_vec1.clone();
    let mut my_vec3 = my_vec1.clone();
    let mut my_vec4 = my_vec1.clone();
    let mut my_vec5 = my_vec1.clone();
    let mut my_vec6 = my_vec1.clone();

    let start = SystemTime::now();
    radix_sort(&mut my_vec1);
    let elapsed = start.elapsed().unwrap();
    println!(
        "radix sort took {elapsed:?}   sorted: {}",
        is_sorted!(my_vec1)
    );

    let start = SystemTime::now();
    quicksort(&mut my_vec2);
    let elapsed = start.elapsed().unwrap();
    println!(
        "quick sort took {elapsed:?}   sorted: {}",
        is_sorted!(my_vec2)
    );

    let start = SystemTime::now();
    block_sort(&mut my_vec3);
    let elapsed = start.elapsed().unwrap();
    //println!("{my_vec2:?}");
    println!(
        "block sort took {elapsed:?}   sorted: {}",
        is_sorted!(my_vec3)
    );

    let start = SystemTime::now();
    mergesort(&mut my_vec4);
    let elapsed = start.elapsed().unwrap();
    println!(
        "merge sort took {elapsed:?}   sorted: {}",
        is_sorted!(my_vec4)
    );

    let start = SystemTime::now();
    insertion_sort(&mut my_vec5);
    let elapsed = start.elapsed().unwrap();
    println!(
        "insertion sort took {elapsed:?}   sorted: {}",
        is_sorted!(my_vec5)
    );

    let start = SystemTime::now();
    bubble_sort(&mut my_vec6);
    let elapsed = start.elapsed().unwrap();
    println!(
        "bubble sort took {elapsed:?}   sorted: {}",
        is_sorted!(my_vec6)
    );
}

fn radix_sort(v: &mut Vec<i32>) {
    let mut loop_times: i32 = 1;
    let mut i = 0;
    while i < loop_times {
        let mut buckets: Vec<Vec<i32>> = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        let divisor = 10_i32.pow(i as u32);
        for j in v.iter() {
            if i == 0 && ((*j as f64).log10() as i32) + 1 > loop_times {
                loop_times = ((*j as f64).log10() as i32) + 1;
            }
            let bucket = ((j / divisor) % 10) as usize;
            buckets[bucket].push(*j);
        }
        let mut insert_idx = 0;
        *v = vec![];
        for b in buckets.iter() {
            for n in b.into_iter() {
                v.insert(insert_idx, *n);
                insert_idx += 1;
            }
        }
        i += 1;
    }
}

fn bubble_sort(v: &mut Vec<i32>) {
    let len = v.len() - 1;
    for i in 0..len {
        for j in 0..(len - i) {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }
}

fn quicksort(v: &mut Vec<i32>) {
    let low = 0;
    let high = v.len() - 1;
    quicksort_impl(v, low, high);
}

fn quicksort_impl(v: &mut Vec<i32>, low: usize, high: usize) {
    if low >= high {
        return;
    }
    let low_end = quicksort_partition(v, low, high);
    quicksort_impl(v, low, low_end);
    quicksort_impl(v, low_end + 1, high);
}

fn quicksort_partition(v: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let mid = low + (high - low) / 2;
    let pivot = v[mid];
    let mut done = false;
    let mut high = high;
    let mut low = low;
    while !done {
        while v[low] < pivot {
            low += 1;
        }
        while v[high] > pivot {
            high -= 1;
        }
        if low >= high {
            done = true;
        } else {
            v.swap(low, high);
            low += 1;
            high -= 1;
        }
    }
    high
}

fn mergesort(v: &mut Vec<i32>) {
    let high = v.len() - 1;
    if high < 1 {
        return;
    }
    if high == 1 {
        if v[0] > v[1] {
            v.swap(0, 1);
        }
        return;
    }
    let mid = high / 2;
    let mut left: Vec<i32> = v.drain(0..=mid).collect();
    let mut right: Vec<i32> = v.drain(0..).collect();
    mergesort(&mut left);
    mergesort(&mut right);
    merge(&mut left, &mut right, v);
}

fn merge(left: &mut Vec<i32>, right: &mut Vec<i32>, result: &mut Vec<i32>) {
    while left.len() > 0 && right.len() > 0 {
        if left[0] < right[0] {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }
    if left.len() > 0 {
        result.append(left);
    }
    if right.len() > 0 {
        result.append(right);
    }
}

fn block_sort(v: &mut Vec<i32>) {
    // make a block for each perf cpu
    let mut blocks: Vec<Vec<i32>> = vec![];
    let mut handles = vec![];
    let div = v.len() / 16;
    for _ in 0..16 {
        let mut block: Vec<i32> = v.drain(0..div).collect();
        handles.push(std::thread::spawn(|| {
            mergesort(&mut block);
            block
        }));
    }
    for handle in handles {
        blocks.push(handle.join().unwrap())
    }
    for block in blocks.iter_mut() {
        block_merge(v, block);
    }
}

fn block_merge(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    if v1.len() == 0 {
        v1.append(v2);
        return;
    }
    let mut insert_idx = 0;
    for n in v2.iter() {
        while insert_idx < v1.len() && n > &v1[insert_idx] {
            insert_idx += 1;
        }
        v1.insert(insert_idx, *n);
        insert_idx += 1;
    }
}

fn insertion_sort(v: &mut Vec<i32>) {
    for i in 1..v.len() {
        for j in (1..=i).rev() {
            if v[j - 1] < v[j] {
                break;
            }
            v.swap(j, j - 1);
        }
    }
}

fn gen_rand_vec(size: usize) -> Vec<i32> {
    let mut ret = vec![];
    for _ in 0..size {
        let i = rand::thread_rng().gen_range(0..10000);
        ret.push(i);
    }
    ret
}

fn is_sorted_pred<T: Ord, F: Fn(&T, &T) -> bool>(v: Vec<T>, pred: F) -> bool {
    let mut last = v.get(0).unwrap();
    for i in 1..v.len() {
        if pred(v.get(i).unwrap(), last) {
            return false;
        }
        last = &v[i];
    }
    true
}
