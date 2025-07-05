use std::time::Duration;

fn main() {
    trpl::run(async {
        // let handle = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi no {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });

        let fut1 = async {
            for i in 1..10 {
                println!("hi no {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi no {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        //handle.await.unwrap();
        
        // both futures run to completion
        trpl::join(fut1, fut2).await;
    });
}
