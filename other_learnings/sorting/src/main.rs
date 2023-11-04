use rand::Rng;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::{Duration, SystemTime};
macro_rules! is_sorted {
    ($data:expr) => {
        is_sorted_pred($data, |x, y| x < y)
    };
    ($data:expr, $f:expr) => {
        is_sorted_pred($data, $f)
    };
}

struct Result {
    name: String,
    time: Duration,
    sorted: bool,
}

fn main() {
    let mut size = 1000;
    let mut results: HashMap<usize, Vec<Result>> = HashMap::default();
    let mut radix_valid = true;
    let mut quick_valid = true;
    let mut block_valid = true;
    let mut merge_valid = true;
    let mut shell_valid = true;
    let mut insertion_valid = true;
    let mut selection_valid = true;
    let mut bubble_valid = true;
    while radix_valid
        || quick_valid
        || block_valid
        || merge_valid
        || shell_valid
        || insertion_valid
        || selection_valid
        || bubble_valid
    {
        let mut my_vec1 = gen_rand_vec(size);
        let mut my_vec2 = my_vec1.clone();
        let mut my_vec3 = my_vec1.clone();
        let mut my_vec4 = my_vec1.clone();
        let mut my_vec5 = my_vec1.clone();
        let mut my_vec6 = my_vec1.clone();
        let mut my_vec7 = my_vec1.clone();
        let mut my_vec8 = my_vec1.clone();

        if radix_valid {
            let start = SystemTime::now();
            radix_sort(&mut my_vec1);
            let elapsed = start.elapsed().unwrap();
            results.insert(
                size,
                vec![Result {
                    name: "radix sort".to_string(),
                    time: elapsed,
                    sorted: is_sorted!(my_vec1),
                }],
            );
            if elapsed.as_secs() >= 60 {
                radix_valid = false;
            }
        }

        if quick_valid {
            let start = SystemTime::now();
            quicksort(&mut my_vec2);
            let elapsed = start.elapsed().unwrap();
            results.get_mut(&size).unwrap().push(Result {
                name: "quicksort".to_string(),
                time: elapsed,
                sorted: is_sorted!(my_vec2),
            });
            if elapsed.as_secs() >= 60 {
                quick_valid = false;
            }
        }

        if block_valid {
            let start = SystemTime::now();
            block_sort(&mut my_vec3);
            let elapsed = start.elapsed().unwrap();
            results.get_mut(&size).unwrap().push(Result {
                name: "block sort".to_string(),
                time: elapsed,
                sorted: is_sorted!(my_vec3),
            });
            if elapsed.as_secs() >= 60 {
                block_valid = false;
            }
        }

        if merge_valid {
            let start = SystemTime::now();
            mergesort(&mut my_vec4);
            let elapsed = start.elapsed().unwrap();
            results.get_mut(&size).unwrap().push(Result {
                name: "mergesort".to_string(),
                time: elapsed,
                sorted: is_sorted!(my_vec4),
            });
            if elapsed.as_secs() >= 60 {
                merge_valid = false;
            }
        }

        if shell_valid {
            let start = SystemTime::now();
            shell_sort(&mut my_vec5);
            let elapsed = start.elapsed().unwrap();
            results.get_mut(&size).unwrap().push(Result {
                name: "shell sort".to_string(),
                time: elapsed,
                sorted: is_sorted!(my_vec5),
            });
            if elapsed.as_secs() >= 60 {
                shell_valid = false;
            }
        }

        if insertion_valid {
            let start = SystemTime::now();
            insertion_sort(&mut my_vec6);
            let elapsed = start.elapsed().unwrap();
            results.get_mut(&size).unwrap().push(Result {
                name: "insertion sort".to_string(),
                time: elapsed,
                sorted: is_sorted!(my_vec6),
            });
            if elapsed.as_secs() >= 60 {
                insertion_valid = false;
            }
        }

        if selection_valid {
            let start = SystemTime::now();
            selection_sort(&mut my_vec7);
            let elapsed = start.elapsed().unwrap();
            results.get_mut(&size).unwrap().push(Result {
                name: "selection sort".to_string(),
                time: elapsed,
                sorted: is_sorted!(my_vec7),
            });
            if elapsed.as_secs() >= 60 {
                selection_valid = false;
            }
        }

        if bubble_valid {
            let start = SystemTime::now();
            bubble_sort(&mut my_vec8);
            let elapsed = start.elapsed().unwrap();
            results.get_mut(&size).unwrap().push(Result {
                name: "bubble sort".to_string(),
                time: elapsed,
                sorted: is_sorted!(my_vec8),
            });
            if elapsed.as_secs() >= 60 {
                bubble_valid = false;
            }
        }

        let v = results.get_mut(&size).unwrap();
        v.sort_by_key(|e| e.time);
        let mut place = 1;
        println!("\n\nn size: {size}");
        for rec in v {
            println!(
                "{place}. {}: {:?}, sorted: {}",
                rec.name, rec.time, rec.sorted
            );
            place += 1;
        }

        size *= 2;
    }
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
    for i in 0..16 {
        let mut block: Vec<i32> = v.drain(0..div).collect();
        if i == 15 {
            let mut rem: Vec<i32> = v.drain(0..).collect();
            block.append(&mut rem);
        }
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

fn shell_sort(v: &mut Vec<i32>) {
    let mut gap_vals = vec![];
    let mut val = 1;
    let mut pow = 2;
    while val < v.len() {
        gap_vals.push(val);
        val = 2_usize.pow(pow) - 1;
        pow += 1;
    }
    for val in gap_vals {
        for i in 0..val {
            insertion_sort_interleaved(v, i, val);
        }
    }
}
fn insertion_sort_interleaved(v: &mut Vec<i32>, start: usize, gap: usize) {
    for i in (start + gap..v.len()).step_by(gap) {
        let mut j = i;
        while j >= gap && (j - gap) >= start && v[j] < v[j - gap] {
            v.swap(j, j - gap);
            j -= gap;
        }
    }
}

fn selection_sort(v: &mut Vec<i32>) {
    for i in 0..v.len() - 1 {
        let mut smallest = i;
        for j in i + 1..v.len() {
            if v[j] < v[smallest] {
                smallest = j;
            }
        }
        v.swap(i, smallest);
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

fn is_sorted_pred<T: Ord + Display, F: Fn(&T, &T) -> bool>(v: Vec<T>, pred: F) -> bool {
    let mut last = v.get(0).unwrap();
    for i in 1..v.len() {
        if pred(v.get(i).unwrap(), last) {
            println!("current = {}, last = {last}", v.get(i).unwrap());
            return false;
        }
        last = &v[i];
    }
    true
}
