use actix_web::{
    get,
    web::{self},
    HttpResponse,
};

#[get("/1/{tail:.*}")]
pub async fn cube_the_bits(path: web::Path<String>) -> HttpResponse {
    let xor_numbers: i32 = path
        .split('/')
        .map(|s| s.parse::<i32>().unwrap())
        .reduce(|a, b| a ^ b)
        .unwrap();

    HttpResponse::Ok().body(xor_numbers.pow(3).to_string())
}
