use mini_redis::{client, Result};
#[tokio::main]
pub async fn main() -> Result<()> {
    // 与mini-redis服务器建立连接
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 客户端写入值
    client.set("hello", "world".into()).await?;

    // 获取值
    // 如果键不存在，则返回None
    let result = client.get("hello").await?;

    println!("got value from the server; success={:?}", result.is_some());

    Ok(())
}
