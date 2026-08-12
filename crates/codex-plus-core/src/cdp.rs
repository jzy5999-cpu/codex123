use anyhow::{Context, bail};
use serde::Deserialize;
use std::time::Duration;

const CDP_HTTP_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct CdpTarget {
    pub id: String,
    #[serde(rename = "type")]
    pub target_type: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub url: String,
    #[serde(default, rename = "webSocketDebuggerUrl")]
    pub web_socket_debugger_url: Option<String>,
}

pub async fn list_targets(debug_port: u16) -> anyhow::Result<Vec<CdpTarget>> {
    let urls = [
        format!("http://127.0.0.1:{debug_port}/json"),
        format!("http://[::1]:{debug_port}/json"),
    ];
    let client = reqwest::Client::builder()
        .no_proxy()
        .timeout(CDP_HTTP_TIMEOUT)
        .build()
        .context("failed to build CDP HTTP client")?;

    let mut last_error = None;
    for url in urls {
        match client.get(&url).send().await {
            Ok(response) => {
                let response = response
                    .error_for_status()
                    .with_context(|| format!("CDP target query failed at {url}"))?;
                return response
                    .json::<Vec<CdpTarget>>()
                    .await
                    .context("failed to deserialize CDP targets");
            }
            Err(error) => {
                last_error = Some(error);
            }
        }
    }

    Err(anyhow::Error::from(
        last_error.expect("CDP target URL list should not be empty"),
    ))
    .context("failed to query CDP targets")
}

pub fn pick_page_target(targets: &[CdpTarget]) -> anyhow::Result<CdpTarget> {
    // Only inject into the official desktop surface. Embedded Browser pages
    // can contain "Codex" in their title or URL and must not receive scripts.
    if let Some(target) = targets.iter().find(|target| {
        is_injectable_page_target(target)
            && !is_ignored_codex_page_target(target)
            && (is_codex_app_page_target(target)
                || is_chatgpt_desktop_page(&target.title, &target.url))
    }) {
        return Ok(target.clone());
    }
    bail!("No injectable Codex page target found")
}

fn is_injectable_page_target(target: &CdpTarget) -> bool {
    target.target_type == "page"
        && target
            .web_socket_debugger_url
            .as_deref()
            .is_some_and(|url| !url.is_empty())
}

fn is_codex_app_page_target(target: &CdpTarget) -> bool {
    let Ok(url) = reqwest::Url::parse(target.url.trim()) else {
        return false;
    };
    url.scheme().eq_ignore_ascii_case("app")
        && url.host_str() == Some("-")
        && url.path().eq_ignore_ascii_case("/index.html")
}

fn is_chatgpt_desktop_page(title: &str, url: &str) -> bool {
    let title = title.trim().to_ascii_lowercase();
    let url = url.trim().to_ascii_lowercase();
    title == "chatgpt"
        && (url == "https://chatgpt.com"
            || url.starts_with("https://chatgpt.com/")
            || url == "https://chat.openai.com"
            || url.starts_with("https://chat.openai.com/")
            || url.starts_with("data:text/html"))
}

fn is_ignored_codex_page_target(target: &CdpTarget) -> bool {
    is_avatar_overlay_page_target(target) || is_quick_chat_page_target(target)
}

pub fn is_avatar_overlay_page_target(target: &CdpTarget) -> bool {
    initial_route(target).is_some_and(|route| route.eq_ignore_ascii_case("/avatar-overlay"))
}

pub fn is_quick_chat_page_target(target: &CdpTarget) -> bool {
    initial_route(target).is_some_and(|route| {
        let route = route.to_ascii_lowercase();
        route == "/chatgpt/quick-chat"
            || route == "/chatgpt/quick-chat-prewarm"
            || route.starts_with("/chatgpt/quick-chat/")
    })
}

fn initial_route(target: &CdpTarget) -> Option<String> {
    if !is_injectable_page_target(target) {
        return None;
    }
    let url = reqwest::Url::parse(target.url.trim()).ok()?;
    if !url.scheme().eq_ignore_ascii_case("app")
        || url.host_str() != Some("-")
        || !url.path().eq_ignore_ascii_case("/index.html")
    {
        return None;
    }
    url.query_pairs()
        .find(|(key, _)| key.eq_ignore_ascii_case("initialRoute"))
        .map(|(_, value)| value.into_owned())
}
