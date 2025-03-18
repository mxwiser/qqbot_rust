## 一个基于Rust的QQ官方机器人基本框架
- [✓] 支持web_hook验证
- [✓] 支持消息去重
- [✓] 支持群聊、私聊消息接收
- [✓] 支持群聊、私聊文本消息回复
- [] 支持图文消息发送
- [] 未完待续...
> 基础功能框架已经实现，其他功能基于框架开发即可。
> 使用前请将bot.env.example重命名为bot.env，并配置 bot.env 中的信息 ，同时确保运行环境能够找到bot.env文件. 代码使用参考main.rs
> 本库使用Tokio运行时，监听的API路径为 "/"

```bash
docker config
rustup target add x86_64-unknown-linux-musl 
apt-get install musl-tools 
cargo build --release --target x86_64-unknown-linux-musl --features openssl/vendored
docker compose up --build -d
```