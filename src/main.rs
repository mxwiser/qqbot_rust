mod bot_web_hook;
use crate::bot_web_hook::bot_error;
use crate::bot_web_hook::info;
use bot_web_hook::{BotHook, message::MessageEvent};

fn message_process(_message_event: MessageEvent) -> Result<(), bot_error::Error> {
    let _t =_message_event.t.as_ref().unwrap();
    
    if *_t == "GROUP_AT_MESSAGE_CREATE".to_string()
        || *_t == "C2C_MESSAGE_CREATE".to_string()
    {
       
        let _d = _message_event.d.unwrap();
        let _id = _d.author.unwrap().id.unwrap();
        let _text = _d.content.unwrap();
        info!("收到消息 ID:",_id," 内容: ",_text)
    }

    Ok(())
}



#[tokio::main]
async fn main() {
    BotHook::start(message_process).await;
}
