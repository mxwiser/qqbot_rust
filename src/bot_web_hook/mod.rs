#[macro_use]
mod bot_error;
mod message;
mod posix;
use actix_cors::Cors;
use actix_web::HttpRequest;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use dotenv::from_filename;
use ed25519_dalek::Signature;
use ed25519_dalek::SigningKey;
use ed25519_dalek::ed25519::signature::SignerMut;
use message::MessageEvent;
use std::env;

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
    state: web::Data<Mutex<Vec<String>>>,
) -> Result<HttpResponse, bot_error::Error> {
    let _json: serde_json::Value = serde_json::from_str(_req_body.as_str())?;
    let _msg: message::MessageEvent = serde_json::from_str(_req_body.as_str())?;

    println!("收到数据: {:?}", _msg);
    if let Some(op) = _json.get("op") {
        if op.to_string() == "13" {
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(plain_token_vef(_msg)?));
        } else {
            //去重
            let mut ids = state.lock().unwrap();
            if ids.contains(&ok_or!(_msg.id.clone())) {
                return Ok(HttpResponse::Ok().finish());
            } else {
                if ids.len() > 100 {
                    ids.remove(0);
                }
                ids.push(ok_or!(_msg.id.clone()));
            }
            //

            if ok_or!(_msg.t.clone()) == "AT_MESSAGE_CREATE".to_string()|| ok_or!(_msg.t.clone()) == "C2C_MESSAGE_CREATE".to_string()
            {
                posix::BotPosix::message_create(_msg)?;
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
    state: web::Data<Mutex<Vec<String>>>,
) -> impl Responder {
    match hook(req_body, _req, state) {
        Ok(res) => res,
        Err(e) => {
            println!("Error: {}", e);
            HttpResponse::BadRequest().await.unwrap()
        }
    }
}

use actix_web::web;
use std::sync::Mutex;
pub struct BotHook;
impl BotHook {
    pub async fn start() {
        from_filename("bot.env").ok();
        println!(
            "BOT_SECRET: {:?}",
            mask_string(env::var("BOT_SECRET").expect("BOT_SECRET not found!"))
        );
        println!(
            "BotHook listen on: {:?}",
            env::var("BOT_LISTEN").expect("BOT_LISTEN not found!")
        );
        let ids: Vec<String> = Vec::new();
        let state = web::Data::new(Mutex::new(ids));

        let _ = HttpServer::new(move || {
            App::new()
                .wrap(Cors::permissive().supports_credentials())
                .app_data(state.clone())
                .service(greet)
        })
        .bind(env::var("BOT_LISTEN").unwrap())
        .unwrap()
        .run()
        .await;
    }
}
