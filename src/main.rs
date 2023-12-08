use actix_web::{get, web::ServiceConfig, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;

mod day_1;
mod day_4;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/-1/error")]
async fn do_error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(do_error);
        cfg.service(day_1::cube_the_bits);
        cfg.service(day_4::strength_route);
        cfg.service(day_4::contest_route);
    };

    Ok(config.into())
}
