impl Hander {
    pub fn my_func() -> JoinHandle<Result<(),bot_error::Error>>{
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

        let _t=  task::spawn_blocking( move|| ->Result<(),bot_error::Error> {
            let client = reqwest::blocking::Client::new();
            let mut api_url: String = env::var("BOT_API").unwrap();
            let _ok =||->Result<(),bot_error::Error>{
            
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
            };
            return _ok();
        });
       return  _t
    }
}