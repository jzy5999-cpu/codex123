use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, bail};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const PETDEX_MANIFEST_URL: &str = "https://petdex.crafter.run/api/manifest";
const USER_AGENT: &str = "codex123-petdex/0.2";
const MAX_MANIFEST_BYTES: usize = 5 * 1024 * 1024;
const MAX_PET_JSON_BYTES: usize = 1024 * 1024;
const MAX_SPRITESHEET_BYTES: usize = 20 * 1024 * 1024;
const INSTALL_METADATA_FILE: &str = "codex123-installed.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PetdexPet {
    pub slug: String,
    pub display_name: String,
    pub description: String,
    pub author: String,
    pub pet_json_url: String,
    pub spritesheet_url: String,
    pub homepage: String,
    pub tags: Vec<String>,
    pub installed: bool,
    pub installed_path: Option<String>,
    pub update_available: bool,
    pub heat_score: u32,
    pub heat_label: String,
    pub heat_reason: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InstalledPet {
    pub slug: String,
    pub display_name: String,
    pub path: String,
    pub spritesheet_file: Option<String>,
    pub source: String,
    pub pet_json_url: String,
    pub spritesheet_url: String,
    pub installed_at_ms: Option<u128>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PetdexManifest {
    pub manifest_url: String,
    pub pets_dir: String,
    pub pets: Vec<PetdexPet>,
    pub installed: Vec<InstalledPet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PetdexInstallRequest {
    pub slug: String,
    #[serde(default)]
    pub display_name: String,
    pub pet_json_url: String,
    pub spritesheet_url: String,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PetdexInstallResult {
    pub slug: String,
    pub path: String,
    pub pet_json_file: String,
    pub spritesheet_file: String,
    pub overwritten: bool,
}

#[derive(Debug, Clone)]
pub struct PetPackageMetadata {
    pub slug: String,
    pub display_name: String,
    pub source: String,
    pub pet_json_url: String,
    pub spritesheet_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct InstallMetadata {
    source: String,
    slug: String,
    display_name: String,
    pet_json_url: String,
    spritesheet_url: String,
    installed_at_ms: u128,
}

pub async fn fetch_manifest() -> anyhow::Result<PetdexManifest> {
    let client = crate::http_client::proxied_client(USER_AGENT)?;
    let manifest_url = Url::parse(PETDEX_MANIFEST_URL)?;
    let response = client
        .get(manifest_url.clone())
        .send()
        .await
        .context("请求 Petdex manifest 失败")?
        .error_for_status()
        .context("Petdex manifest 返回非成功状态")?;
    let bytes = response
        .bytes()
        .await
        .context("读取 Petdex manifest 失败")?;
    if bytes.len() > MAX_MANIFEST_BYTES {
        bail!(
            "Petdex manifest 超过 {} MB 限制",
            MAX_MANIFEST_BYTES / 1024 / 1024
        );
    }
    let value: Value = serde_json::from_slice(&bytes).context("Petdex manifest 不是有效 JSON")?;
    let pets_dir = default_pets_dir();
    let installed = list_installed_pets_in_dir(&pets_dir)?;
    let installed_by_slug = installed
        .iter()
        .map(|pet| (pet.slug.as_str(), pet.clone()))
        .collect::<std::collections::HashMap<_, _>>();
    let mut pets = parse_manifest_pets(&value)
        .into_iter()
        .map(|mut pet| {
            if let Some(installed) = installed_by_slug.get(pet.slug.as_str()) {
                pet.installed = true;
                pet.installed_path = Some(installed.path.clone());
                pet.update_available = installed.pet_json_url != pet.pet_json_url
                    || installed.spritesheet_url != pet.spritesheet_url;
            }
            apply_pet_heat(&mut pet);
            pet
        })
        .collect::<Vec<_>>();
    sort_pets_by_heat(&mut pets);
    Ok(PetdexManifest {
        manifest_url: manifest_url.to_string(),
        pets_dir: pets_dir.to_string_lossy().to_string(),
        pets,
        installed,
    })
}

pub fn list_installed_pets() -> anyhow::Result<Vec<InstalledPet>> {
    list_installed_pets_in_dir(&default_pets_dir())
}

pub async fn install_from_petdex(
    request: PetdexInstallRequest,
) -> anyhow::Result<PetdexInstallResult> {
    let slug = validate_slug(&request.slug)?;
    let pet_json_url = validate_petdex_asset_url(&request.pet_json_url)?;
    let spritesheet_url = validate_petdex_asset_url(&request.spritesheet_url)?;
    let client = crate::http_client::proxied_client(USER_AGENT)?;
    let pet_json = download_limited(&client, pet_json_url, MAX_PET_JSON_BYTES, "pet.json").await?;
    let spritesheet = download_limited(
        &client,
        spritesheet_url,
        MAX_SPRITESHEET_BYTES,
        "spritesheet",
    )
    .await?;
    install_package_from_bytes(
        &default_pets_dir(),
        PetPackageMetadata {
            slug,
            display_name: request.display_name,
            source: "petdex".to_string(),
            pet_json_url: request.pet_json_url,
            spritesheet_url: request.spritesheet_url,
        },
        &pet_json,
        &spritesheet,
        request.overwrite,
    )
}

pub fn install_package_from_bytes(
    pets_dir: &Path,
    metadata: PetPackageMetadata,
    pet_json: &[u8],
    spritesheet: &[u8],
    overwrite: bool,
) -> anyhow::Result<PetdexInstallResult> {
    let slug = validate_slug(&metadata.slug)?;
    if pet_json.len() > MAX_PET_JSON_BYTES {
        bail!("pet.json 超过 {} MB 限制", MAX_PET_JSON_BYTES / 1024 / 1024);
    }
    if spritesheet.len() > MAX_SPRITESHEET_BYTES {
        bail!(
            "spritesheet 超过 {} MB 限制",
            MAX_SPRITESHEET_BYTES / 1024 / 1024
        );
    }
    let mut json_value: Value =
        serde_json::from_slice(pet_json).context("pet.json 不是有效 JSON")?;
    if let Value::Object(ref mut object) = json_value {
        object
            .entry("name")
            .or_insert_with(|| Value::String(metadata.display_name.clone()));
    } else {
        bail!("pet.json 根节点必须是对象");
    }
    let extension = detect_spritesheet_extension(spritesheet)
        .ok_or_else(|| anyhow::anyhow!("spritesheet 必须是 PNG 或 WEBP"))?;

    fs::create_dir_all(pets_dir)
        .with_context(|| format!("创建宠物目录失败：{}", pets_dir.display()))?;
    let target_dir = pets_dir.join(&slug);
    let overwritten = target_dir.exists();
    if overwritten && !overwrite {
        bail!("宠物 {slug} 已安装，请确认覆盖后重试");
    }

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let temp_dir = pets_dir.join(format!(".codex123-install-{slug}-{nonce}"));
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)
            .with_context(|| format!("清理临时目录失败：{}", temp_dir.display()))?;
    }
    fs::create_dir_all(&temp_dir)
        .with_context(|| format!("创建临时目录失败：{}", temp_dir.display()))?;

    let pretty_json = serde_json::to_vec_pretty(&json_value)?;
    fs::write(temp_dir.join("pet.json"), pretty_json)
        .with_context(|| format!("写入 pet.json 失败：{}", temp_dir.display()))?;
    let spritesheet_file = format!("spritesheet.{extension}");
    fs::write(temp_dir.join(&spritesheet_file), spritesheet)
        .with_context(|| format!("写入 spritesheet 失败：{}", temp_dir.display()))?;
    let install_metadata = InstallMetadata {
        source: metadata.source,
        slug: slug.clone(),
        display_name: metadata.display_name,
        pet_json_url: metadata.pet_json_url,
        spritesheet_url: metadata.spritesheet_url,
        installed_at_ms: nonce,
    };
    fs::write(
        temp_dir.join(INSTALL_METADATA_FILE),
        serde_json::to_vec_pretty(&install_metadata)?,
    )
    .with_context(|| format!("写入安装元数据失败：{}", temp_dir.display()))?;

    if overwritten {
        fs::remove_dir_all(&target_dir)
            .with_context(|| format!("覆盖已安装宠物失败：{}", target_dir.display()))?;
    }
    fs::rename(&temp_dir, &target_dir)
        .with_context(|| format!("安装宠物失败：{}", target_dir.display()))?;

    Ok(PetdexInstallResult {
        slug,
        path: target_dir.to_string_lossy().to_string(),
        pet_json_file: "pet.json".to_string(),
        spritesheet_file,
        overwritten,
    })
}

pub fn default_pets_dir() -> PathBuf {
    crate::relay_config::default_codex_home_dir().join("pets")
}

pub fn delete_installed_pet(slug: &str) -> anyhow::Result<InstalledPet> {
    let slug = validate_slug(slug)?;
    let pets_dir = default_pets_dir();
    let target_dir = pets_dir.join(&slug);
    let installed = read_installed_pet_dir(&target_dir, &slug)
        .ok_or_else(|| anyhow::anyhow!("宠物 {slug} 未安装"))?;
    fs::remove_dir_all(&target_dir)
        .with_context(|| format!("删除宠物目录失败：{}", target_dir.display()))?;
    Ok(installed)
}

fn list_installed_pets_in_dir(pets_dir: &Path) -> anyhow::Result<Vec<InstalledPet>> {
    let mut installed = Vec::new();
    let entries = match fs::read_dir(pets_dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(installed),
        Err(error) => {
            return Err(error).with_context(|| format!("读取宠物目录失败：{}", pets_dir.display()));
        }
    };
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(slug) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if slug.starts_with('.') || validate_slug(slug).is_err() {
            continue;
        }
        if let Some(pet) = read_installed_pet_dir(&path, slug) {
            installed.push(pet);
        }
    }
    installed.sort_by(|left, right| left.slug.cmp(&right.slug));
    Ok(installed)
}

fn read_installed_pet_dir(path: &Path, slug: &str) -> Option<InstalledPet> {
    let pet_json = path.join("pet.json");
    if !pet_json.is_file() {
        return None;
    }
    let metadata = read_install_metadata(path);
    let display_name = metadata
        .as_ref()
        .map(|metadata| metadata.display_name.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| {
            fs::read_to_string(&pet_json)
                .ok()
                .and_then(|contents| serde_json::from_str::<Value>(&contents).ok())
                .and_then(|value| {
                    first_string_value(&value, &["displayName", "display_name", "name", "title"])
                })
        })
        .unwrap_or_else(|| slug.to_string());
    let spritesheet_file = ["spritesheet.webp", "spritesheet.png"]
        .iter()
        .find(|file| path.join(file).is_file())
        .map(|file| (*file).to_string());
    Some(InstalledPet {
        slug: slug.to_string(),
        display_name,
        path: path.to_string_lossy().to_string(),
        spritesheet_file,
        source: metadata
            .as_ref()
            .map(|metadata| metadata.source.clone())
            .unwrap_or_default(),
        pet_json_url: metadata
            .as_ref()
            .map(|metadata| metadata.pet_json_url.clone())
            .unwrap_or_default(),
        spritesheet_url: metadata
            .as_ref()
            .map(|metadata| metadata.spritesheet_url.clone())
            .unwrap_or_default(),
        installed_at_ms: metadata.map(|metadata| metadata.installed_at_ms),
    })
}

fn read_install_metadata(path: &Path) -> Option<InstallMetadata> {
    fs::read_to_string(path.join(INSTALL_METADATA_FILE))
        .ok()
        .and_then(|contents| serde_json::from_str::<InstallMetadata>(&contents).ok())
}

fn parse_manifest_pets(value: &Value) -> Vec<PetdexPet> {
    let items = manifest_items(value);
    let mut pets = Vec::new();
    for item in items {
        let Some(slug) = first_string_value(item, &["slug", "id"]) else {
            continue;
        };
        let Ok(slug) = validate_slug(&slug) else {
            continue;
        };
        let Some(pet_json_url) = first_string_value(
            item,
            &[
                "petJsonUrl",
                "pet_json_url",
                "petJson",
                "pet_json",
                "petJsonURL",
            ],
        ) else {
            continue;
        };
        let Some(spritesheet_url) = first_string_value(
            item,
            &[
                "spritesheetUrl",
                "spritesheet_url",
                "spritesheet",
                "spritesheetURL",
            ],
        ) else {
            continue;
        };
        if validate_petdex_asset_url(&pet_json_url).is_err()
            || validate_petdex_asset_url(&spritesheet_url).is_err()
        {
            continue;
        }
        let display_name =
            first_string_value(item, &["displayName", "display_name", "name", "title"])
                .unwrap_or_else(|| slug.clone());
        let description = first_string_value(item, &["description", "summary"]).unwrap_or_default();
        let author = first_string_value(item, &["author", "credit", "creator", "submittedBy"])
            .unwrap_or_default();
        let homepage = first_string_value(item, &["homepage", "url", "pageUrl"])
            .unwrap_or_else(|| format!("https://petdex.crafter.run/pets/{slug}"));
        pets.push(PetdexPet {
            slug,
            display_name,
            description,
            author,
            pet_json_url,
            spritesheet_url,
            homepage,
            tags: collect_tags(item),
            installed: false,
            installed_path: None,
            update_available: false,
            heat_score: 0,
            heat_label: String::new(),
            heat_reason: Vec::new(),
        });
    }
    pets
}

fn apply_pet_heat(pet: &mut PetdexPet) {
    let mut heat_score = 0;
    let mut heat_reason = Vec::new();

    if pet.installed {
        heat_score += 100;
        heat_reason.push("已安装".to_string());
    }
    if pet.update_available {
        heat_score += 40;
        heat_reason.push("可更新".to_string());
    }
    if !pet.author.trim().is_empty() {
        heat_score += 8;
        heat_reason.push("作者信息完整".to_string());
    }
    if !pet.description.trim().is_empty() {
        heat_score += 8;
        heat_reason.push("有描述".to_string());
    }
    if !pet.homepage.trim().is_empty() {
        heat_score += 6;
        heat_reason.push("有主页".to_string());
    }
    let tag_bonus = pet.tags.len().min(5) as u32 * 3;
    if tag_bonus > 0 {
        heat_score += tag_bonus;
        heat_reason.push(format!("标签 {} 个", pet.tags.len().min(5)));
    }

    pet.heat_score = heat_score;
    pet.heat_label = heat_label(heat_score).to_string();
    pet.heat_reason = heat_reason;
}

fn heat_label(heat_score: u32) -> &'static str {
    if heat_score >= 120 {
        "优先推荐"
    } else if heat_score >= 80 {
        "热度较高"
    } else if heat_score >= 40 {
        "值得关注"
    } else {
        "普通"
    }
}

