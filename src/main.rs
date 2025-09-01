mod compute_sha;
mod peer_discovery;
mod peer_server;
mod client_api;

use std::collections::HashMap;
use std::{env, thread};
use std::process::Command;
use std::path::{MAIN_SEPARATOR, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::sleep;

use anyhow::Context;

#[cfg(target_os = "windows")]
const EXE_SUFFIX: &str = ".exe";

#[cfg(not(target_os = "windows"))]
const EXE_SUFFIX: &str = "";

fn main() -> anyhow::Result<()> {
    let easytier_path = env::var("EASYTIER_PATH").unwrap_or(format!(".{}", MAIN_SEPARATOR));
    let network_name = env::var("CONNECT_CHAT_NETWORK_NAME").unwrap_or("ConnectChat_CENTER".to_string());
    let network_secret = env::var("CONNECT_CHAT_NETWORK_SECRET").unwrap_or(compute_sha::compute_sha512(&network_name));

    let mut easytier_core_path = PathBuf::from(easytier_path);
    easytier_core_path.push(format!("easytier-core{}", EXE_SUFFIX));
    println!("easytier-core path: {}", easytier_core_path.to_string_lossy());


    let mut easytier_daemon = Command::new(easytier_core_path)
        .arg("-d")
        .arg("--network-name")
        .arg(network_name)
        .arg("--network-secret")
        .arg(network_secret)
        .arg("-p")
        .arg("tcp://public.easytier.top:11010")
        .arg("-p")
        .arg("tcp://turn.hb.629957.xyz:11010")
        .arg("-p")
        .arg("tcp://8.138.6.53:11010")
        .arg("-p")
        .arg("tcp://turn.js.629957.xyz:11012")
        .arg("-p")
        .arg("tcp://et.sh.suhoan.cn:11010")
        .arg("-p")
        .arg("tcp://103.194.107.246:11010")
        .arg("-p")
        .arg("tcp://turn.bj.629957.xyz:11010")
        .arg("--enable-kcp-proxy")
        .arg("--compression")
        .arg("zstd")
        .arg("--encryption-algorithm")
        .arg("chacha20")
        .arg("--multi-thread")
        .arg("--multi-thread-count")
        .arg("6")
        .spawn()
        .context("Error starting easytier daemon.")?;

    println!("Easytier started, waiting for 5 seconds...");
    sleep(Duration::new(5, 0));

    let signed_in = Arc::new(Mutex::new(false));
    let user_data_info = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    let (signed_in_copy_1, user_data_info_copy_1) = (signed_in.clone(), user_data_info.clone());
    thread::spawn(|| {
        // println!("Client API started!");
        client_api::sync_server(signed_in_copy_1, user_data_info_copy_1)
    });

    easytier_daemon.wait().context("Easytier didn't start.")?;

    Ok(())
}
