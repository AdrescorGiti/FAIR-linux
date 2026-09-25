pub const DOH_ENDPOINTS: &[&str] = &["https://1.1.1.1/dns-query", "https://dns.google/resolve"];
pub const DNS_SERVERS: &[&str] = &["dns.comss.one", "94.140.14.14", "1.1.1.1", "8.8.8.8"];
pub const MARKER: &str = "# FAIR — managed block, do not edit";

pub const SERVICES: &[(&str, &[&str])] = &[
    (
        "Gemini",
        &[
            "gemini.google.com",
            "aistudio.google.com",
            "labs.google.com",
            "alkalimakersuite-pa.clients6.google.com",
            "generativelanguage.googleapis.com",
            "accounts.google.com",
            "ogs.google.com",
            "www.gstatic.com",
        ],
    ),
    (
        "ChatGPT",
        &["chatgpt.com", "chat.openai.com", "platform.openai.com"],
    ),
    ("Claude", &["claude.ai", "anthropic.com"]),
    ("Grok", &["grok.com", "x.ai"]),
    ("Copilot", &["copilot.microsoft.com"]),
    ("Perplexity", &["perplexity.ai"]),
    (
        "YouTube",
        &[
            "youtube.com",
            "www.youtube.com",
            "m.youtube.com",
            "youtu.be",
            "i.ytimg.com",
            "s.ytimg.com",
            "yt3.ggpht.com",
        ],
    ),
    (
        "Discord",
        &[
            "discord.com",
            "www.discord.com",
            "discordapp.com",
            "discordapp.net",
            "cdn.discordapp.com",
            "media.discordapp.net",
        ],
    ),
    (
        "Instagram",
        &["instagram.com", "www.instagram.com", "cdninstagram.com"],
    ),
    (
        "AWS",
        &[
            "aws.amazon.com",
            "signin.aws.amazon.com",
            "console.aws.amazon.com",
            "s3.amazonaws.com",
            "awsapps.com",
            "sso.amazonaws.com",
        ],
    ),
    (
        "Telegram",
        &[
            "telegram.org",
            "web.telegram.org",
            "core.telegram.org",
            "desktop.telegram.org",
        ],
    ),
];

pub const PROBES: &[(&str, &str)] = &[
    ("Gemini", "https://gemini.google.com"),
    ("ChatGPT", "https://chatgpt.com"),
    ("Claude", "https://claude.ai"),
    ("Grok", "https://grok.com"),
    ("Copilot", "https://copilot.microsoft.com"),
    ("Perplexity", "https://www.perplexity.ai"),
    ("YouTube", "https://www.youtube.com"),
    ("Discord", "https://discord.com"),
    ("Instagram", "https://www.instagram.com"),
    ("AWS", "https://signin.aws.amazon.com"),
    ("Telegram", "https://web.telegram.org"),
];

pub const PARTIAL: &[&str] = &["Gemini", "YouTube", "Discord", "Instagram", "AWS", "Telegram"];