use poem::{handler, listener::TcpListener, post, Route, Server};

#[handler]
async fn payload(req: String) {
    println!("{:#}", req)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/", post(payload));
    
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}