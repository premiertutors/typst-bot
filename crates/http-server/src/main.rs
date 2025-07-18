use actix_web::{post, web, App, HttpResponse, HttpServer};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use worker_lib::{render, Sandbox};

#[derive(Deserialize)]
struct Req {
    code: String,
}

#[derive(Serialize)]
struct Resp {
    images: Vec<String>,
    more_pages: usize,
    warnings: String,
}

#[post("/render")]
async fn do_render(body: web::Json<Req>, data: web::Data<Sandbox>) -> HttpResponse {
    let out = web::block(move || render(&data, body.code.clone())).await;
    match out {
        Ok(Ok(o)) => HttpResponse::Ok().json(Resp {
            images: o.images.into_iter().map(|img| BASE64_STANDARD.encode(img)).collect(),
            more_pages: o.more_pages,
            warnings: o.warnings,
        }),
        Ok(Err(e)) => HttpResponse::BadRequest().body(e),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sandbox = Sandbox::new();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sandbox.clone()))
            .service(do_render)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
