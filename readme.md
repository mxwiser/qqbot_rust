## 一个基于Rust的QQ官方机器人基本框架
- [✓] 支持web_hook验证
- [✓] 支持消息去重
- [✓] 支持群聊、私聊消息接收
- [✓] 支持群聊、私聊文本消息回复
- [] 支持图文消息发送
- [] 未完待续...
> 在posix.rs中对接Bot消息处理函数即可。使用前请将bot.env.example重命名为bot.env，并配置 bot.env 中的信息 ，同时确保运行环境能够找到bot.env文件. 


```bash
docker config
rustup target add x86_64-unknown-linux-musl 
apt-get install musl-tools 
cargo build --release --target x86_64-unknown-linux-musl --features openssl/vendored
dcoker compose up --build -d
```