
use mini_redis::{client, Result};
use tokio_stream::StreamExt;
#[tokio::main]
pub async fn main() -> Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;

    // subscribe to channel foo
    let mut subscriber = client.subscribe(vec!["foo".into()]).await?;
    let messages = subscriber.into_stream();
    tokio::pin!(messages);
    // await messages on channel foo
    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}
