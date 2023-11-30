async fn major() -> &'static str {
    "Hello, world!"
}

async fn bonus() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(major))
        .route("/-1/error", get(bonus))
}

use crate::imports::*;

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn major() {
        let app = TestApp::spawn().await;

        let r = app.get("/").await;

        assert_eq!(r.status(), StatusCode::OK);
        assert_eq!(r.text().await.unwrap(), "Hello, world!");
    }

    #[tokio::test]
    async fn bonus() {
        let app = TestApp::spawn().await;

        let r = app.get("/-1/error").await;

        assert_eq!(r.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    use super::*;
}