fn sort_pets_by_heat(pets: &mut [PetdexPet]) {
    pets.sort_by(|left, right| {
        right
            .heat_score
            .cmp(&left.heat_score)
            .then_with(|| right.update_available.cmp(&left.update_available))
            .then_with(|| right.installed.cmp(&left.installed))
            .then_with(|| {
                left.display_name
                    .to_lowercase()
                    .cmp(&right.display_name.to_lowercase())
            })
            .then_with(|| left.slug.cmp(&right.slug))
    });
}

fn manifest_items(value: &Value) -> Vec<&Value> {
    if let Value::Array(items) = value {
        return items.iter().collect();
    }
    for key in ["pets", "items", "data", "results"] {
        if let Some(Value::Array(items)) = value.get(key) {
            return items.iter().collect();
        }
    }
    Vec::new()
}

fn first_string_value(value: &Value, keys: &[&str]) -> Option<String> {
    if let Value::Object(object) = value {
        for key in keys {
            if let Some(raw) = object.get(*key) {
                if let Some(value) = raw
                    .as_str()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                {
                    return Some(value.to_string());
                }
            }
        }
        for nested_key in [
            "asset",
            "assets",
            "package",
            "files",
            "metadata",
            "submitter",
        ] {
            if let Some(nested) = object.get(nested_key) {
                if let Some(value) = first_string_value(nested, keys) {
                    return Some(value);
                }
            }
        }
    }
    None
}

