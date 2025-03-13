mod bot_web_hook;

use std::time::Duration;

use crate::bot_web_hook::bot_error;
use crate::bot_web_hook::info;
use bot_web_hook::message::MessageHelper;
use bot_web_hook::{BotHook, message::MessageEvent};



use bot_web_hook::bot_trait::BotTrait;
impl BotTrait for BotHook {
    async  fn message_process(_message_event: &MessageEvent) -> Result<(), bot_error::Error> {
        let _t =_message_event.t.as_ref().unwrap();
        if _t.as_ref() == "GROUP_AT_MESSAGE_CREATE".to_string()
            || _t.as_ref() == "C2C_MESSAGE_CREATE".to_string()
        {
            let _d = _message_event.d.as_ref().unwrap();
            let _id = _d.author.as_ref().unwrap().id.as_ref().unwrap();
            let _text = _d.content.as_ref().unwrap();
            info!("收到消息 ID:",_id," 内容: ",_text);
            MessageHelper::rot_message(&_text, _message_event).await?;
        }
        Ok(())
    }
}


#[tokio::main]
async fn main() {

     BotHook::start().await;
}
