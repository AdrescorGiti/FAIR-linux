use crate::error::{FairError, Result};
use crate::services::MARKER;
use std::fs;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

pub const HOSTS_PATH: &str = "/etc/hosts";
const BACKUP_SUFFIX: &str = ".fair.bak";

pub struct HostEntry {
    pub ip: String,
    pub domain: String,
}

pub fn read_hosts() -> Result<String> {
    let raw = fs::read(HOSTS_PATH)?;
    String::from_utf8(raw).map_err(|_| FairError::HostsCorrupted("не UTF-8".into()))
}

pub fn write_hosts(content: &str) -> Result<()> {
    fs::write(HOSTS_PATH, content).map_err(FairError::from)
}

pub fn is_enabled() -> bool {
    read_hosts().map(|c| c.contains(MARKER)).unwrap_or(false)
}

pub fn backup() -> Result<PathBuf> {
    let target = PathBuf::from(format!("{HOSTS_PATH}{BACKUP_SUFFIX}"));
    fs::copy(Path::new(HOSTS_PATH), &target)?;
    Ok(target)
}

pub fn append_entries(entries: &[HostEntry]) -> Result<()> {
    backup()?;

    let (mut body, _) = strip_entries(&read_hosts()?);
    if !body.is_empty() {
        if !body.ends_with('\n') {
            body.push('\n');
        }
        body.push('\n');
    }
    body.push_str(MARKER);
    body.push('\n');
    for entry in entries {
        body.push_str(&entry.ip);
        body.push('\t');
        body.push_str(&entry.domain);
        body.push('\n');
    }

    write_hosts(&body)
}

pub fn remove_entries() -> Result<()> {
    let current = read_hosts()?;
    let (stripped, removed) = strip_entries(&current);
    if !removed {
        return Ok(());
    }
    backup()?;
    write_hosts(&stripped)
}

fn strip_entries(content: &str) -> (String, bool) {
    let mut out = String::with_capacity(content.len());
    let mut inside_block = false;
    let mut removed = false;

    for line in content.lines() {
        if inside_block {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                removed = true;
                continue;
            }
            let head = trimmed.split_whitespace().next().unwrap_or("");
            if head.parse::<IpAddr>().is_ok() {
                removed = true;
                continue;
            }
            inside_block = false;
        }

        if line.trim() == MARKER {
            inside_block = true;
            removed = true;
            continue;
        }

        out.push_str(line);
        out.push('\n');
    }

    (out, removed)
}