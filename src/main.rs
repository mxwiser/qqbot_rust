

#[macro_use]
mod bot_error;
mod web_hook;
use web_hook::WebListener;
use std::thread;
use dotenv::dotenv;
use std::env;

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

