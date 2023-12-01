async fn major() -> &'static str {
    "Hello, world!"
}

async fn bonus() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

static MAJOR_URL: &str = "/";
static BONUS_URL: &str = "/-1/error";

pub fn router() -> Router {
    Router::new()
        .route(MAJOR_URL, get(major))
        .route(BONUS_URL, get(bonus))
}

use crate::imports::*;

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn major() {
        let app = TestApp::spawn().await;

        let r = app.get(MAJOR_URL).await;

        assert_eq!(r.status(), StatusCode::OK);
        assert_eq!(r.text().await.unwrap(), "Hello, world!");
    }

    #[tokio::test]
    async fn bonus() {
        let app = TestApp::spawn().await;

        let r = app.get(BONUS_URL).await;

        assert_eq!(r.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    use super::*;
}
