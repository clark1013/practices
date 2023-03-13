use mini_redis::client;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }


    // // Establish a connection to the server
    // let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // // Spawn two tasks, one gets a key, the other sets a key
    // let t1 = tokio::spawn(async {
    //     let res = client.get("foo").await;
    // });

    // let t2 = tokio::spawn(async {
    //     client.set("foo", "bar".into()).await;
    // });

    // t1.await.unwrap();
    // t2.await.unwrap();
}
