# 🌐 FAIR — Free AI In Russia

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Slint](https://img.shields.io/badge/Slint-1.17+-blue?style=for-the-badge)](https://slint.dev/)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)
[![Arch Linux](https://img.shields.io/badge/Arch_Linux-supported-1793D1?style=for-the-badge&logo=arch-linux)](https://archlinux.org/)

**Обход блокировок AI-сервисов через `/etc/hosts`. Без VPN. Без регистрации. Без прокси.**

[🇷🇺 Русский](#ru) · [🇬🇧 English](#en) · [⚡ Установка](#install) · [⚠️ Ограничения](#limits) · [🔧 Разработка](#dev)

---

<a id="ru"></a>

## 🇷🇺 Русский

### Что делает FAIR

1. Получает актуальные IPv4-адреса сервисов через публичный DNS `dns.comss.one`.
2. Записывает их в `/etc/hosts` внутри маркерного блока.
3. Делает бэкап `/etc/hosts` перед каждой правкой.
4. Сбрасывает системный DNS-кэш.
5. Проверяет доступность сервисов через `curl`.
6. Умеет удалять только свои записи — не трогая остальной файл.

### Поддерживаемые сервисы

| Категория | Сервис | Статус |
|-----------|--------|--------|
| AI | ChatGPT, Claude, Grok, Copilot, Perplexity | ✅ Полный |
| AI | Gemini | Проблемы на ПК ⚠️ |
| Media | YouTube | ⚠️ Частичный |
| Social | Discord, Instagram | ⚠️ Частичный |
| Cloud | AWS | ⚠️ Частичный |

**«Частичный»** означает: страница откроется, но не всё содержимое будет работать. YouTube-видео идут через `googlevideo.com` с динамическими IP — hosts покрывает лишь часть. Discord-голос работает через UDP/WebRTC — hosts на это не влияет. AWS покрыт только на уровне входа и S3.

<a id="install"></a>

### ⚡ Установка

**Требования:** Arch Linux, `cargo`, `nslookup` (пакет `bind`), `curl`.

```bash
git clone https://github.com/fair-project/fair.git
cd fair
makepkg -si
sudo fair
```

**Первый запуск** требует root — FAIR пишет в `/etc/hosts`. Из меню приложений запускается через `pkexec`, из терминала — через `sudo fair`.

<a id="limits"></a>

### ⚠️ Ограничения

- **FAIR обходит только DNS-блокировки.** Если РКН режет по SNI, по IP или по протоколу — hosts бессилен.
- **DPI-обход FAIR не делает.** YouTube-замедление, Discord-голос, Telegram — это DPI. Нужен `zapret2`, `AmneziaVPN` или `v2ray`.
- **Amazon (магазин) не заработает** — он ушёл из РФ сам, карты не принимаются. AWS покрыт только частично.
- **Резервная копия** `/etc/hosts` лежит в `/etc/hosts.fair.bak`. Откат: `sudo cp /etc/hosts.fair.bak /etc/hosts`.

### Ручной откат

```bash
sudo cp /etc/hosts.fair.bak /etc/hosts
sudo resolvectl flush-caches
```

[⬆ Наверх](#ru) · [🇬🇧 English](#en)

---

<a id="en"></a>

## 🇬🇧 English

### What FAIR does

1. Resolves current IPv4 addresses via public DNS `dns.comss.one`.
2. Writes them into `/etc/hosts` inside a marker block.
3. Backs up `/etc/hosts` before every modification.
4. Flushes the system DNS cache.
5. Probes service availability via `curl`.
6. Removes only its own entries — never touches the rest of the file.

### Supported services

| Category | Service | Status |
|----------|---------|--------|
| AI | ChatGPT, Claude, Grok, Copilot, Perplexity | ✅ Full |
| AI | Gemini | Doesnt work correctly on PC ⚠️ |
| Media | YouTube | ⚠️ Partial |
| Social | Discord, Instagram | ⚠️ Partial |
| Cloud | AWS | ⚠️ Partial |

**Partial** means: the page loads, but not everything works. YouTube videos go through `googlevideo.com` with dynamic IPs — hosts covers only a fraction. Discord voice uses UDP/WebRTC — hosts has no effect. AWS is covered for sign-in and S3 only.

<a id="install-en"></a>

### ⚡ Installation

**Requirements:** Arch Linux, `cargo`, `nslookup` (package `bind`), `curl`.

```bash
git clone https://github.com/fair-project/fair.git
cd fair
makepkg -si
sudo fair
```

**First launch** needs root — FAIR writes to `/etc/hosts`. Launch from the app menu via `pkexec`, from terminal via `sudo fair`.

<a id="limits-en"></a>

### ⚠️ Limitations

- **FAIR bypasses DNS blocks only.** If Roskomnadzor filters by SNI, by IP or by protocol — hosts won't help.
- **No DPI bypass.** YouTube throttling, Discord voice, Telegram — that's DPI. Use `zapret2`, `AmneziaVPN` or `v2ray`.
- **Amazon (store) won't work** — it left Russia voluntarily, cards are not accepted. AWS is only partially covered.
- **Backup** of `/etc/hosts` is at `/etc/hosts.fair.bak`. Rollback: `sudo cp /etc/hosts.fair.bak /etc/hosts`.

### Manual rollback

```bash
sudo cp /etc/hosts.fair.bak /etc/hosts
sudo resolvectl flush-caches
```

[⬆ Back to top](#en) · [🇷🇺 Русский](#ru)

---

<a id="dev"></a>

## 🔧 Разработка / Development

```bash
cargo build             # debug
cargo build --release   # release
makepkg -si             # install package
```

**Структура / Structure:**

```
src/
├── main.rs        — entry point
├── app.rs         — state, orchestration
├── hosts.rs       — /etc/hosts read/write
├── dns.rs         — nslookup wrapper
├── network.rs     — flush cache, curl probes
├── services.rs    — service list
├── ui.rs          — Slint bridge
└── error.rs       — FairError enum
ui/
├── app.slint      — main window
├── theme.slint    — colors, fonts
└── components/    — reusable widgets
```

## 📄 Лицензия / License

MIT — см. [LICENSE](LICENSE).
```

## Пересборка

```bash
cd /home/giti/dev/fair
makepkg -si
```
