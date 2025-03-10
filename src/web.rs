use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use actix_web::HttpRequest;
use actix_cors::Cors;
use actix_web::get;
pub  struct  WebListener;
use ed25519_dalek::ed25519::signature::SignerMut;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;


const BOT_SECRET: &str = "DG5g3B4j9X2KOErG";







#[get("/aoe4")]
async fn greet_get(_req: HttpRequest) -> impl Responder {
  
    let private_key:[u8;32] =BOT_SECRET.repeat(5)[..32].as_bytes().try_into().unwrap();
    println!("private:{:?}",private_key);
    let mut  signingkey: SigningKey =SigningKey::from_bytes(&private_key);
    println!("publice_key:{:?}",signingkey.verifying_key().to_bytes());
    let message: &[u8] = b"1725442341Arq0D5A61EgUu4OxUvOp";
    let signature: Signature = signingkey.try_sign(message).unwrap();
    HttpResponse::Ok().body(signature.to_string())
}
#[post("/aoe4")]
async fn greet_post(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}




impl  WebListener{
    pub async fn listen()  {
        let _ = HttpServer::new(|| {
            App::new()
                .wrap(
                    Cors::permissive().
                    supports_credentials() 
                )
                .service(greet_get)
                .service(greet_post)
        })
        .bind("0.0.0.0:7081").unwrap()
        .run().await;
    }
}