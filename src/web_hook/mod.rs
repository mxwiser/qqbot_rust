
use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use actix_web::HttpRequest;
use actix_cors::Cors;
use actix_web::get;
pub  struct  WebListener;
use ed25519_dalek::ed25519::signature::SignerMut;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;
use rand_core::le;

use super::bot_error;
use super::bot_error::ThrErr;


const BOT_SECRET: &str = "DG5g3B4j9X2KOErG";


fn plain_token_vef(plain_token:&str,event_ts:&str) -> String {
    let msg =  format!("{}{}", event_ts, plain_token);
    println!("{:?}",msg);
    let private_key:[u8;32] =BOT_SECRET.repeat(5)[..32].as_bytes().try_into().unwrap();
    let mut  signingkey: SigningKey =SigningKey::from_bytes(&private_key);
    let signature: Signature = signingkey.try_sign((msg).as_bytes()).unwrap();
    println!("{:?}",signature);
    println!("{:?}",signature.to_string().to_ascii_lowercase());
    return signature.to_string().to_ascii_lowercase();
}


fn message(_req_body: String,_req: HttpRequest) -> Result<HttpResponse,bot_error::Error> {
    
    
    
    let _json: serde_json::Value = serde_json::from_str(_req_body.as_str())?;

  

    if let Some(op) = _json.get("op") {
        if op.to_string()== "13" {
            println!("{:?}",_json);
            
            let d =  _json.get("d").ok_or(ThrErr::thr_err())?;
            let plain_token =d.get("plain_token").ok_or(ThrErr::thr_err())?.as_str().unwrap();
            let event_ts = d.get("event_ts").ok_or(ThrErr::thr_err())?.as_str().unwrap();
            let sig= plain_token_vef(plain_token, event_ts);
            
            let json_obj = serde_json::json!({
                "d":{
                    "plain_token": plain_token,
                    "signature": sig.as_str(),
                }

            });
            println!("{:?}",d.get("plain_token").ok_or(ThrErr::thr_err())?.as_str());
            return  Ok(HttpResponse::Ok().json(json_obj));
       
        }
        
    }

   

    
    Ok(HttpResponse::Ok().body(""))
    
   
}






async fn greet_get(req_body: String,_req: HttpRequest) -> impl Responder {
    match message(req_body, _req) {
        Ok(res  )=>{
            res
        },
        Err(e)=>{
          println!("Error: {}", e);  
          HttpResponse::BadGateway().await.unwrap()
        }
    }
  
    // println!("body:{:?}",req_body);

    // let private_key:[u8;32] =BOT_SECRET.repeat(5)[..32].as_bytes().try_into().unwrap();
    // println!("private:{:?}",private_key);
    // let mut  signingkey: SigningKey =SigningKey::from_bytes(&private_key);
    // println!("publice_key:{:?}",signingkey.verifying_key().to_bytes());
    // let message: &[u8] = b"1725442341Arq0D5A61EgUu4OxUvOp";
    // let signature: Signature = signingkey.try_sign(message).unwrap();
    // HttpResponse::Ok().body(signature.to_string())

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




impl  WebListener{
    pub async fn listen()  {
        let _ = HttpServer::new(|| {
            App::new()
                .wrap(
                    Cors::permissive().
                    supports_credentials() 
                )
                .service(greet)
        })
        .bind("0.0.0.0:8080").unwrap()
        .run().await;
    }
}