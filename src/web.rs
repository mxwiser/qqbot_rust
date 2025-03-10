use actix_web::{App, HttpServer, Responder, HttpResponse};
use actix_web::HttpRequest;
use actix_cors::Cors;
use actix_web::get;

pub  struct  WebListener;


#[get("/aoe4")]
async fn greet(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"message": "Hello, World!"}"#)
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
        .bind("0.0.0.0:7081").unwrap()
        .run().await;
    }
}