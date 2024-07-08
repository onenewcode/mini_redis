#![warn(rust_2018_idioms)]

use mini_redis::{clients::Client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let client = Client::connect("127.0.0.1:6379").await?;

    // subscribe to channel foo
    let mut subscriber = client.subscribe(vec!["foo".into()]).await?;

    // await messages on channel foo
    if let Some(msg) = subscriber.next_message().await? {
        println!(
            "got message from the channel: {}; message = {:?}",
            msg.channel, msg.content
        );
    }

    Ok(())
}
