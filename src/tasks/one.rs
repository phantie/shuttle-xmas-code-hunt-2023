async fn both(Path(nums): Path<String>) -> impl IntoResponse {
    nums.split("/")
        .map(|v| v.parse::<i32>())
        .map(Result::unwrap)
        .fold(0, |acc, x| acc ^ x)
        .pow(3)
        .to_string()
}

static URL: &str = "/1/*nums";

pub fn router() -> Router {
    Router::new().route(URL, get(both))
}

use crate::imports::*;

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn both() {
        let app = TestApp::spawn().await;

        for (path, expect) in [("/4/8", 1728), ("/10", 1000), ("/4/5/8/10", 27)] {
            let r = app.get(&format!("/1{path}")).await;
            assert_eq!(r.status(), StatusCode::OK);
            assert_eq!(r.text().await.unwrap().parse::<i32>().unwrap(), expect);
        }
    }

    use super::*;
}
