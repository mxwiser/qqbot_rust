mod bot_web_hook;



use bot_web_hook::{BotHook, message::MessageEvent,info,bot_error,message,bot_trait};



impl bot_trait::BotTrait for BotHook {
    async  fn message_process(_message_event: &MessageEvent) -> Result<(), bot_error::Error> {
        let _t =_message_event.t.as_ref().unwrap();
        if _t.as_ref() == "GROUP_AT_MESSAGE_CREATE".to_string()
            || _t.as_ref() == "C2C_MESSAGE_CREATE".to_string()
        {
            let _d = _message_event.d.as_ref().unwrap();
            let _id = _d.author.as_ref().unwrap().id.as_ref().unwrap();
            let _text = _d.content.as_ref().unwrap();
            info!("收到消息 ID:",_id," 内容: ",_text);
            message::MessageHelper::rot_message(&_text, _message_event);

        }
        Ok(())
    }
}


#[tokio::main]
async fn main() {
     BotHook::start().await;
}