fn collect_tags(value: &Value) -> Vec<String> {
    let mut tags = Vec::new();
    collect_string_array(value, "tags", &mut tags);
    collect_string_array(value, "vibes", &mut tags);
    collect_string_array(value, "kinds", &mut tags);
    collect_string_array(value, "kind", &mut tags);
    tags.sort();
    tags.dedup();
    tags
}

fn collect_string_array(value: &Value, key: &str, tags: &mut Vec<String>) {
    let Some(raw) = value.get(key) else {
        return;
    };
    match raw {
        Value::Array(items) => {
            for item in items {
                if let Some(tag) = item.as_str().map(str::trim).filter(|tag| !tag.is_empty()) {
                    tags.push(tag.to_string());
                }
            }
        }
        Value::String(tag) if !tag.trim().is_empty() => tags.push(tag.trim().to_string()),
        _ => {}
    }
}

fn validate_slug(slug: &str) -> anyhow::Result<String> {
    let trimmed = slug.trim();
    if trimmed.is_empty() || trimmed == "." || trimmed == ".." || trimmed.len() > 80 {
        bail!("宠物 slug 无效");
    }
    if trimmed
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.'))
    {
        Ok(trimmed.to_string())
    } else {
        bail!("宠物 slug 只能包含小写字母、数字、点、下划线和连字符")
    }
}

