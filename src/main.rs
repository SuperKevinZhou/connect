mod compute_sha;

use std::env;
use std::process::{Command, Child};
use std::path::{MAIN_SEPARATOR, PathBuf};

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


    let mut easytier_daemon = Command::new(easytier_core_path)
        .arg("-d")
        .arg("--network-name")
        .arg(network_name)
        .arg("--network-secret")
        .arg(network_secret)
        .arg("-p")
        .arg("tcp://public.easytier.top:11010")
        .arg("-p")
        .arg("tcp://gz.minebg.top:11010")
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

    easytier_daemon.wait().context("Easytier didn't start.")?;

    Ok(())
}
