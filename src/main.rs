use axum::{
    routing::get,
    Json, Router, Server,
};
use serde::Serialize;
use std::{net::UdpSocket, sync::Mutex};
use tokio::task;
use lazy_static::lazy_static;

#[derive(Serialize, Clone)]
struct LogEntry {
    source: String,
    message: String,
}

lazy_static! {
    static ref LOGS: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());
}

async fn get_logs() -> Json<Vec<LogEntry>> {
    let logs = LOGS.lock().unwrap().clone();
    Json(logs)
}

fn start_syslog_listener() {
    task::spawn_blocking(|| {
        let socket = UdpSocket::bind("0.0.0.0:514").expect("Failed to bind UDP port 514");
        let mut buf = [0u8; 4096];

        loop {
            match socket.recv_from(&mut buf) {
                Ok((size, src)) => {
                    let message = String::from_utf8_lossy(&buf[..size]).to_string();
                    let entry = LogEntry {
                        source: src.to_string(),
                        message,
                    };
                    LOGS.lock().unwrap().push(entry);
                }
                Err(e) => eprintln!("Erreur de réception : {e}"),
            }
        }
    });
}

#[tokio::main]
async fn main() {
    start_syslog_listener();

    let app = Router::new().route("/logs", get(get_logs));

    println!("API en écoute sur http://0.0.0.0:3000");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
