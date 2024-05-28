use std::vec::Vec;

pub fn sort<T: Clone + Ord + Copy>(xs: Vec<T>) -> Vec<T> {
    if xs.len() <= 1 {
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
        let vec0 = first.to_vec();
        let vec1 = second.to_vec();
        let x0: Vec<T> = sort(vec0);
        let x1: Vec<T> = sort(vec1);
        let mut sorted: Vec<T> = vec![xs[0]; size];
        merge(x0, x1, &mut sorted);
        return sorted;
    }
}

fn merge<T: Clone + Ord + Copy>(xs: Vec<T>, ys: Vec<T>, sorted: &mut Vec<T>) {
    let mut i = 0;
    let mut j = 0;
    let mut index = 0;
    while index < sorted.len() {
        if i < xs.len() && (j == ys.len() || xs[i] < ys[j]) {
            sorted[index] = xs[i];
            i += 1;
        } else {
            sorted[index] = ys[j];
            j += 1;
        }
        index += 1;
    }
}