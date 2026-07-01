use codex_plus_core::settings::BackendSettings;

#[test]
fn paste_fix_defaults_to_false() {
    let settings = BackendSettings::default();
    assert!(!settings.codex_app_paste_fix);
    assert!(!settings.codex_app_force_chinese_locale);
    assert!(settings.codex_app_plugin_auto_expand);

    let json = serde_json::to_value(&settings).expect("serialize default settings");
    assert_eq!(
        json.get("codexAppPasteFix")
            .and_then(|value| value.as_bool()),
        Some(false),
        "default BackendSettings JSON should include codexAppPasteFix = false"
    );
    assert_eq!(
        json.get("codexAppForceChineseLocale")
            .and_then(|value| value.as_bool()),
        Some(false),
        "default BackendSettings JSON should include codexAppForceChineseLocale = false"
    );
    assert_eq!(
        json.get("codexAppPluginAutoExpand")
            .and_then(|value| value.as_bool()),
        Some(true),
        "default BackendSettings JSON should include codexAppPluginAutoExpand = true"
    );
}

#[test]
fn enhancement_toggles_round_trip_through_json() {
    let mut settings = BackendSettings::default();
    settings.codex_app_paste_fix = true;
    settings.codex_app_force_chinese_locale = true;
    settings.codex_app_plugin_auto_expand = false;

    let json = serde_json::to_value(&settings).expect("serialize");
    assert_eq!(
        json.get("codexAppPasteFix")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
    assert_eq!(
        json.get("codexAppForceChineseLocale")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
    assert_eq!(
        json.get("codexAppPluginAutoExpand")
            .and_then(|value| value.as_bool()),
        Some(false)
    );

    let parsed: BackendSettings =
        serde_json::from_value(json).expect("deserialize enhancement toggles");
    assert!(parsed.codex_app_paste_fix);
    assert!(parsed.codex_app_force_chinese_locale);
    assert!(!parsed.codex_app_plugin_auto_expand);
}

#[test]
fn paste_fix_missing_from_old_json_defaults_to_false() {
    let json = serde_json::json!({
        "codexAppPath": "",
        "enhancementsEnabled": true
    });

    let parsed: BackendSettings = serde_json::from_value(json)
        .expect("old settings JSON without codexAppPasteFix should still load");
    assert!(!parsed.codex_app_paste_fix);
    assert!(!parsed.codex_app_force_chinese_locale);
    assert!(parsed.codex_app_plugin_auto_expand);
}
