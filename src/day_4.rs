use actix_web::{post, web, HttpResponse};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize, Debug, Clone)]
struct Reindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies_eaten_yesterday: u32,
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
                self.tallest.name, self.tallest.antler_width
            )
            .as_str(),
        )?;
        map.serialize_entry(
            "magician",
            format!(
                "{} could blast you away with a snow magic power of {}",
                self.magician.name, self.magician.snow_magic_power
            )
            .as_str(),
        )?;
        map.serialize_entry(
            "consumer",
            format!(
                "{} ate lots of candies, but also some {}",
                self.consumer.name, self.consumer.favorite_food
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

    let fastest = get_best_reindeer(&reindeer_list, |r| r.speed);
    let tallest = get_best_reindeer(&reindeer_list, |r| r.height);
    let magician = get_best_reindeer(&reindeer_list, |r| r.snow_magic_power);
    let consumer = get_best_reindeer(&reindeer_list, |r| r.candies_eaten_yesterday);

    HttpResponse::Ok().json(ContestResult {
        fastest,
        tallest,
        magician,
        consumer,
    })
}
