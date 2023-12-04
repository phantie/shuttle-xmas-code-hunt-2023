#[derive(serde::Serialize, serde::Deserialize)]
struct Params {
    strength: i32,
    #[allow(unused)]
    name: String,
}

async fn major(Json(params): Json<Vec<Params>>) -> impl IntoResponse {
    params
        .iter()
        .map(|param| param.strength)
        .sum::<i32>()
        .to_string()
}

#[derive(serde::Deserialize)]
struct MoreParams {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies_eaten: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BonusResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn bonus(Json(params): Json<Vec<MoreParams>>) -> impl IntoResponse {
    let fastest = params
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .unwrap();

    let tallest = params.iter().max_by_key(|param| param.height).unwrap();

    let magician = params
        .iter()
        .max_by_key(|param| param.snow_magic_power)
        .unwrap();

    let consumer = params
        .iter()
        .max_by_key(|param| param.candies_eaten)
        .unwrap();

    Json(BonusResult {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}

static MAJOR_URL: &str = "/4/strength";
static BONUS_URL: &str = "/4/contest";

pub fn router() -> Router {
    Router::new()
        .route(MAJOR_URL, post(major))
        .route(BONUS_URL, post(bonus))
}

use crate::imports::*;

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn major() {
        let app = TestApp::spawn().await;

        let data = [Params {
            strength: 5,
            name: "".into(),
        }];

        let r = app.post(MAJOR_URL, &data).await;
        assert_eq!(r.status(), StatusCode::OK);

        assert_eq!(r.text().await.unwrap().parse::<u32>().unwrap(), 5);
    }

    #[tokio::test]
    async fn bonus() {
        let app = TestApp::spawn().await;

        let data = serde_json::json!([{
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
        }]);

        let r = app.post(BONUS_URL, &data).await;

        assert_eq!(r.status(), StatusCode::OK);
        let json = r.json::<BonusResult>().await.unwrap();

        assert!(json.fastest.ends_with("Dasher"));
        assert!(json.consumer.starts_with("Dancer"));
        assert!(json.tallest.starts_with("Dasher"));
        assert!(json.magician.starts_with("Dasher"));
    }

    use super::*;
}
