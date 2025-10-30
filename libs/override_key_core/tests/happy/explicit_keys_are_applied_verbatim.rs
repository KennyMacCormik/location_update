use override_key_core::ApplyOverrides;
use override_key_derive::ApplyOverrides;
use config::Config;

#[derive(ApplyOverrides)]
struct ExplicitArgs {
    #[override_key = "custom.endpoint"]
    endpoint: Option<String>,

    #[override_key = "custom.timeout"]
    timeout: Option<String>,
}

#[test]
fn explicit_keys_are_applied_verbatim() {
    let args = ExplicitArgs {
        endpoint: Some("https://api.local".into()),
        timeout: Some("15s".into()),
    };

    let cfg = args
        .apply_overrides(Config::builder())
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(cfg.get_string("custom.endpoint").unwrap(), "https://api.local");
    assert_eq!(cfg.get_string("custom.timeout").unwrap(), "15s");
}