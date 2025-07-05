use std::thread;

fn main() {
    let v = vec![1,2,3];
    
    // adding the move keyword before the closure, forces the closure to take
    // ownership of the value rather than borrowing
    let handle = thread::spawn(move ||{
        println!("vector: {v:?}");
    });
    
    handle.join().unwrap();
}