use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // 建立与mini-redis服务器的连接
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("从服务器端获取到结果={:?}", result.is_some());

    Ok(())
}