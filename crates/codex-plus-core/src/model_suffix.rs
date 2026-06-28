//! Model list context-window parsing and catalog generation.
//!
//! Legacy suffix syntax is still accepted: `deepseek-chat[1M]` means
//! slug=`deepseek-chat` and context window=`1000000`. New UI stores windows in
//! `RelayProfile.model_windows` instead of appending suffixes to model names.

use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelCatalogEntry {
    pub slug: String,
    pub display_name: String,
    pub suffix_window: Option<u64>,
}

pub fn parse_model_suffix(raw: &str) -> (String, Option<u64>) {
    let raw = raw.trim();
    if let Some(close) = raw.rfind(']') {
        if close == raw.len() - 1 {
            if let Some(open) = raw[..close].rfind('[') {
                let inner = raw[open + 1..close].trim();
                let slug = raw[..open].trim();
                if !slug.is_empty() {
                    if let Some(window) = parse_window_token(inner) {
                        return (slug.to_string(), Some(window));
                    }
                }
            }
        }
    }
    (raw.to_string(), None)
}

pub fn migrate_model_list_with_suffixes(model_list: &str) -> (String, HashMap<String, String>) {
    let mut clean_lines = Vec::new();
    let mut windows = HashMap::new();
    for raw in model_list
        .split(['\r', '\n', ','])
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let (slug, window) = parse_model_suffix(raw);
        clean_lines.push(slug.clone());
        if let Some(window) = window {
            windows.insert(slug, window.to_string());
        }
    }
    (clean_lines.join("\n"), windows)
}

fn parse_window_token(token: &str) -> Option<u64> {
    let token = token.trim();
    if token.is_empty() {
        return None;
    }
    let (num_part, multiplier) = match token.chars().last() {
        Some('K' | 'k') => (&token[..token.len() - 1], 1_000u64),
        Some('M' | 'm') => (&token[..token.len() - 1], 1_000_000u64),
        Some(_) => (token, 1u64),
        None => return None,
    };
    num_part
        .trim()
        .parse::<u64>()
        .ok()
        .map(|value| value * multiplier)
        .filter(|value| *value > 0)
}

pub fn collect_catalog_entries(
    model_list: &str,
    model_windows: &HashMap<String, String>,
    current_model: &str,
) -> Vec<ModelCatalogEntry> {
    let mut seen = HashSet::new();
    let mut list_entries = Vec::new();
    for raw in model_list
        .split(['\r', '\n', ','])
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let (slug, _) = parse_model_suffix(raw);
        if slug.is_empty() || !seen.insert(slug.clone()) {
            continue;
        }
        let suffix_window = model_windows
            .get(&slug)
            .and_then(|token| parse_window_token(token));
        list_entries.push(ModelCatalogEntry {
            display_name: slug.clone(),
            slug,
            suffix_window,
        });
    }

    let mut entries = Vec::new();
    let current_model = current_model.trim();
    if !current_model.is_empty() {
        let (slug, _) = parse_model_suffix(current_model);
        if !slug.is_empty() {
            let suffix_window = model_windows
                .get(&slug)
                .and_then(|token| parse_window_token(token));
            entries.push(ModelCatalogEntry {
                display_name: slug.clone(),
                slug: slug.clone(),
                suffix_window,
            });
            list_entries.retain(|entry| entry.slug != slug);
        }
    }

    entries.append(&mut list_entries);
    entries
}

const BUNDLED_TEMPLATE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/codex-models.json"
));

pub fn build_model_catalog_json(
    entries: &[ModelCatalogEntry],
    fallback_window: Option<u64>,
) -> String {
    build_model_catalog_json_with_template(entries, fallback_window, None)
}

pub fn build_model_catalog_json_with_template(
    entries: &[ModelCatalogEntry],
    fallback_window: Option<u64>,
    template: Option<&Value>,
) -> String {
    let template = template
        .cloned()
        .or_else(load_bundled_template_entry)
        .unwrap_or_else(|| json!({}));

    let models: Vec<Value> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let context_window = entry.suffix_window.or(fallback_window).unwrap_or(272_000);
            let mut model = template.clone();
            model["slug"] = json!(entry.slug);
            model["display_name"] = json!(entry.display_name);
            model["description"] = json!(entry.display_name);
            model["context_window"] = json!(context_window);
            model["max_context_window"] = json!(context_window);
            model["effective_context_window_percent"] = json!(100);
            model["auto_compact_token_limit"] = Value::Null;
            model["priority"] = json!(1000 + index);
            model["visibility"] = json!("list");
            model["supported_in_api"] = json!(true);
            model["additional_speed_tiers"] = json!([]);
            model["service_tiers"] = json!([]);
            model["availability_nux"] = Value::Null;
            model["upgrade"] = Value::Null;
            model
        })
        .collect();
    serde_json::to_string_pretty(&json!({ "models": models })).unwrap_or_default()
}

fn load_bundled_template_entry() -> Option<Value> {
    let catalog: Value = serde_json::from_str(BUNDLED_TEMPLATE_JSON).ok()?;
    catalog.get("models")?.as_array()?.first().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_model_suffix_units() {
        assert_eq!(
            parse_model_suffix("deepseek-chat[1M]"),
            ("deepseek-chat".to_string(), Some(1_000_000))
        );
        assert_eq!(
            parse_model_suffix("qwen[200K]"),
            ("qwen".to_string(), Some(200_000))
        );
        assert_eq!(
            parse_model_suffix("plain-model"),
            ("plain-model".to_string(), None)
        );
    }

    #[test]
    fn migrates_model_list_suffixes() {
        let (clean, windows) = migrate_model_list_with_suffixes("a[1M]\nb\nc[200K]");
        assert_eq!(clean, "a\nb\nc");
        assert_eq!(windows.get("a").map(String::as_str), Some("1000000"));
        assert_eq!(windows.get("c").map(String::as_str), Some("200000"));
    }

    #[test]
    fn collects_current_model_first_and_uses_windows_map() {
        let windows = HashMap::from([
            ("b".to_string(), "1M".to_string()),
            ("a".to_string(), "200000".to_string()),
        ]);
        let entries = collect_catalog_entries("a\nb", &windows, "b");
        assert_eq!(entries[0].slug, "b");
        assert_eq!(entries[0].suffix_window, Some(1_000_000));
        assert_eq!(entries[1].slug, "a");
        assert_eq!(entries[1].suffix_window, Some(200_000));
    }
}
