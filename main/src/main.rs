mod my_merge_sort;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut s1 = String::from("hello");
    let len = main::calculate_length(&mut s1); // Borrowing s1 immutably
    println!("The length of '{}' is {}", s1, len); // s1 is still valid here

    println!("returned = {}", main::mutate_in_own_thread(s1));
    // println!("s1 = {} ", s1); // <-- can no longer use s1
}

mod main {

    pub fn calculate_length(s: &mut String) -> usize {
        s.push_str("mutant");
        s.len()
    }

    use std::thread;
    //use crate::thread;

    pub fn mutate_in_own_thread(mut s: String) -> usize {
        //s.push_str("mutant");
        let handle = thread::spawn(move || {
            s.push_str("mutant");
        });
        1 // s.len()
    }
}
