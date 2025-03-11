use serde::Deserialize;
#[derive(Deserialize,Clone, Debug)]
#[allow(dead_code)]
pub struct MessageEvent {
    #[serde(default)]
    pub id: Option<String>,
    pub op: Option<i32>,
    #[serde(default)]
    pub d: Option<Data>,
    #[serde(default)]
    pub t: Option<String>,
}
#[derive(Deserialize,Clone, Debug)]
#[allow(dead_code)]
pub struct Data {
    #[serde(default)]
    pub author: Option<Author>,
    #[serde(default)]
    pub plain_token: Option<String>,
    #[serde(default)]
    pub event_ts: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub group_openid: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Deserialize, Debug,Clone)]
#[allow(dead_code)]
pub struct Author {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub union_openid: Option<String>,
    #[serde(default)]
    pub user_openid: Option<String>,
    #[serde(default)]
    pub group_openid: Option<String>,
    #[serde(default)]
    pub member_openid: Option<String>,
}



use crate::bot_web_hook::APP_ACCESS_TOKEN;
use std::env;
use tokio::task;
use super::bot_error;
pub struct MessageHelper;

impl  MessageHelper{
    pub    fn rot_message(msg:String,me:MessageEvent) -> Result<(),bot_error::Error> {
           let json_obj = serde_json::json!({
              "content":msg,
              "msg_type": 0,
              "msg_id":ok_or!(ok_or!(me.d.clone()).id)
            });
            let   _token = APP_ACCESS_TOKEN.lock().unwrap();
            let token = _token.to_string().clone();
            drop(_token);
            
            task::spawn_blocking(move|| -> Result<(),bot_error::Error> {
                // 执行阻塞操作
                let client = reqwest::blocking::Client::new();
                let _response:reqwest::blocking::Response = 
                client.post(env::var("BOT_API").unwrap()+&"/v2/users/"+&ok_or!(ok_or!(ok_or!(me.d).author).id)+&"/messages".to_string())
                .json(&json_obj).header("Authorization",format!("QQBot {}",token.clone())).
                send().expect("请求失败");
                let body:String=_response.text().unwrap();
                let _json:serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
                println!("{}",token);
                Ok(())
 
            });

            
 
            //let body:String=_response.text().unwrap();
            // let _json:serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
            //println!("{}",body);
 



        Ok(())
    }
}