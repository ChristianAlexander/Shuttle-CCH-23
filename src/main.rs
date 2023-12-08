use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse,
};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/-1/error")]
async fn do_error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}

#[get("/1/{tail:.*}")]
async fn cube_the_bits(path: web::Path<String>) -> HttpResponse {
    let xor_numbers: i32 = path
        .split('/')
        .map(|s| s.parse::<i32>().unwrap())
        .reduce(|a, b| a ^ b)
        .unwrap();

    HttpResponse::Ok().body(xor_numbers.pow(3).to_string())
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(do_error);
        cfg.service(cube_the_bits);
    };

    Ok(config.into())
}
