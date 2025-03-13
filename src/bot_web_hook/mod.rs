#[macro_use]
pub mod bot_error;
pub mod message;
pub mod posix;
use actix_cors::Cors;
use actix_web::HttpRequest;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use dotenv::from_filename;
use ed25519_dalek::Signature;
use ed25519_dalek::SigningKey;
use ed25519_dalek::ed25519::signature::SignerMut;
use message::MessageEvent;
use std::{ env, u64};
#[allow(unused_imports)]
pub use tklog::{trace,debug, error, fatal, info,warn};
use tklog::{Format, LEVEL, LOG};
lazy_static::lazy_static! {
    static ref APP_ACCESS_TOKEN: Arc<Mutex<String>> = Arc::new(Mutex::new("aaaa".to_string()));
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

fn hook(
    _req_body: String,
    _req: HttpRequest,
    message_event:web::Data<AppState>
) -> Result<HttpResponse, bot_error::Error> {
    let _json: serde_json::Value = serde_json::from_str(_req_body.as_str())?;
    //info!("Receive: {:?}", _json);
    let _msg: message::MessageEvent = serde_json::from_str(_req_body.as_str())?;
    if let Some(op) = _json.get("op") {
        
        if op.to_string() == "13" {
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(plain_token_vef(_msg)?));
        } else {
            let mut ids = message_event.ids.lock().unwrap();
            if ids.contains(&ok_or!(_msg.id.clone())) {
                return Ok(HttpResponse::Ok().finish());
            } else {
                if ids.len() > 100 {
                    ids.remove(0);
                }
                ids.push(ok_or!(_msg.id.clone()));
            }
                match (message_event.handler)(&_msg) {
                    Ok(_ok) => {}
                    Err(_e) => {
                        info!("message_event error! ", _e)
                    }
                }

        }
    }
    Ok(HttpResponse::Ok().finish())
}

fn mask_string(s: String) -> String {
    if s.len() <= 5 {
        s.to_string()
    } else {
        format!("{}{}", &s[..5], "*".repeat(s.len() - 5))
    }
}

#[actix_web::route("/aoe4", method = "GET", method = "POST")]
async fn greet(
    req_body: String,
    _req: HttpRequest,
    message_event:web::Data<AppState>
) -> impl Responder {
    match hook(req_body, _req,message_event) {
        Ok(res) => res,
        Err(e) => {
            info!("Error: ", e);
            HttpResponse::Ok().finish()
        }
    }
}
#[derive(Clone)]
struct AppState {
    ids:Arc<Mutex<Vec<String>>>,
    handler: Arc<dyn Fn(&MessageEvent) ->Result<(),bot_error::Error> + Send + Sync>,
}

use actix_web::web;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


pub struct BotHook;
async fn renew_app_access_token() {
    let _handle = thread::spawn(|| {
        //APP_ACCESS_TOKEN
        info!("APP_ACCESS_TOKEN thread start！");

        let json_obj = serde_json::json!({
            "appId": env::var("BOT_APPID").unwrap(),
            "clientSecret": env::var("BOT_SECRET").unwrap(),
        });

        //https://bots.qq.com/app/getAppAccessToken
        let client = reqwest::blocking::Client::new();
     
        loop {
            let  _request = client
            .post("https://bots.qq.com/app/getAppAccessToken")
            .json(&json_obj);
            let _response: reqwest::blocking::Response =_request.send().unwrap();
            let body: String = _response.text().unwrap();
            let _json: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();
            if let Some(access_token) = _json.get("access_token") {
                if let Some(expires_in) = _json.get("expires_in") {
                    let mut token = APP_ACCESS_TOKEN.lock().unwrap();
                    *token = access_token.as_str().unwrap().to_string();
                    let time = expires_in.as_str().unwrap().parse::<u64>().unwrap();
                    info!("APP_ACCESS_TOKEN Renew:", token," expires_in: ", time);
                    drop(token);
                    thread::sleep(Duration::from_secs(time));
                }
            }
        }
    });
}






impl BotHook {




    pub async fn start(message_event:fn( &MessageEvent) ->Result<(),bot_error::Error>) {
        LOG.set_console(true)
        .set_level(LEVEL::Info)
        .set_format(Format::LevelFlag|Format::Date|Format::Time);
        //.set_cutmode_by_size("./assets/runtime.log", 10000, 10, true);

        from_filename("bot.env").ok();
        info!(
            "BOT_APPID: ",
            mask_string(env::var("BOT_APPID").expect("BOT_APPID not found!"))
        );
        info!(
            "BOT_SECRET: ",
            mask_string(env::var("BOT_SECRET").expect("BOT_SECRET not found!"))
        );
        info!(
            "BotHook listen on: ",
            env::var("BOT_LISTEN").expect("BOT_LISTEN not found!")
        );

        renew_app_access_token().await;

        

        // let ids: Vec<String> = Vec::new();
        let vids: Vec<String> = Vec::new();
        let nids=Mutex::new(vids);

        let _as = AppState {
            ids: Arc::new(nids),
            handler: Arc::new(message_event),
        };
        let _was =web::Data::new(_as);
        let _ = HttpServer::new(move || {
            App::new()
                .wrap(Cors::permissive().supports_credentials())
                .app_data(_was.clone())
                .service(greet)
        })
        .bind(env::var("BOT_LISTEN").unwrap())
        .unwrap()
        .run()
        .await;
    }
}
