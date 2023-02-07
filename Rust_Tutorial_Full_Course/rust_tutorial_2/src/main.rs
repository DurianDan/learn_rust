// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 2:15:00 to 2:32:57

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
//Atomically Reference Counted => allow shared ownership of a variable

fn main() {
    concurrency_basic_demo();
    concurrency_practical();
}

fn concurrency_practical(){
    struct Bank{
        balance: f32
    }

    fn withdraw(the_bank: Arc<Mutex<Bank>>, amount: f32){
        // Arc allow multiple ownership
        // Mutex block other threads, allowing only 1 thread to access a variable at a time
        let mut bank_reference = the_bank.lock().unwrap();
        if bank_reference.balance < 5.00{
            println!("Currenct balance: {}",bank_reference.balance);
        } else {
            bank_reference.balance -= amount;
            println!("Customer withdraw {}, \nCurrent balance: {}",
                    amount, bank_reference.balance)
        }
    }
    fn customer(the_bank: Arc<Mutex<Bank>>){
        withdraw(the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = 
        Arc::new(Mutex::new(Bank { balance: 100.0}));

    // create 10 threads
    let handles = (0..10).map(|_| {
        
        let bank_ref = bank.clone();
        // when use with .clone(), Arc automatically add a new ownership
        thread::spawn(||{customer(bank_ref)})
    });

    // join all thread to main
    for handle in handles{
        handle.join().unwrap();
    }
}

fn concurrency_basic_demo(){
    // concurrency
    let thread1 = thread::spawn(||{
        for i in 1..=10{
            println!("Spwaned thread: {}", i);
            // try remove the sleep command
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..=5{
        println!("Main thread: {}", i);
        // if main thread doesn't sleep, the CPU will always be busy with the main thread.
        // main thread, then, will execuse first, followed by spawned threads
        thread::sleep(Duration::from_millis(1))
    };
    
    // sometimes, the main thread finish execution before the spawned threads
    // then, the spawned threads will stop execution
    // use .join() to force spawned threads join the main thread, to finnish execution
    thread1.join().unwrap();
}