use crate::dns;
use crate::error::{FairError, Result};
use crate::hosts::{self, HostEntry};
use crate::network;
use crate::services::{PARTIAL, PROBES, SERVICES};
use std::net::IpAddr;
use std::sync::Mutex;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time::timeout;

const RESOLVE_TIMEOUT: Duration = Duration::from_secs(20);
const PROBE_TIMEOUT: Duration = Duration::from_secs(12);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Enabled,
    Disabled,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct CheckResult {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

pub struct AppState {
    log: Mutex<Vec<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            log: Mutex::new(Vec::new()),
        }
    }

    pub fn status(&self) -> Status {
        if hosts::read_hosts().is_err() {
            return Status::Unknown;
        }
        if hosts::is_enabled() {
            Status::Enabled
        } else {
            Status::Disabled
        }
    }

    pub fn drain_log(&self) -> Vec<String> {
        self.log
            .lock()
            .map(|mut guard| std::mem::take(&mut *guard))
            .unwrap_or_default()
    }

    pub async fn enable(&self) -> Result<()> {
        self.record("[*] Резолвлю адреса параллельно через несколько DNS-серверов");

        let mut set = JoinSet::new();
        let mut order = 0usize;
        for (name, domains) in SERVICES {
            for domain in *domains {
                let index = order;
                order += 1;
                let name = *name;
                let domain = *domain;
                set.spawn(async move {
                    let outcome = timeout(RESOLVE_TIMEOUT, dns::resolve_ip(domain))
                        .await
                        .unwrap_or_else(|_| {
                            Err(FairError::Dns(format!("{domain}: превышено время ожидания")))
                        });
                    (index, name, domain, outcome)
                });
            }
        }

        let mut results: Vec<(usize, &str, &str, Result<IpAddr>)> = Vec::new();
        while let Some(joined) = set.join_next().await {
            if let Ok(item) = joined {
                results.push(item);
            }
        }
        results.sort_by_key(|(index, _, _, _)| *index);

        let mut entries = Vec::new();
        for (_, name, domain, outcome) in results {
            match outcome {
                Ok(ip) => {
                    self.record(format!("[✓] {name}: {domain} → {ip}"));
                    entries.push(HostEntry {
                        ip: ip.to_string(),
                        domain: domain.to_string(),
                    });
                }
                Err(err) => self.record(format!("[!] {name}: {domain} — {err}")),
            }
        }

        if entries.is_empty() {
            return Err(FairError::Dns("ни один домен не разрешился".into()));
        }

        hosts::append_entries(&entries)?;
        self.record(format!(
            "[✓] /etc/hosts обновлён ({} записей)",
            entries.len()
        ));

        for name in PARTIAL {
            if SERVICES.iter().any(|(n, _)| n == name) {
                self.record(format!(
                    "[!] {name}: покрыт частично, возможны ограничения"
                ));
            }
        }

        match network::flush_dns_cache().await {
            Ok(()) => self.record("[✓] DNS-кэш очищен"),
            Err(err) => self.record(format!("[!] DNS-кэш: {err}")),
        }

        Ok(())
    }

    pub async fn disable(&self) -> Result<()> {
        if !hosts::is_enabled() {
            self.record("[*] FAIR уже отключён");
            return Ok(());
        }

        hosts::remove_entries()?;
        self.record("[✓] Записи FAIR удалены из /etc/hosts");

        match network::flush_dns_cache().await {
            Ok(()) => self.record("[✓] DNS-кэш очищен"),
            Err(err) => self.record(format!("[!] DNS-кэш: {err}")),
        }

        Ok(())
    }

    pub async fn check(&self) -> Result<Vec<CheckResult>> {
        self.record("[*] Проверяю доступность сервисов параллельно");

        let mut set = JoinSet::new();
        for (index, (name, url)) in PROBES.iter().enumerate() {
            let name = *name;
            let url = *url;
            set.spawn(async move {
                let code = timeout(PROBE_TIMEOUT, network::check_service(url))
                    .await
                    .unwrap_or(Ok(0))
                    .unwrap_or(0);
                (index, name, code)
            });
        }

        let mut raw: Vec<(usize, &str, u16)> = Vec::new();
        while let Some(joined) = set.join_next().await {
            if let Ok(item) = joined {
                raw.push(item);
            }
        }
        raw.sort_by_key(|(index, _, _)| *index);

        let mut results = Vec::new();
        for (_, name, code) in raw {
            let result = match code {
                0 => CheckResult {
                    name: name.to_string(),
                    ok: false,
                    detail: "нет соединения".into(),
                },
                code if code < 400 => CheckResult {
                    name: name.to_string(),
                    ok: true,
                    detail: format!("доступен · HTTP {code}"),
                },
                code if code == 403 || code == 503 => CheckResult {
                    name: name.to_string(),
                    ok: true,
                    detail: format!("доступен · защита HTTP {code}"),
                },
                code => CheckResult {
                    name: name.to_string(),
                    ok: false,
                    detail: format!("HTTP {code}"),
                },
            };

            self.record(format!(
                "{} {}: {}",
                if result.ok { "[✓]" } else { "[✗]" },
                result.name,
                result.detail
            ));
            results.push(result);
        }

        Ok(results)
    }

    fn record(&self, line: impl Into<String>) {
        if let Ok(mut guard) = self.log.lock() {
            guard.push(line.into());
        }
    }
}