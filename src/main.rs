mod bot_web_hook;
use bot_web_hook::BotHook;
use macros::bot_event;


#[bot_event]
async fn process(_message_event: bot_web_hook::message::MessageEvent) {
    let _t =_message_event.t.as_ref().unwrap();
    if _t.as_ref() == "GROUP_AT_MESSAGE_CREATE".to_string()
        || _t.as_ref() == "C2C_MESSAGE_CREATE".to_string()
    {
        let _d = _message_event.d.as_ref().unwrap();
        let _id = _d.author.as_ref().unwrap().id.as_ref().unwrap();
        let _text = _d.content.as_ref().unwrap();
        bot_web_hook::info!("收到消息 ID:",_id," 内容: ",_text);
        bot_web_hook::message:: MessageHelper::rot_message(&_text, &_message_event).await.unwrap().unwrap();  
    }
}

#[actix_web::main]
async fn main() {

     BotHook::start(process).await;
}
