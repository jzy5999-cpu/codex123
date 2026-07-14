use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use std::time::Duration;

use crate::user_scripts::UserScriptManager;

pub const DEFAULT_MARKET_INDEX_URL: &str =
    "https://raw.githubusercontent.com/BigPizzaV3/CodexPlusPlusScriptMarket/main/index.json";
const SCRIPT_MARKET_USER_AGENT: &str = "codex123/script-market";
const SCRIPT_MARKET_REQUEST_TIMEOUT: Duration = Duration::from_secs(12);
const SCRIPT_MARKET_RETRY_DELAYS: [Duration; 2] =
    [Duration::from_millis(250), Duration::from_millis(750)];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptMarketSource {
    Network,
    Cache,
}

#[derive(Debug, Clone)]
pub struct ScriptMarketFetchResult {
    pub manifest: ScriptMarketManifest,
    pub source: ScriptMarketSource,
    pub warning: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScriptMarketManifest {
    pub version: u64,
    pub updated_at: Option<String>,
    pub scripts: Vec<MarketScript>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketScript {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub homepage: String,
    pub script_url: String,
    #[serde(default)]
    pub sha256: String,
}

pub fn parse_market_manifest(raw: Value) -> anyhow::Result<ScriptMarketManifest> {
    if !raw.get("scripts").is_some_and(Value::is_array) {
        anyhow::bail!("script market manifest is missing the scripts array");
    }
    let version = raw.get("version").and_then(Value::as_u64).unwrap_or(1);
    let updated_at = raw
        .get("updated_at")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    let scripts: Vec<MarketScript> = raw
        .get("scripts")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(parse_market_script)
        .collect();

    if scripts.is_empty() {
        anyhow::bail!("script market manifest contains no valid scripts");
    }

    Ok(ScriptMarketManifest {
        version,
        updated_at,
        scripts,
    })
}

pub async fn fetch_market_manifest(url: &str) -> anyhow::Result<ScriptMarketManifest> {
    let client = crate::http_client::proxied_client(SCRIPT_MARKET_USER_AGENT)?;
    fetch_market_manifest_with_client(&client, url).await
}

pub async fn fetch_market_manifest_cached(
    url: &str,
    cache_path: &Path,
) -> anyhow::Result<ScriptMarketFetchResult> {
    match fetch_market_manifest(url).await {
        Ok(manifest) => {
            if let Ok(bytes) = serde_json::to_vec_pretty(&manifest) {
                let _ = crate::settings::atomic_write(cache_path, &bytes);
            }
            Ok(ScriptMarketFetchResult {
                manifest,
                source: ScriptMarketSource::Network,
                warning: None,
            })
        }
        Err(network_error) => {
            let cached = std::fs::read(cache_path).with_context(|| {
                format!("脚本市场联网失败：{network_error}；且没有可用本地缓存")
            })?;
            let raw = serde_json::from_slice::<Value>(&cached)
                .context("脚本市场联网失败，且本地缓存不是有效 JSON")?;
            let manifest =
                parse_market_manifest(raw).context("脚本市场联网失败，且本地缓存清单无效")?;
            Ok(ScriptMarketFetchResult {
                manifest,
                source: ScriptMarketSource::Cache,
                warning: Some(format!("联网刷新失败，当前显示最近缓存：{network_error}")),
            })
        }
    }
}

pub async fn download_script(url: &str) -> anyhow::Result<Vec<u8>> {
    let client = crate::http_client::proxied_client(SCRIPT_MARKET_USER_AGENT)?;
    let response = send_with_retry(&client, url, "script download").await?;
    Ok(response
        .bytes()
        .await
        .context("failed to read script download body")?
        .to_vec())
}

async fn fetch_market_manifest_with_client(
    client: &reqwest::Client,
    url: &str,
) -> anyhow::Result<ScriptMarketManifest> {
    let response = send_with_retry(client, url, "script market index").await?;
    let raw = response
        .json::<Value>()
        .await
        .context("failed to decode script market index JSON")?;
    parse_market_manifest(raw)
}

async fn send_with_retry(
    client: &reqwest::Client,
    url: &str,
    label: &str,
) -> anyhow::Result<reqwest::Response> {
    let mut last_error = None;
    for attempt in 0..=SCRIPT_MARKET_RETRY_DELAYS.len() {
        match client
            .get(url)
            .timeout(SCRIPT_MARKET_REQUEST_TIMEOUT)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
        {
            Ok(response) => return Ok(response),
            Err(error) => last_error = Some(error),
        }
        if let Some(delay) = SCRIPT_MARKET_RETRY_DELAYS.get(attempt) {
            tokio::time::sleep(*delay).await;
        }
    }
    Err(anyhow::Error::from(
        last_error.expect("script market retry loop should record an error"),
    ))
    .with_context(|| format!("failed to request {label} {url} after 3 attempts"))
}

pub fn install_market_script_content(
    manager: &UserScriptManager,
    script: &MarketScript,
    content: &[u8],
) -> anyhow::Result<()> {
    let path = manager.user_script_path_for_market_id(&script.id);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create user script directory {}",
                parent.display()
            )
        })?;
    }
    crate::settings::atomic_write(&path, content)
        .with_context(|| format!("failed to write script {}", path.display()))?;
    manager.record_market_install(script)?;
    Ok(())
}

pub async fn install_market_script(
    manager: &UserScriptManager,
    script: &MarketScript,
) -> anyhow::Result<()> {
    let content = download_script(&script.script_url).await?;
    install_market_script_content(manager, script, &content)
}

fn parse_market_script(raw: Value) -> Option<MarketScript> {
    let id = required_string(&raw, "id")?;
    let name = required_string(&raw, "name")?;
    let version = required_string(&raw, "version")?;
    let script_url = required_string(&raw, "script_url")?;
    Some(MarketScript {
        id,
        name,
        description: optional_string(&raw, "description"),
        version,
        author: optional_string(&raw, "author"),
        tags: raw
            .get("tags")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default(),
        homepage: optional_string(&raw, "homepage"),
        script_url,
        sha256: optional_string(&raw, "sha256"),
    })
}

fn required_string(raw: &Value, key: &str) -> Option<String> {
    raw.get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn optional_string(raw: &Value, key: &str) -> String {
    raw.get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .unwrap_or_default()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn valid_manifest() -> Value {
        json!({
            "version": 1,
            "updated_at": "2026-07-14T00:00:00Z",
            "scripts": [{
                "id": "sample",
                "name": "Sample",
                "version": "1.0.0",
                "script_url": "https://example.test/sample.js"
            }]
        })
    }

    #[test]
    fn manifest_rejects_missing_or_empty_script_list() {
        assert!(parse_market_manifest(json!({"version": 1})).is_err());
        assert!(parse_market_manifest(json!({"version": 1, "scripts": []})).is_err());
    }

    #[tokio::test]
    async fn cached_fetch_falls_back_when_network_is_unavailable() {
        let temp = tempfile::tempdir().unwrap();
        let cache = temp.path().join("script-market.json");
        std::fs::write(&cache, serde_json::to_vec(&valid_manifest()).unwrap()).unwrap();

        let result = fetch_market_manifest_cached("http://127.0.0.1:9/index.json", &cache)
            .await
            .unwrap();

        assert_eq!(result.source, ScriptMarketSource::Cache);
        assert_eq!(result.manifest.scripts[0].id, "sample");
        assert!(result.warning.unwrap().contains("联网刷新失败"));
    }
}
