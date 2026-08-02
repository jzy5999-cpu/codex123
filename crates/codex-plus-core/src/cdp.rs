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
    let pages = targets.iter().filter(|target| {
        target.target_type == "page"
            && !is_ignored_codex_page_target(target)
            && target
                .web_socket_debugger_url
                .as_deref()
                .is_some_and(|url| !url.is_empty())
    });

    let mut first_page = None;
    for target in pages {
        first_page.get_or_insert(target);
        let haystack = format!("{} {}", target.title, target.url).to_lowercase();
        if haystack.contains("codex") {
            return Ok(target.clone());
        }
    }

    if let Some(target) = first_page {
        return Ok(target.clone());
    }

    bail!("No injectable Codex page target found")
}

fn is_ignored_codex_page_target(target: &CdpTarget) -> bool {
    is_avatar_overlay_page_target(target) || is_quick_chat_page_target(target)
}

pub fn is_avatar_overlay_page_target(target: &CdpTarget) -> bool {
    let haystack = format!("{} {}", target.title, target.url).to_lowercase();
    [
        "initialroute=%2favatar-overlay",
        "initialroute=/avatar-overlay",
        "/avatar-overlay",
        "avatar-overlay",
    ]
    .iter()
    .any(|marker| haystack.contains(marker))
}

pub fn is_quick_chat_page_target(target: &CdpTarget) -> bool {
    if target.target_type != "page" {
        return false;
    }
    let url = target.url.to_lowercase();
    if !url.starts_with("app://") {
        return false;
    }
    [
        "initialroute=%2fchatgpt%2fquick-chat",
        "initialroute=/chatgpt/quick-chat",
    ]
    .iter()
    .any(|marker| url.contains(marker))
}
