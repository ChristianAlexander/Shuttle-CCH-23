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

#[cfg(test)]
mod tests {
    use actix_web::{body, http::header::ContentType, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_cube_the_bits_part_one() {
        let app = test::init_service(App::new().service(cube_the_bits)).await;
        let req = test::TestRequest::default()
            .uri("http://localhost:8000/1/4/8")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;

        assert_eq!(bytes.unwrap(), "1728");
    }

    #[actix_web::test]
    async fn test_cube_the_bits_part_two() {
        let app = test::init_service(App::new().service(cube_the_bits)).await;
        let req = test::TestRequest::default()
            .uri("http://localhost:8000/1/4/5/8/10")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;

        assert_eq!(bytes.unwrap(), "27");
    }
}
