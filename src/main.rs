mod bot_web_hook;
use bot_web_hook::BotHook;
use macros::bot_event;
use dotenv::from_filename;

#[bot_event]
async fn process(_message_event: bot_web_hook::message::MessageEvent) {
    let _t =_message_event.t.as_ref().unwrap();
    if _t.as_ref() == "GROUP_AT_MESSAGE_CREATE".to_string()
        || _t.as_ref() == "C2C_MESSAGE_CREATE".to_string()
    {
        let _d = _message_event.d.as_ref().unwrap();
        let _id = _d.author.as_ref().unwrap().id.as_ref().unwrap();
        let _text = _d.content.as_ref().unwrap();
        println!("收到消息 ID:{} 内容:{}",_id,_text);
        bot_web_hook::message:: MessageHelper::rot_message(&_text, &_message_event).await.unwrap().unwrap();  
    }
}
#[bot_event]
async  fn renew(_key:String,time:u64) {
       println!("renew access_token: {}  expires_in: {}",_key,time);
}

#[tokio::main]
async fn main() {
    from_filename("bot.env").ok();
    let _bot= BotHook::new_with_renew_event(process, renew);
    let _server = _bot.start();
    tokio::try_join!(_server).unwrap();
}