fn validate_petdex_asset_url(url: &str) -> anyhow::Result<Url> {
    let parsed = Url::parse(url.trim()).context("资源 URL 无效")?;
    if parsed.scheme() != "https" {
        bail!("Petdex 资源必须使用 HTTPS");
    }
    let host = parsed.host_str().unwrap_or_default().to_ascii_lowercase();
    let allowed = host == "petdex.crafter.run"
        || host.ends_with(".petdex.crafter.run")
        || host == "ufs.sh"
        || host.ends_with(".ufs.sh")
        || host == "utfs.io"
        || host.ends_with(".utfs.io")
        || host.ends_with(".r2.dev");
    if !allowed || host == "localhost" || host.ends_with(".localhost") {
        bail!("Petdex 资源域名不在允许列表中");
    }
    Ok(parsed)
}

async fn download_limited(
    client: &reqwest::Client,
    url: Url,
    max_bytes: usize,
    label: &str,
) -> anyhow::Result<Vec<u8>> {
    let response = client
        .get(url.clone())
        .send()
        .await
        .with_context(|| format!("下载 {label} 失败"))?
        .error_for_status()
        .with_context(|| format!("下载 {label} 返回非成功状态"))?;
    if let Some(length) = response.content_length() {
        if length > max_bytes as u64 {
            bail!("{label} 超过 {} MB 限制", max_bytes / 1024 / 1024);
        }
    }
    let bytes = response
        .bytes()
        .await
        .with_context(|| format!("读取 {label} 失败"))?;
    if bytes.len() > max_bytes {
        bail!("{label} 超过 {} MB 限制", max_bytes / 1024 / 1024);
    }
    Ok(bytes.to_vec())
}

