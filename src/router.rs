pub fn router() -> Router {
    let router = Router::new()

        .nest("", tasks::minus_one::router())
        .nest("", tasks::one::router())

        /* -_- */
    ;

    router
}

use crate::tasks;
use axum::Router;
