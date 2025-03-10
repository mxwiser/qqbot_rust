use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::HttpRequest;
use actix_cors::Cors;






pub  struct  WebListener;






impl  WebListener{
    async fn greet(_req: HttpRequest) -> impl Responder {

        HttpResponse::Ok()
            .content_type("application/json")
            .body(r#"{"message": "Hello, World!"}"#)
    }
    pub async fn listen()  {
        let _ = HttpServer::new(|| {
            App::new()
                .wrap(
                    Cors::permissive().
                    supports_credentials() 
                )
                .route("/aoe4/{path:.*}", web::get().to(WebListener::greet))
        })
        .bind("0.0.0.0:7081").unwrap()
        .run().await;

    }
    
}