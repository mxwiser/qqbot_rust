mod bot_web_hook;

use bot_web_hook::{message::MessageEvent, BotHook};
#[allow(unused_imports)]
use crate::bot_web_hook::bot_error;



fn message_process(_message_event:MessageEvent)->Result<(),bot_error::Error>{
    println!("{:?}",_message_event);
    Ok(())
}



// impl BotHookEvent for BotHook {
//     fn message_event(_me:MessageEvent)->Result<(),bot_error::Error>{
//         format!("{:?}",_me);
//         Ok(())
//      }
// }

#[tokio::main]
async fn main() {
    BotHook::start(message_process).await;
}
