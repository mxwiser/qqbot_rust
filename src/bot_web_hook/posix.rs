use super::{bot_error, MessageEvent};
pub struct BotPosix;
impl BotPosix{
    pub fn message_create(_msg:MessageEvent)->Result<(),bot_error::Error>{
        //println!("收到数据: {:?}", _msg);
        let _message = ok_or!(ok_or!(_msg.d.clone()).content).trim().to_string();
        let _user_id = ok_or!(ok_or!(ok_or!(_msg.d).author).id);
        println!("请求ID：{} 消息 {}",_user_id,_message);
        Ok(())
    }
    pub fn message_event(_msg:MessageEvent)->Result<(),bot_error::Error>{
        //println!("收到数据: {:?}", _msg);
        let _t = ok_or!(_msg.t.clone());
        let _user_id = ok_or!(_msg.op);
        println!("请求OP：{} 事件 {}",_user_id,_t);
        Ok(())
  }
}