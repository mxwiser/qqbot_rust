



// use crate::bot_web_hook::message;

// use super::{bot_error, MessageEvent};
// use super::info;
// pub struct BotPosix;
// impl BotPosix{
//     //接收到信息这个函数会被执行
//     pub fn message_create(_me:MessageEvent)->Result<(),bot_error::Error>{
//         //info!("收到数据:", _msg);
//         let _message = ok_or!(ok_or!(_me.d.clone()).content).trim().to_string();
//         let _user_id = ok_or!(ok_or!(ok_or!(_me.d.clone()).author).id);
//         info!("请求ID：",_user_id," 内容 ",_message);
//         message::MessageHelper::rot_message(_message, _me)?;
//         Ok(())
//     }
//     //接收到事件这个函数会被执行
//     pub fn message_event(_msg:MessageEvent)->Result<(),bot_error::Error>{
//         //info!("收到数据:", _msg);
//         let _t = ok_or!(_msg.t.clone());
//         let _user_id = ok_or!(_msg.op);
//         info!("请求OP：",_user_id," 事件 ",_t);
//         Ok(())
//   }
// }