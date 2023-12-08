use actix_web::{post, web, HttpResponse};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize, Debug, Clone)]
struct Reindeer {
    name: String,
    strength: u32,
    speed: Option<f32>,
    height: Option<u32>,
    antler_width: Option<u32>,
    snow_magic_power: Option<u32>,
    favorite_food: Option<String>,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies_eaten_yesterday: Option<u32>,
}

struct ContestResult {
    fastest: Reindeer,
    tallest: Reindeer,
    magician: Reindeer,
    consumer: Reindeer,
}

impl Serialize for ContestResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry(
            "fastest",
            format!(
                "Speeding past the finish line with a strength of {} is {}",
                self.fastest.strength, self.fastest.name
            )
            .as_str(),
        )?;
        map.serialize_entry(
            "tallest",
            format!(
                "{} is standing tall with his {} cm wide antlers",
                self.tallest.name,
                self.tallest.antler_width.unwrap()
            )
            .as_str(),
        )?;
        map.serialize_entry(
            "magician",
            format!(
                "{} could blast you away with a snow magic power of {}",
                self.magician.name,
                self.magician.snow_magic_power.unwrap()
            )
            .as_str(),
        )?;
        map.serialize_entry(
            "consumer",
            format!(
                "{} ate lots of candies, but also some {}",
                self.consumer.name,
                self.consumer.favorite_food.clone().unwrap()
            )
            .as_str(),
        )?;
        map.end()
    }
}

#[post("/4/strength")]
pub async fn strength_route(body: web::Json<Vec<Reindeer>>) -> HttpResponse {
    let result = body.iter().fold(0, |acc, r| acc + r.strength);
    HttpResponse::Ok().body(result.to_string())
}

fn get_best_reindeer<'a, F, V>(reindeer_list: &'a Vec<Reindeer>, f: F) -> Reindeer
where
    V: PartialOrd,
    F: Fn(&Reindeer) -> V,
{
    reindeer_list
        .iter()
        .max_by(|a, b| f(a).partial_cmp(&f(b)).unwrap_or(std::cmp::Ordering::Less))
        .unwrap()
        .clone()
}

#[post("/4/contest")]
pub async fn contest_route(body: web::Json<Vec<Reindeer>>) -> HttpResponse {
    let reindeer_list = body.into_inner();

    let fastest = get_best_reindeer(&reindeer_list, |r| r.speed.unwrap());
    let tallest = get_best_reindeer(&reindeer_list, |r| r.height.unwrap());
    let magician = get_best_reindeer(&reindeer_list, |r| r.snow_magic_power.unwrap());
    let consumer = get_best_reindeer(&reindeer_list, |r| r.candies_eaten_yesterday.unwrap());

    HttpResponse::Ok().json(ContestResult {
        fastest,
        tallest,
        magician,
        consumer,
    })
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header, test, App};

    use super::*;

    #[derive(Deserialize, Debug)]
    struct ContestResultMap {
        fastest: String,
        tallest: String,
        magician: String,
        consumer: String,
    }

    #[actix_web::test]
    async fn test_post_strength() {
        let payload = r#"[
            { "name": "Dasher", "strength": 5 },
            { "name": "Dancer", "strength": 6 },
            { "name": "Prancer", "strength": 4 },
            { "name": "Vixen", "strength": 7 }
          ]"#
        .as_bytes();
        let app = test::init_service(App::new().service(strength_route)).await;
        let req = test::TestRequest::post()
            .uri("http://localhost:8000/4/strength")
            .insert_header(header::ContentType::json())
            .set_payload(payload)
            .to_request();
        let resp = test::call_and_read_body(&app, req).await;

        assert_eq!(resp, "22");
    }

    #[actix_web::test]
    async fn test_post_contest() {
        let payload = r#"[
            {
              "name": "Dasher",
              "strength": 5,
              "speed": 50.4,
              "height": 80,
              "antler_width": 36,
              "snow_magic_power": 9001,
              "favorite_food": "hay",
              "cAnD13s_3ATeN-yesT3rdAy": 2
            },
            {
              "name": "Dancer",
              "strength": 6,
              "speed": 48.2,
              "height": 65,
              "antler_width": 37,
              "snow_magic_power": 4004,
              "favorite_food": "grass",
              "cAnD13s_3ATeN-yesT3rdAy": 5
            }
          ]"#
        .as_bytes();
        let app = test::init_service(App::new().service(contest_route)).await;
        let req = test::TestRequest::post()
            .uri("http://localhost:8000/4/contest")
            .insert_header(header::ContentType::json())
            .set_payload(payload)
            .to_request();
        let resp: ContestResultMap = test::call_and_read_body_json(&app, req).await;

        assert_eq!(
            resp.fastest,
            "Speeding past the finish line with a strength of 5 is Dasher"
        );

        assert_eq!(
            resp.tallest,
            "Dasher is standing tall with his 36 cm wide antlers"
        );

        assert_eq!(
            resp.magician,
            "Dasher could blast you away with a snow magic power of 9001"
        );

        assert_eq!(
            resp.consumer,
            "Dancer ate lots of candies, but also some grass"
        );
    }
}
