#[macro_use]
pub mod bot_error;
pub mod message;
use actix_cors::Cors;
#[allow(unused_imports)]
use actix_files::Files;
use actix_web::HttpRequest;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use ed25519_dalek::Signature;
use ed25519_dalek::SigningKey;
use ed25519_dalek::ed25519::signature::SignerMut;
use message::MessageEvent;
use tokio::spawn;
use tokio::task::spawn_blocking;
use tokio::time::sleep;
use std::{ env, u64};



lazy_static::lazy_static! {
    pub static ref APP_ACCESS_TOKEN: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
}

fn plain_token_vef(_msg: MessageEvent) -> Result<serde_json::Value, bot_error::Error> {
    let plain_token = ok_or!(ok_or!(_msg.d.clone()).plain_token);
    let event_ts = ok_or!(ok_or!(_msg.d).event_ts);
    let dec = format!("{}{}", event_ts, plain_token);
    let private_key: [u8; 32] = env::var("BOT_SECRET").unwrap()[..32]
        .as_bytes()
        .try_into()
        .unwrap();
    let mut signingkey: SigningKey = SigningKey::from_bytes(&private_key);
    let signature: Signature = signingkey.try_sign((dec).as_bytes()).unwrap();
    let json_obj = serde_json::json!({
            "plain_token": plain_token,
            "signature": signature.to_string().to_ascii_lowercase(),
    });
    Ok(json_obj)
}


async  fn hook(
    _req_body: String,
    _req: HttpRequest,
    message_event:web::Data<AppState>
) -> Result<HttpResponse, bot_error::Error> {
    let _msg: message::MessageEvent = serde_json::from_str(_req_body.as_str())?;
    if let Some(op) = _msg.op.as_ref() {
        
        if op.to_string() == "13" {
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(plain_token_vef(_msg)?));
        } else {
            {
                let mut ids = message_event.ids.lock().unwrap();
                if ids.contains(&ok_or!(_msg.id.clone())) {
                    return Ok(HttpResponse::Ok().finish());
                } else {
                    if ids.len() > 100 {
                        ids.remove(0);
                    }
                    ids.push(ok_or!(_msg.id.clone()));
                }
            }
 
                (message_event.event)(_msg);
        
        }
    }
    Ok(HttpResponse::Ok().finish())
}



#[actix_web::route("/", method = "GET", method = "POST")]
async fn greet(
    req_body: String,
    _req: HttpRequest,
    message_event:web::Data<AppState>
) -> impl Responder {
    match hook(req_body, _req,message_event).await {
        Ok(res) => res,
        Err(_e) => {
            HttpResponse::Ok().finish()
        }
    }
}

use actix_web::web;
use std::sync::{Arc, Mutex};
use std::time::Duration;


type MessageHandler = Arc<fn (message::MessageEvent)>;
#[derive(Clone)]
struct AppState {
    ids:Arc<Mutex<Vec<String>>>,
    event:MessageHandler
}
pub  use actix_web::dev::Server;
#[allow(dead_code)]
fn renew_event(_key:String,_time:u64) {}
#[allow(dead_code)]
pub struct BotHook{app_state:AppState,renew_event:Arc<fn (key:String,time:u64)>}
impl BotHook {

    fn renew_app_access_token(&self) {
        let renew_event = self.renew_event.clone(); 
        spawn( async move {
            loop {
            let event_clone = renew_event.clone();
            let  time=  spawn_blocking(move || {
                    
                    let json_obj = serde_json::json!({
                        "appId": env::var("BOT_APPID").unwrap(),
                        "clientSecret": env::var("BOT_SECRET").unwrap(),
                    });        
                    let client = reqwest::blocking::Client::new();
                    let  _request = client
                    .post("https://bots.qq.com/app/getAppAccessToken")
                    .json(&json_obj);
                    let _response: reqwest::blocking::Response =_request.send().unwrap();
                    let body: String = _response.text().unwrap();
                    let _json: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
                    let mut  time =0;
                    if let Some(access_token) = _json.get("access_token") {
                        if let Some(expires_in) = _json.get("expires_in") {
                            let mut token = APP_ACCESS_TOKEN.lock().unwrap();
                            *token = access_token.as_str().unwrap().to_string();
                             time = expires_in.as_str().unwrap().parse::<u64>().unwrap();
                             (event_clone)(token.to_string(),time);
                            //println!("APP_ACCESS_TOKEN Renew: {} expires_in: {}", token,time);
                            drop(token);
                        }
                    }
                    return time;
                }).await.unwrap();
                sleep(Duration::from_secs(time)).await;
            }
        });
    }
    #[allow(dead_code)]
    pub fn new(handler:fn ( message::MessageEvent)) -> BotHook {
        let vids: Vec<String> = Vec::new();
        let nids=Mutex::new(vids);
        let _as = AppState {
            ids: Arc::new(nids),
            event:Arc::new(handler)
        };
        let renew_event = Arc::new(renew_event as fn(String, u64));
        return BotHook { app_state: _as, renew_event };
    }
    #[allow(dead_code)]
    pub fn new_with_renew_event(handler:fn ( message::MessageEvent),event:fn (key:String,time:u64)) -> BotHook{
        let vids: Vec<String> = Vec::new();
        let nids=Mutex::new(vids);
        let _as = AppState {
            ids: Arc::new(nids),
            event:Arc::new(handler)
        };
        let renew_event=Arc::new(event);
        return BotHook{app_state:_as,renew_event};
    }

    #[allow(dead_code)]
    pub fn start(&self) ->Server {
        env::var("BOT_APPID").expect("BOT_APPID not found!");
        env::var("BOT_SECRET").expect("BOT_SECRET not found!");
        env::var("BOT_LISTEN").expect("BOT_LISTEN not found!");
        self.renew_app_access_token();
        let _as = self.app_state.clone();
        let _was =web::Data::new(_as);
        let _app = HttpServer::new(move || {
            App::new()
                .wrap(Cors::permissive().supports_credentials())
                .app_data(_was.clone())
                .service(greet)
                .service(Files::new("/assets", "./assets/"))
        })
        .bind(env::var("BOT_LISTEN").unwrap())
        .unwrap()
        .run();
      return _app;
    }
}
