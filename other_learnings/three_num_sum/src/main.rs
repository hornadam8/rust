use std::collections::HashMap;
use std::time::SystemTime;

fn main() {
    let target = 33;
    let arr = vec![10, 11, 5, 7, 6, 6, 9, 1, 13, 6, 8, 4, 10, 9, 12, 13, 7, 1, 12, 13, 3, 15, 2, 5, 15, 14, 13, 14, 11, 10, 3, 2, 8, 4, 13, 5, 14, 12, 6, 14, 5, 14, 11, 2, 13, 12, 7, 13, 11, 14, 14, 10, 12, 3, 4, 14, 4, 4, 13, 6, 7, 3, 3, 1, 9, 14, 5, 8, 3, 8, 6, 4, 2, 3, 15, 12, 8, 11, 2, 10, 3, 5, 11, 14, 7, 12, 10, 11, 2, 11, 3, 15, 3, 13, 8, 13, 2, 7, 8, 6];
    let start = SystemTime::now();
    let groups = three_num_sum(target , arr);
    let elapsed = start.elapsed().unwrap();
    println!("Found in {elapsed:?}");
    println!("Got {} groups", groups.len());
}


fn three_num_sum(target: i32, arr: Vec<i32>) -> Vec<(i32, i32, i32)> {
    let mut sum_groups = vec![];
    let mut count = 0;
    for i in 0..arr.len() - 2 {
        for j in i+1..arr.len() - 1 {
            for k in i+2..arr.len() {
                count += 1;
                let (a,b,c) = (arr[i], arr[j], arr[k]);
                let sum = a + b + c;
                if sum == target {
                    sum_groups.push((a,b,c));
                }
            }
        }
    }
    println!("Took {count} steps");
    return sum_groups;
}