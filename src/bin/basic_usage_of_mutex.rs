use std::sync::{Mutex, MutexGuard};

fn main() {
    let text: Mutex<String> = Mutex::new(String::from("Rust "));
    {
        //Get a lock to modify the value
        let mut guard: MutexGuard<'_, String> = text.lock().unwrap();
        guard.push_str("is a pretty interesting programming language.");
    }
    //Lock is released after guard goes out of scope

    //Get a lock again to use value
    println!("text: {}", text.lock().unwrap());
}