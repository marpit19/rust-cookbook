use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi no: {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // main thread will wait for the spawned thread to finish and then run its for loop
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi no: {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    
    // main thread waits because of the call to: handle.join()
    //handle.join().unwrap();
}
