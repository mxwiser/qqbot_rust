


mod bot_web_hook;
use bot_web_hook::BotHook;
use std::thread;



#[tokio::main]
async fn main() {
    
    let handle = thread::spawn(async|| {
            println!("BotHook thread started.");
            let _ = BotHook::start().await;
    });

    handle.join().unwrap().await;
    println!("Hello, world!");
}

