
#[macro_use]
mod bot_error;
use dotenv::from_filename;
use std::env;
use actix_web::{ App, HttpResponse, HttpServer, Responder};
use actix_web::HttpRequest;
use actix_cors::Cors;


use ed25519_dalek::ed25519::signature::SignerMut;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;





fn plain_token_vef(plain_token:&str,event_ts:&str) -> String {
    let msg =  format!("{}{}", event_ts, plain_token);
    let private_key:[u8;32] =env::var("BOT_SECRET").unwrap()[..32].as_bytes().try_into().unwrap();
    let mut  signingkey: SigningKey =SigningKey::from_bytes(&private_key);
    let signature: Signature = signingkey.try_sign((msg).as_bytes()).unwrap();
    return signature.to_string().to_ascii_lowercase();
}


fn message(_req_body: String,_req: HttpRequest) -> Result<HttpResponse,bot_error::Error> {
      
    let _json: serde_json::Value = serde_json::from_str(_req_body.as_str())?;
    println!("收到数据: {:?}",_json);
    if let Some(op) = _json.get("op") {
        if op.to_string()== "13" {
            let d =  json_ok_or!(_json,"d");
            let plain_token =json_ok_or!(d,"plain_token").as_str().unwrap();
            let event_ts = json_ok_or!(d,"event_ts").as_str().unwrap();
            let sig= plain_token_vef(plain_token, event_ts);
            let json_obj = serde_json::json!({
                    "plain_token": plain_token,
                    "signature": sig.as_str(),
            });
            return  Ok(HttpResponse::Ok().content_type("application/json").json(json_obj));
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
async fn greet(req_body: String,_req: HttpRequest) -> impl Responder {
    match message(req_body, _req) {
        Ok(res  )=>res,
        Err(e)=>{println!("Error: {}", e);
        HttpResponse::BadRequest().await.unwrap()
        }
    }
    
}


pub  struct  BotHook;
impl  BotHook{
    pub async fn start()  {
        from_filename("bot.env").ok();
        println!("BOT_SECRET: {:?}",mask_string(env::var("BOT_SECRET").expect("BOT_SECRET not found!")));
        println!("BotHook listen on: {:?}",env::var("BOT_LISTEN").expect("BOT_LISTEN not found!"));
        let _ = HttpServer::new(|| {
            App::new()
                .wrap(
                    Cors::permissive().
                    supports_credentials() 
                )
                .service(greet)
        })
        .bind(env::var("BOT_LISTEN").unwrap()).unwrap()
        .run().await;
    }
}