fn detect_spritesheet_extension(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some("png");
    }
    if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return Some("webp");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const PNG_BYTES: &[u8] = b"\x89PNG\r\n\x1a\nfake";

    #[test]
    fn parse_manifest_accepts_petdex_shape() {
        let value = serde_json::json!({
            "pets": [{
                "slug": "boba",
                "displayName": "Boba",
                "description": "A focused companion",
                "credit": "Crafter",
                "petJsonUrl": "https://yu2vz9gndp.ufs.sh/f/pet-json",
                "spritesheetUrl": "https://yu2vz9gndp.ufs.sh/f/sprite",
                "vibes": ["focused", "cozy"]
            }]
        });
        let pets = parse_manifest_pets(&value);
        assert_eq!(pets.len(), 1);
        assert_eq!(pets[0].slug, "boba");
        assert_eq!(pets[0].display_name, "Boba");
        assert_eq!(pets[0].tags, vec!["cozy", "focused"]);
        assert_eq!(pets[0].heat_score, 0);
    }

    #[test]
    fn apply_pet_heat_prefers_installed_and_update_available() {
        let mut plain = PetdexPet {
            slug: "plain".to_string(),
            display_name: "Plain".to_string(),
            description: String::new(),
            author: String::new(),
            pet_json_url: "https://petdex.crafter.run/pets/plain/pet.json".to_string(),
            spritesheet_url: "https://petdex.crafter.run/pets/plain/spritesheet.webp".to_string(),
            homepage: String::new(),
            tags: Vec::new(),
            installed: false,
            installed_path: None,
            update_available: false,
            heat_score: 0,
            heat_label: String::new(),
            heat_reason: Vec::new(),
        };
        let mut installed = plain.clone();
        installed.installed = true;
        let mut update = installed.clone();
        update.update_available = true;

        apply_pet_heat(&mut plain);
        apply_pet_heat(&mut installed);
        apply_pet_heat(&mut update);

        assert!(installed.heat_score > plain.heat_score);
        assert!(update.heat_score > installed.heat_score);
        assert_eq!(installed.heat_label, "热度较高");
    }

    #[test]
    fn apply_pet_heat_caps_tag_bonus_at_five_tags() {
        let mut pet = PetdexPet {
            slug: "taggy".to_string(),
            display_name: "Taggy".to_string(),
            description: String::new(),
            author: String::new(),
            pet_json_url: "https://petdex.crafter.run/pets/taggy/pet.json".to_string(),
            spritesheet_url: "https://petdex.crafter.run/pets/taggy/spritesheet.webp".to_string(),
            homepage: String::new(),
            tags: vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string(),
                "five".to_string(),
                "six".to_string(),
            ],
            installed: false,
            installed_path: None,
            update_available: false,
            heat_score: 0,
            heat_label: String::new(),
            heat_reason: Vec::new(),
        };

        apply_pet_heat(&mut pet);

        assert_eq!(pet.heat_score, 15);
    }

    #[test]
    fn sort_pets_by_heat_uses_stable_name_and_slug_tiebreakers() {
        let mut pets = vec![
            PetdexPet {
                slug: "zulu".to_string(),
                display_name: "Alpha".to_string(),
                description: String::new(),
                author: String::new(),
                pet_json_url: "https://petdex.crafter.run/pets/zulu/pet.json".to_string(),
                spritesheet_url: "https://petdex.crafter.run/pets/zulu/spritesheet.webp"
                    .to_string(),
                homepage: String::new(),
                tags: Vec::new(),
                installed: false,
                installed_path: None,
                update_available: false,
                heat_score: 40,
                heat_label: "值得关注".to_string(),
                heat_reason: Vec::new(),
            },
            PetdexPet {
                slug: "alpha".to_string(),
                display_name: "Alpha".to_string(),
                description: String::new(),
                author: String::new(),
                pet_json_url: "https://petdex.crafter.run/pets/alpha/pet.json".to_string(),
                spritesheet_url: "https://petdex.crafter.run/pets/alpha/spritesheet.webp"
                    .to_string(),
                homepage: String::new(),
                tags: Vec::new(),
                installed: false,
                installed_path: None,
                update_available: false,
                heat_score: 40,
                heat_label: "值得关注".to_string(),
                heat_reason: Vec::new(),
            },
        ];

        sort_pets_by_heat(&mut pets);

        assert_eq!(pets[0].slug, "alpha");
        assert_eq!(pets[1].slug, "zulu");
    }

    #[test]
    fn install_package_writes_pet_json_and_spritesheet() {
        let temp = tempfile::tempdir().unwrap();
        let result = install_package_from_bytes(
            temp.path(),
            PetPackageMetadata {
                slug: "boba".to_string(),
                display_name: "Boba".to_string(),
                source: "petdex".to_string(),
                pet_json_url: "https://petdex.crafter.run/pets/boba/pet.json".to_string(),
                spritesheet_url: "https://petdex.crafter.run/pets/boba/spritesheet.webp"
                    .to_string(),
            },
            br#"{"states":{"idle":{}}}"#,
            PNG_BYTES,
            false,
        )
        .unwrap();
        let target = temp.path().join("boba");
        assert_eq!(result.slug, "boba");
        assert_eq!(result.spritesheet_file, "spritesheet.png");
        assert!(target.join("pet.json").is_file());
        assert!(target.join("spritesheet.png").is_file());
        assert!(target.join(INSTALL_METADATA_FILE).is_file());
    }

    #[test]
    fn install_package_rejects_path_traversal_slug() {
        let temp = tempfile::tempdir().unwrap();
        let error = install_package_from_bytes(
            temp.path(),
            PetPackageMetadata {
                slug: "../bad".to_string(),
                display_name: String::new(),
                source: "petdex".to_string(),
                pet_json_url: String::new(),
                spritesheet_url: String::new(),
            },
            br#"{}"#,
            PNG_BYTES,
            false,
        )
        .unwrap_err();
        assert!(error.to_string().contains("slug"));
    }

    #[test]
    fn install_package_requires_overwrite_confirmation() {
        let temp = tempfile::tempdir().unwrap();
        let metadata = PetPackageMetadata {
            slug: "boba".to_string(),
            display_name: "Boba".to_string(),
            source: "petdex".to_string(),
            pet_json_url: "https://petdex.crafter.run/pets/boba/pet.json".to_string(),
            spritesheet_url: "https://petdex.crafter.run/pets/boba/spritesheet.webp".to_string(),
        };
        install_package_from_bytes(temp.path(), metadata.clone(), br#"{}"#, PNG_BYTES, false)
            .unwrap();
        let error = install_package_from_bytes(temp.path(), metadata, br#"{}"#, PNG_BYTES, false)
            .unwrap_err();
        assert!(error.to_string().contains("已安装"));
    }

    #[test]
    fn install_package_rejects_non_image_spritesheet() {
        let temp = tempfile::tempdir().unwrap();
        let error = install_package_from_bytes(
            temp.path(),
            PetPackageMetadata {
                slug: "boba".to_string(),
                display_name: "Boba".to_string(),
                source: "petdex".to_string(),
                pet_json_url: String::new(),
                spritesheet_url: String::new(),
            },
            br#"{}"#,
            b"not an image",
            false,
        )
        .unwrap_err();
        assert!(error.to_string().contains("PNG 或 WEBP"));
    }

    #[test]
    fn delete_installed_pet_removes_only_valid_slug_dir() {
        let temp = tempfile::tempdir().unwrap();
        let result = install_package_from_bytes(
            temp.path(),
            PetPackageMetadata {
                slug: "boba".to_string(),
                display_name: "Boba".to_string(),
                source: "petdex".to_string(),
                pet_json_url: "https://petdex.crafter.run/pets/boba/pet.json".to_string(),
                spritesheet_url: "https://petdex.crafter.run/pets/boba/spritesheet.webp"
                    .to_string(),
            },
            br#"{"states":{"idle":{}}}"#,
            PNG_BYTES,
            false,
        )
        .unwrap();
        assert!(Path::new(&result.path).exists());
        let installed = read_installed_pet_dir(Path::new(&result.path), "boba").unwrap();
        fs::remove_dir_all(&installed.path).unwrap();
        assert!(!Path::new(&installed.path).exists());
    }
}
