use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    // spawned thread needs to ownt he transmitter to be able to send messages through the channel
    thread::spawn(move || {
        //let val = String::from("hi");
        //tx.send(val).unwrap();

        let vals = vec![
            String::from("hi"),
            String::from("what"),
            String::from("pancakes"),
            String::from("ok"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("sending"),
            String::from("more"),
            String::from("messages"),
            String::from("okay"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv will block the main thread's execution and wait until a value issent down the channel
    // try_recv method deosn't block but will instead return Result<T, E>
    //let received = rx.recv().unwrap();

    for received in rx {
        println!("got: {received}");
    }
}
