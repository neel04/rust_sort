// Get Sorter trait
use algos::Sorter;

// Declare public modules
pub mod algos;

fn run_algo(vec: &[i32], sorter: &dyn Sorter, fn_name: &str) -> Vec<i32> {
    let sorted_vec = sorter.sort(vec);

    println!("---- {} Sort ----", fn_name);
    println!("Unsorted array: {:?}", &vec);
    println!("Sorted array: {:?}", &sorted_vec);

    sorted_vec
}

fn main() {
    let my_slice = &[3, 5, 2, 1, 0, 4, 9, 8, 7, 6];

    // Get all the algos
    let bubble_sort = algos::BubbleSorter;
    let insertion_sort = algos::InsertionSorter;

    // Run the algos
    let _ = run_algo(my_slice, &bubble_sort, "Bubble");
    let _ = run_algo(my_slice, &insertion_sort, "Insertion");
}