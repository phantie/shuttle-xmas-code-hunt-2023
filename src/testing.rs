pub struct TestApp {
    pub address: String,
    app_handle: tokio::task::JoinHandle<hyper::Result<()>>,
}

impl TestApp {
    pub async fn spawn() -> Self {
        let port = 0; // any vacant port
        let host = "localhost";
        let address = format!("{host}:{port}");
        let listener = std::net::TcpListener::bind(&address).expect("vacant port");

        let port = listener.local_addr().unwrap().port();
        let address = format!("http://{host}:{port}");
        let app_handle = tokio::spawn(
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(crate::router::router().into_make_service()),
        );

        Self {
            app_handle,
            address,
        }
    }

    pub async fn get(&self, path: &str) -> reqwest::Response {
        reqwest::get(format!("{}{path}", self.address))
            .await
            .expect("app to live")
    }

    pub async fn post<D: serde::Serialize>(&self, path: &str, data: &D) -> reqwest::Response {
        let client = reqwest::Client::new();
        client
            .post(format!("{}{path}", self.address))
            .json(data)
            .send()
            .await
            .expect("app to live")
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.app_handle.abort();
    }
}
