use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let vals = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = vals.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("the val was: {value}");
        }
    });
}
