#[cfg(test)]
mod tests {
    use main::my_merge_sort::sort;
    use std::vec::Vec;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn struct_field_to_json() {
        let mut numbers: Vec<i32> = (1..=10).collect();
        let mut rng = thread_rng();
        numbers.shuffle(&mut rng);

        print!("Tests!");
        let vec: Vec<i32> = sort(numbers);
        let expected: Vec<i32> = (1..=10).collect();
        assert_eq!(vec, expected)
    }
}


