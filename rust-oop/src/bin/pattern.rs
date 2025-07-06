fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_var @ 3..=7 } => println!("found id in range: {id_var}"),
        Message::Hello { id: 10..=12 } => {
            println!("found an id in another range")
        }
        Message::Hello { id } => println!("found some other id: {id}"),
    }
}
