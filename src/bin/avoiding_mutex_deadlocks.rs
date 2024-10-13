use std::sync::{Mutex, Arc, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Duration;

//Thread 1 locks languages and then tries to lock fields.
//Thread 2 locks fields first and then tries to lock languages.
//Thread 1 will be waiting for being able to lock fields, locking languages and 
//Thread 2 will be waiting for being able to lock languages, locking fields.
//This leads to deadlocks. 

fn main() {
    let languages: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let fields: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));

    let languages_result: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let fields_result: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));

    let handles: Vec<JoinHandle<(String, String)>> = vec![
        thread::spawn({
            let languages_clone: Arc<Mutex<String>> = Arc::clone(&languages);
            let fields_clone: Arc<Mutex<String>> = Arc::clone(&fields);
            move || {
                let mut guard1: MutexGuard<'_, String> = languages_clone.lock().unwrap();
                *guard1 += "Rust/";
                //Add a sleep
                thread::sleep(Duration::from_millis(10)); 
                let mut guard2: MutexGuard<'_, String> = fields_clone.lock().unwrap();
                *guard2 += "System programming/";
                (String::from("C++/"), String::from("embedded system/"))
            }
        }),
        thread::spawn({
            let languages_clone: Arc<Mutex<String>> = Arc::clone(&languages);
            let fields_clone: Arc<Mutex<String>> = Arc::clone(&fields);
            move || {
                let mut guard2: MutexGuard<'_, String> = fields_clone.lock().unwrap();
                *guard2 += "Data science/";
                let mut guard1: MutexGuard<'_, String> = languages_clone.lock().unwrap();
                *guard1 += "Python/";

                //***** need to lock in consistent order *****
                // let mut guard1: MutexGuard<'_, String> = languages_clone.lock().unwrap();
                // *guard1 += "Python/";
                // let mut guard2: MutexGuard<'_, String> = fields_clone.lock().unwrap();
                // *guard2 += "Data science/";

                (String::from("Golang/"), String::from("Microservices/"))
            }
        }),
    ];

    let results: Vec<(String, String)> = handles
        .into_iter()
        .map(|handle: JoinHandle<(String, String)>| handle.join().unwrap())
        .collect::<Vec<(String, String)>>();

    let returned_languages: String = results.iter().map(|(language, _): &(String, String)| language.clone()).collect();
    let returned_fields: String = results.iter().map(|(_, field): &(String, String)| field.clone()).collect();

    {
        let mut guard1_result: MutexGuard<'_, String> = languages_result.lock().unwrap();
        let mut guard2_result: MutexGuard<'_, String> = fields_result.lock().unwrap();
        *guard1_result = languages.lock().unwrap().clone() + &returned_languages;
        *guard2_result = fields.lock().unwrap().clone() + &returned_fields;
    }

    println!("Computer languages: {}", *languages_result.lock().unwrap());
    println!("Main field: {}", *fields_result.lock().unwrap());
}