use std::{pin::Pin, time::Duration};

fn main() {
    // trpl::run(async {
    //     let (tx, mut rx) = trpl::channel();

    //     let val = String::from("hi");
    //     tx.send(val).unwrap();

    //     let received = rx.recv().await.unwrap();
    //     println!("got: {received}");
    // });

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx_fut1 = async move {
            let vals = vec![
                String::from("hi"),
                String::from("hola"),
                String::from("idk"),
                String::from("ok"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // keep waiting until we determine that there are no more messages
        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("got: {val}");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("hi2"),
                String::from("hola2"),
                String::from("idk2"),
                String::from("ok2"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx_fut1), Box::pin(rx_fut), Box::pin(tx_fut)];

        trpl::join_all(futures).await;
    });
}
