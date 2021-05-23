use serde_derive::Serialize;
use slog::{info, o};
use slog::{Drain, Logger};
use std::net::SocketAddr;
use warp::{http::HeaderValue, hyper::header::CONTENT_TYPE, Filter, Rejection, Reply};

#[derive(Serialize)]
struct Message {
    id: u32,
    text: String,
}

async fn handle_messages() -> Result<impl Reply, Rejection> {
    let messages = vec![
        Message {
            id: 0,
            text: "Hello, Joe!".to_string(),
        },
        Message {
            id: 1,
            text: "Hello, Mike!".to_string(),
        },
    ];

    let mut res = warp::reply::Response::new(serde_json::to_string(&messages).unwrap().into());
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(res)
}

async fn core_main() -> Result<(), Box<dyn std::error::Error>> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!());

    let log2 = log.clone();
    let log_filter = warp::log::custom(move |info| {
        info!(
            log2,
            "Replied {}",
            info.status();
            "query" => format!("{} {}", info.method(), info.path()),
            "remote_addr" => info.remote_addr()
        );
    });

    let messages = warp::path("messages")
        .and_then(handle_messages)
        .with(log_filter);

    let addr: SocketAddr = "0.0.0.0:5555".parse().unwrap();
    let (addr, serve) = warp::serve(messages).bind_ephemeral(addr);

    info!(log, "Listening on http://{}", addr);
    serve.await;

    Ok(())
}

#[tokio::main]
async fn main() {
    match core_main().await {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
