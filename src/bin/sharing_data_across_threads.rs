use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

fn main() {
    let result: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    let languages: Vec<&str> = vec!["Rust", "Golang", "Python"];
    for (index, language) in languages.into_iter().enumerate() {
        //Unlike a typical clone operation, which creates a deep copy of the data, 
        //Arc::clone() only increments the reference count and creates a new reference to the same data.
        let cloned_result: Arc<Mutex<String>> = Arc::clone(&result);
        let handle: JoinHandle<()> = thread::spawn(move || {
            println!("Thread{}: pushed data is {}", index + 1, language);
            let mut guard: MutexGuard<'_, String> = cloned_result.lock().unwrap();
            guard.push_str(&format!("{} ", language));
        });
        handles.push(handle)
    }

    //Waiting for all threads to complete each execution using join()
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", result.lock().unwrap());
}