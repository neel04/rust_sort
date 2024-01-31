pub trait Sorter {
    fn sort(&self, v: &[i32]) -> Vec<i32>;
}

pub struct BubbleSorter;

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