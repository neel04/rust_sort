// use algos.rs which is placed in src/algos.rs
use crate::algos::bubble_sort;

// Declare public modules
pub mod algos;

fn run_algo(vec: &Vec<i32>, sorting_fn: fn(&Vec<i32>) -> Vec<i32>, fn_name: String) -> Vec<i32> {
    println!("---- {} Sort ----", fn_name);
    println!("Unsorted array: {:?}", &vec);
    let sorted_vec = sorting_fn(vec);
    println!("Sorted array: {:?}", &sorted_vec);
    sorted_vec
}

fn main() {
    let my_vec = vec![10, 3, 5, 2, 1, 0, 4, 9, 8, 7, 6];
    let _sorted_vec = run_algo(&my_vec, bubble_sort, "Bubble".to_string());
}