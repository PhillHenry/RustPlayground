mod my_merge_sort;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut s1 = String::from("hello");
    let len = main::calculate_length(&mut s1); // Borrowing s1 immutably
    println!("The length of '{}' is {}", s1, len); // s1 is still valid here
}

mod main {

    pub fn calculate_length(s: &mut String) -> usize {
        s.push_str("mutant");
        s.len()
    }
}
