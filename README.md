# 概述
## 客户端
客户端的代码都在examples中

# tokio 使用
## 注意事项
### 'static 约束
当使用 Tokio 创建一个任务时，该任务类型的生命周期必须是 'static。意味着，在任务中不能使用外部数据的引用:
```rs
use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    task::spawn(async {
        println!("Here's a vec: {:?}", v);
    });
}
```
上面代码中，spawn 出的任务引用了外部环境中的变量 v ，导致以下报错:

```shell
error[E0373]: async block may outlive the current function, but
              it borrows `v`, which is owned by the current function
 --> src/main.rs:7:23
  |
7 |       task::spawn(async {
  |  _______________________^
8 | |         println!("Here's a vec: {:?}", v);
  | |                                        - `v` is borrowed here
9 | |     });
  | |_____^ may outlive borrowed value `v`
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:7:17
  |
7 |       task::spawn(async {
  |  _________________^
8 | |         println!("Here's a vector: {:?}", v);
9 | |     });
  | |_____^
help: to force the async block to take ownership of `v` (and any other
      referenced variables), use the `move` keyword
  |
7 |     task::spawn(async move {
8 |         println!("Here's a vec: {:?}", v);
9 |     });
  |
```

# 依赖
## bytes 
我们使用 Vec<u8> 来保存目标数据，但是它有一个问题，对它进行克隆时会将底层数据也整个复制一份，效率很低，但是克隆操作对于我们在多连接间共享数据又是必不可少的。

因此这里咱们新引入一个 bytes 包，它包含一个 Bytes 类型，当对该类型的值进行克隆时，就不再会克隆底层数据。事实上，Bytes 是一个引用计数类型，跟 Arc 非常类似，或者准确的说，Bytes 就是基于 Arc 实现的，但相比后者Bytes 提供了一些额外的能力。

cargo run --bin mini-redis-cli set foo bar

# 执行流程
首先执行servic中的run方法，然后构建一个Listener，作为服务器监听器，