use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

fn main() {
    let thread_list: Arc<(Mutex<Vec<String>>, Condvar)> = Arc::new((Mutex::new(Vec::new()), Condvar::new())); 
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for number in 0..6 {
        let thread_list_clone: Arc<(Mutex<Vec<String>>, Condvar)> = Arc::clone(&thread_list);
        let handle: JoinHandle<()> = thread::spawn(move || {
            let (lock, cvar): &(Mutex<Vec<String>>, Condvar) = &*thread_list_clone;
            let mut guard: MutexGuard<'_, Vec<String>> = lock.lock().unwrap();
            guard.push(format!("Thread{}", number+1));
            //Send a notification to main thread
            cvar.notify_one();
        });
        handles.push(handle);
    }

    let (lock, cvar): &(Mutex<Vec<String>>, Condvar) = &*thread_list;
    let mut guard: MutexGuard<'_, Vec<String>> = lock.lock().unwrap();

    while guard.len() < 6 {
        //After calling cvar.wait(), main thread releases the lock of 'guard'　temporarily, 
        //so other threads can get a lock to push data.
        //When cvar.notify_one() is called in other threads,
        //main thread gets a lock again and check guard.len()
        guard = cvar.wait(guard).unwrap();
    }

    println!("Threads: {:?}", *guard);
}