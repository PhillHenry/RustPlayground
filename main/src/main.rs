mod my_merge_sort;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut s1 = String::from("hello");

    let s_mutated = main::mutate(&mut s1);
    // this is allowed:
    println!("s_mutated = {}", s_mutated);
    println!("s1 = {}", s1);
    // but if the above 2 lines were replaced with:
    //println!("s_mutated = {}, s1 = {}", s_mutated, s1);
    // it would not be allowed!
    // The reason is actually simple. Once s_mutated has been 'spent' there are no longer any
    // mutable references to s1, therefore s1 can be used once more.

    let len = main::mutate_length(&mut s1);
    println!("The length of '{}' is {}", s1, len); // s1 is still valid here

    println!("returned = {}", main::mutate_in_own_thread(s1));
    // println!("s1 = {} ", s1); // <-- can no longer use s1
}

mod main {
    pub fn mutate(s: &mut String) -> &mut String {
        s.push_str(", mutant");
        return s
    }

    /**
    s: &mut String means `s` is a reference that is only valid in the function body
    */
    pub fn mutate_length(s: &mut String) -> usize {
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
        // println!("s = {}", s); // s has now been lost into the ether
        1 // s.len()
    }
}
