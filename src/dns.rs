use crate::error::{FairError, Result};
use crate::services::{DNS_SERVERS, DOH_ENDPOINTS};
use std::net::IpAddr;
use tokio::process::Command;

pub async fn resolve_ip(domain: &str) -> Result<IpAddr> {
    for endpoint in DOH_ENDPOINTS {
        if let Ok(ip) = query_doh(domain, endpoint).await {
            return Ok(ip);
        }
    }

    let mut last_err = FairError::Dns(format!("{domain}: не удалось разрешить ни через один сервер"));
    for server in DNS_SERVERS {
        match query_plain(domain, server).await {
            Ok(ip) => return Ok(ip),
            Err(err) => last_err = err,
        }
    }

    Err(last_err)
}

async fn query_doh(domain: &str, endpoint: &str) -> Result<IpAddr> {
    let url = format!("{endpoint}?name={domain}&type=A");
    let output = Command::new("curl")
        .args(["-sS", "--max-time", "6", "-H", "Accept: application/dns-json", &url])
        .kill_on_drop(true)
        .output()
        .await?;

    let body = String::from_utf8_lossy(&output.stdout);
    parse_doh_answer(&body)
        .ok_or_else(|| FairError::Dns(format!("{domain}: DoH {endpoint} без ответа")))
}

fn parse_doh_answer(body: &str) -> Option<IpAddr> {
    let marker = "\"type\":1";
    let mut search_from = 0usize;

    while let Some(rel) = body[search_from..].find(marker) {
        let type_pos = search_from + rel;
        let after = &body[type_pos..];
        if let Some(data_rel) = after.find("\"data\":\"") {
            let value_start = type_pos + data_rel + "\"data\":\"".len();
            if let Some(end_rel) = body[value_start..].find('"') {
                let candidate = &body[value_start..value_start + end_rel];
                if let Ok(ip) = candidate.parse::<IpAddr>() {
                    if ip.is_ipv4() {
                        return Some(ip);
                    }
                }
            }
        }
        search_from = type_pos + marker.len();
    }

    None
}

async fn query_plain(domain: &str, dns_server: &str) -> Result<IpAddr> {
    let output = Command::new("nslookup")
        .arg(domain)
        .arg(dns_server)
        .kill_on_drop(true)
        .output()
        .await?;

    let text = String::from_utf8_lossy(&output.stdout);
    parse_plain_answer(&text)
        .ok_or_else(|| FairError::Dns(format!("{domain}: A-запись не найдена через {dns_server}")))
}

fn parse_plain_answer(text: &str) -> Option<IpAddr> {
    for raw in text.lines() {
        let line = raw.trim();
        let Some(rest) = line.strip_prefix("Address:") else {
            continue;
        };
        let candidate = rest.trim();
        if candidate.contains('#') {
            continue;
        }
        if let Ok(ip) = candidate.parse::<IpAddr>() {
            if ip.is_ipv4() {
                return Some(ip);
            }
        }
    }
    None
}