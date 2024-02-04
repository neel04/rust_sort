pub trait Sorter {
    fn sort(&self, v: &[i32]) -> Vec<i32>;
}

pub struct BubbleSorter;
pub struct InsertionSorter;

impl Sorter for BubbleSorter {
    fn sort(&self, v: &[i32]) -> Vec<i32> {
        /* 
        Implements bubble sort algorithm
        */

        let mut sorted_vec = v.to_vec();
        let vec_length = sorted_vec.len();

        for i in 0..vec_length - 1 {
            for j in 0..vec_length - i - 1 {
                if sorted_vec[j] > sorted_vec[j+1] {
                    sorted_vec.swap(j, j+1);
                }
            }
        }

        sorted_vec
    }
}

impl Sorter for InsertionSorter {
    fn sort(&self, v: &[i32]) -> Vec<i32> {
        /*
        Implements insertion sort algorithm
        */

        let mut sorted_vec = v.to_vec();
        let vec_length = sorted_vec.len();

        for i in 1..vec_length {
            let key = sorted_vec[i];
            let mut j = i - 1;

            while sorted_vec[j] > key {
                sorted_vec.swap(j+1, j);

                match j.checked_sub(1) {
                    Some(val) => j = val,
                    None => break, // j becomes negative, exit the loop
                }
            }

        }

        sorted_vec

    }
}