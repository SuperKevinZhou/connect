use std::{collections::HashMap, sync::{Arc, Mutex}};

use axum::{
    routing::get,
    extract::{Query, State},
    Router
};

#[derive(Clone)]
struct AppState {
    signed_in: Arc<Mutex<bool>>,
    user_data_info: Arc<Mutex<HashMap<String, String>>>,
}

async fn login(Query(params): Query<HashMap<String, String>>, State(state): State<AppState>) -> String {
    if !params.contains_key("user_name") || !params.contains_key("user_uuid") {
        "Error: `user_name` and `user_uuid` not given".to_string()
    } else {
        *state.user_data_info.lock().unwrap().entry("user_name".to_string()).and_modify(|e| *e = params.get(&"user_name".to_string()).unwrap().to_string()).or_insert_with(|| params.get(&"user_name".to_string()).unwrap().to_string()) = params.get(&"user_name".to_string()).unwrap().to_string();
        *state.user_data_info.lock().unwrap().entry("user_uuid".to_string()).and_modify(|e| *e = params.get(&"user_uuid".to_string()).unwrap().to_string()).or_insert_with(|| params.get(&"user_uuid".to_string()).unwrap().to_string()) = params.get(&"user_uuid".to_string()).unwrap().to_string();
        *state.signed_in.lock().unwrap() = true;
        "Login successful".to_string()
    }
}

async fn server(signed_in: Arc<Mutex<bool>>, user_data_info: Arc<Mutex<HashMap<String, String>>>) {
    println!("Client API server started!");
    let app = Router::new()
        .route("/login", get(login))
        .with_state(AppState { signed_in, user_data_info });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn sync_server(signed_in: Arc<Mutex<bool>>, user_data_info: Arc<Mutex<HashMap<String, String>>>) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(server(signed_in, user_data_info));
}