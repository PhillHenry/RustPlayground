use std::vec::Vec;

pub fn sort<T: Clone + Ord + Copy>(xs: Vec<T>) -> Vec<T> {
    if xs.len() == 1 {
        return xs
    } else if xs.len() == 2 {
        if xs[0] < xs[1] {
            return xs
        } else {
            return vec![xs[1], xs[0]]
        }
    } else {
        let size = xs.len();
        let mid = size / 2;
        let (first, second) = xs.split_at(mid);
        let x0: Vec<T> = sort(first.to_vec());
        let x1: Vec<T> = sort(second.to_vec());
        let mut sorted: Vec<T> = Vec::with_capacity(size);
        let mut i = 0;
        let mut j = 0;
        let mut index = 0;
        while index < size {
            if i < x0.len() && (j == x1.len() || x0[i] < x1[j]) {
                sorted[index] = x0[i];
                i += 1;
            } else {
                sorted[index] = x1[j];
                j += 1;
            }
            index += 1;
        }
        return sorted;
    }
}