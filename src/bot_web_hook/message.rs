use serde::Deserialize;
#[derive(Deserialize, Clone, Debug)]
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
#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct Data {
    #[serde(default)]
    pub group_openid: Option<String>,
    #[serde(default)]
    pub author: Option<Author>,
    #[serde(default)]
    pub plain_token: Option<String>,
    #[serde(default)]
    pub event_ts: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Author {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub union_openid: Option<String>,
    #[serde(default)]
    pub user_openid: Option<String>,
    #[serde(default)]
    pub member_openid: Option<String>,
}

use super::bot_error;
use crate::bot_web_hook::APP_ACCESS_TOKEN;
use std::env;
use tokio::task;


pub struct MessageHelper;
#[allow(unused)]
use actix_web::rt::task::JoinHandle;
impl MessageHelper {
    pub fn rot_message(msg: &String, _me:& MessageEvent) -> JoinHandle<Result<(), bot_error::Error>> {
        let me = _me.clone();
        let msg_id = me.d.as_ref().unwrap().id.as_ref().unwrap();
        let json_obj = serde_json::json!({
          "content":msg,
          "msg_type": 0,
          "msg_id": msg_id
        });
        let _token = APP_ACCESS_TOKEN.lock().unwrap();
        let token = _token.to_string().clone();
        drop(_token);

        let _t=  task::spawn_blocking( move|| -> Result<(), bot_error::Error> {
            let client = reqwest::blocking::Client::new();
            let mut api_url: String = env::var("BOT_API").unwrap();
            if ok_or!(me.t.clone()) == "GROUP_AT_MESSAGE_CREATE".to_string() {
                api_url = api_url
                    + &"/v2/groups/"
                    + &ok_or!(ok_or!(me.d.clone()).group_openid)
                    + &"/messages".to_string();
            }
            if ok_or!(me.t.clone()) == "C2C_MESSAGE_CREATE".to_string() {
                api_url = api_url
                    + &"/v2/users/"
                    + &ok_or!(ok_or!(ok_or!(me.d.clone()).author).id)
                    + &"/messages".to_string();
            }
            let _response: reqwest::blocking::Response = client
                .post(api_url)
                .json(&json_obj)
                .header("Authorization", format!("QQBot {}", token))
                .send()
                .unwrap();
            Ok(())
        });

       return  _t
    }
}
