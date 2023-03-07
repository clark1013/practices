use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut cli = client::connect("127.0.0.1:6379").await?;
    cli.set("hello", "world".into()).await?;
    let result = cli.get("hello").await?;
    println!("get hello return {:?}", result);
    Ok(())
}
