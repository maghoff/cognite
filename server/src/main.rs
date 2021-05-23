use serde_derive::{Deserialize, Serialize};
use slog::{info, o};
use slog::{Drain, Logger};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use warp::{
    http::{self, HeaderValue},
    hyper::header::CONTENT_TYPE,
    Filter, Rejection, Reply,
};

#[derive(Serialize, Deserialize)]
struct Message {
    id: u32,
    text: String,
}

type Messages = Vec<Message>;

#[derive(Clone)]
struct Store {
    messages: Arc<Mutex<Messages>>,
}

impl Store {
    fn new() -> Self {
        Store {
            messages: Arc::new(Mutex::new(vec![
                Message {
                    id: 0,
                    text: "Hello, Joe!".to_string(),
                },
                Message {
                    id: 1,
                    text: "Hello, Mike!".to_string(),
                },
            ])),
        }
    }
}

async fn handle_messages(store: Store) -> Result<impl Reply, Rejection> {
    let messages = store.messages.lock().unwrap();

    let mut res = warp::reply::Response::new(serde_json::to_string(&*messages).unwrap().into());

    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    Ok(res)
}

// Heavy inspiration from
// https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
fn json_body() -> impl Filter<Extract = (Message,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn add_message(message: Message, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.messages.lock().unwrap().push(message);

    Ok(warp::reply::with_status(
        "Added message",
        http::StatusCode::CREATED,
    ))
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

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let messages = warp::path("messages")
        .and(store_filter.clone())
        .and_then(handle_messages)
        .with(log_filter);

    let add_message = warp::post()
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(add_message);

    let routes = messages.or(add_message);

    let addr: SocketAddr = "0.0.0.0:5555".parse().unwrap();
    let (addr, serve) = warp::serve(routes).bind_ephemeral(addr);

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
