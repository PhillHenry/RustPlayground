use std::vec::Vec;

pub fn sort<T: Clone + Ord + Copy + ToString + std::fmt::Debug>(xs: Vec<T>) -> Vec<T> {
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
        println!("mid = {}, szie = {}, first = {}, second = {}", mid, size,
                 first.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "),
                 second.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "));
        let x0: Vec<T> = sort(vec0);
        let x1: Vec<T> = sort(vec1);
        let mut sorted: Vec<T> = vec![xs[0]; size];
        let mut i = 0;
        let mut j = 0;
        let mut index = 0;
        println!("x0 = {:?}, x1 = {:?}, sorted = {:?}", x0, x1, sorted);
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