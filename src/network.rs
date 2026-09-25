use crate::error::{FairError, Result};
use tokio::process::Command;

pub async fn flush_dns_cache() -> Result<()> {
    let candidates: &[(&str, &[&str])] = &[
        ("resolvectl", &["flush-caches"]),
        ("systemd-resolve", &["--flush-caches"]),
        ("nscd", &["-i", "hosts"]),
    ];

    for &(cmd, args) in candidates {
        if let Ok(out) = Command::new(cmd).args(args).kill_on_drop(true).output().await {
            if out.status.success() {
                return Ok(());
            }
        }
    }

    Err(FairError::Service("не удалось очистить DNS-кэш".into()))
}

const UA: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0";

pub async fn check_service(url: &str) -> Result<u16> {
    let out = Command::new("curl")
        .args([
            "-sS",
            "-o", "/dev/null",
            "-w", "%{http_code}",
            "-L",
            "--max-time", "10",
            "--connect-timeout", "5",
            "-A", UA,
            "-H", "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            "-H", "Accept-Language: en-US,en;q=0.5",
            url,
        ])
        .kill_on_drop(true)
        .output()
        .await?;

    let code: u16 = String::from_utf8_lossy(&out.stdout).trim().parse().unwrap_or(0);
    Ok(code)
}