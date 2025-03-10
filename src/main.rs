


mod bot_web_hook;
use bot_web_hook::WebListener;
use std::thread;
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let handle = thread::spawn(async|| {
            println!("Web thread started.");
            let _ = WebListener::listen().await;
    });

    handle.join().unwrap().await;
    println!("Hello, world!");
}

