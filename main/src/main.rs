mod my_merge_sort;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut s1 = String::from("hello");

    main::non_mutation(&s1);

    let mut s_mutated = main::mutate(&mut s1);
    // this is allowed IN THIS ORDER:
    println!("s_mutated = {}", s_mutated);
    s_mutated.push_str("XXX"); // allowed as we have not reused s1 yet

    println!("s1 = {}", s1); // s1 has been mutated!
    // println!("s_mutated = {}", s_mutated); <-- this would fail too. Seems that Rust is very pragmatic and if a violation doesn't cause a problem, allows it
    // but if the above 2 uncommented lines were replaced with:
    // println!("s_mutated = {}, s1 = {}", s_mutated, s1);
    // it would not be allowed!
    // The reason is actually simple. Once s_mutated has been 'spent' there are no longer any
    // mutable references to s1, therefore s1 can be used once more.

    // s_mutated.push_str("XXX"); // <-- not allowed as we've already used s1 again
    s1.push_str(" (polite interruption) ");

    let len = main::mutate_length(&mut s1);
    println!("The length of '{}' is {}", s1, len); // s1 is still valid here

    let handle = main::mutate_in_own_thread(s1);
    println!("returned = {}", handle.join().unwrap());
    // println!("s1 = {} ", s1); // <-- can no longer use s1

    let maybe_fail = main::fails_on_odds(2);
    println!("{:?}", maybe_fail);
}

mod main {

    pub fn fails_on_odds(x: i32) -> Result<String, String> {
        if x % 2 == 0 {
            return Ok(format!("{x} is OK"))
        }
        return Err(format!("{x} is odd"))
    }

    pub fn non_mutation(s: &String) { // & means I'm borrowing it but caller can continue using it
        println!("I'm not mutating {}", s);
    }

    pub fn mutate(s: &mut String) -> &mut String { // &mut seems to say "I'm borrowing it AND mutating it"
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
    use std::thread::JoinHandle;

    pub fn mutate_in_own_thread(mut s: String) -> JoinHandle<String> {
        //s.push_str("mutant");
        let handle = thread::spawn( || { // 'move' appears to be optional...
            s.push_str("mutant");
            return s
        });
        // println!("s = {}", s); // "move occurs because `s` has type `String`, which does not implement the `Copy` trait"
        return handle
    }
}
