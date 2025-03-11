use super::{bot_error, MessageEvent};
pub struct BotPosix;
impl BotPosix{
    pub fn message_create(_msg:MessageEvent)->Result<(),bot_error::Error>{
          let _message = ok_or!(ok_or!(_msg.d.clone()).content);
          let _user_id = ok_or!(ok_or!(ok_or!(_msg.d).author).user_openid);
          println!("请求ID：{} {}",_user_id,_message);
          Ok(())
    }
}