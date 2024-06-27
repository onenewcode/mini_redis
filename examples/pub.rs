use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // publish message `bar` on channel foo
    client.publish("foo", "bar".into()).await?;

    Ok(())
}
