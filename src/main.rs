mod bot_web_hook;

use bot_web_hook::BotHook;



#[tokio::main]
async fn main() {

    let _ = BotHook::start().await;
}
