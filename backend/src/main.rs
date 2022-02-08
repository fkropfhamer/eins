use warp::Filter;
use std::collections::HashMap;
use warp::ws::{Message, WebSocket};
use tokio::sync::{mpsc, RwLock};
use futures_util::{SinkExt, StreamExt, TryFutureExt, FutureExt};
use tokio_stream::wrappers::UnboundedReceiverStream;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use serde::{Deserialize, Serialize};

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    username: String,
    text: String,
}

#[tokio::main]
async fn main() {
    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let chat = warp::path("chat").and(warp::ws()).and(users).map(chat_handle);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let status = warp::path("status").map(|| warp::reply::json(&HashMap::from([("status", "ok")])));
    let index = warp::path::end().map(|| "index");

    let echo = warp::path("echo")
        .and(warp::ws())
        .map(echo_handle);
    
    let routes = index.or(status).or(hello).or(echo).or(chat);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn chat_handle(ws: warp::ws::Ws, users: Users) -> impl warp::Reply {
    ws.on_upgrade(move |socket| user_connected(socket, users))
}

async fn user_connected(ws: WebSocket, users: Users) {
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {})
                .await;
        }
    });

    users.write().await.insert(my_id, tx);

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => { break; }
        };

        user_message(my_id, msg, &users).await;
    }

    user_disconnected(my_id, &users).await;
}

async fn user_message(my_id: usize, msg: Message, users: &Users) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let chat_message = ChatMessage {
        username: format!("<User#{}>", my_id),
        text: msg.to_string()
    };

    let new_msg = if let Ok(s) = serde_json::to_string(&chat_message) {
        s
    } else {
        return;
    };

    for (&uid, tx) in users.read().await.iter() {
        if my_id != uid {
            if let Err(_disconnected) = tx.send(Message::text(new_msg.clone())) {}
        }
    }
}

async fn user_disconnected(my_id: usize, users: &Users) {
    eprintln!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    users.write().await.remove(&my_id);
}


fn echo_handle(ws: warp::ws::Ws) -> impl warp::Reply {
    ws.on_upgrade(|websocket| {
        let (tx, rx) = websocket.split();
        rx.forward(tx).map(|_result| {
        })
    })
}
