use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::thread::{self, JoinHandle};

fn main() {
    let message: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for number in 1..6 {
        let cloned_message :Arc<Mutex<String>>= Arc::clone(&message);
        let handle: JoinHandle<()> = thread::spawn(move || {
            //If Mutex is in poisoned state by occuring a panic in other threads,
            //lock() returns PoisonError. 
            //ex) Result<MutexGuard<'_, String>, PoisonError<MutexGuard<'_, String>>> = message.lock();
            match cloned_message.lock() {
                Ok(mut guard) => {
                    if number == 2 {
                        //cause an error　intentionally 
                        panic!("Intentional panic in Thread 2");
                    }
                    *guard += &format!("Hello world from Thread{}//", number);
                },
                Err(..) => {
                    eprintln!("Failed to lock cloned_message in Thread {}. Mutex is in poisoned state.", number)
                },
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        //If the associated thread panics, [Err] is returned 
        if let Err(err) = handle.join() {
            if let Some(message) = err.downcast_ref::<&str>() {
                eprintln!("A thread panicked with message: {}", message);
            } else {
                eprintln!("A thread panicked, but the message was not a &str.");
            }
        }
    }

    // Handle poisoned state
    let result: Result<MutexGuard<'_, String>, PoisonError<MutexGuard<'_, String>>> = message.lock();
    match result {
        Ok(guard) => println!("Message: {:?}", *guard),
        Err(poisoned) => {
            let guard: MutexGuard<'_, String> = poisoned.into_inner();
            println!("Recovered from poisoned state: {:?}", *guard);
        }
    }
}
