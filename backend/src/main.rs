use futures_util::{FutureExt, StreamExt};
use warp::Filter;
use std::collections::HashMap;


#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let status = warp::path("status").map(|| warp::reply::json(&HashMap::from([("status", "ok")])));
    let index = warp::path::end().map(|| "index");

    let echo = warp::path("echo")
        .and(warp::ws())
        .map(echo_re);
    
    let routes = index.or(status).or(hello).or(echo);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


fn echo_re(ws: warp::ws::Ws) -> impl warp::Reply {
    ws.on_upgrade(|websocket| {
        let (tx, rx) = websocket.split();
        rx.forward(tx).map(|_result| {
        })
    })
}
