
mod web;
use web::WebListener;
use std::thread;

#[tokio::main]
async fn main() {

    let handle = thread::spawn(async|| {
            println!("Web thread started.");
            let _ = WebListener::listen().await;
    });

    handle.join().unwrap().await;
    println!("Hello, world!");
}

