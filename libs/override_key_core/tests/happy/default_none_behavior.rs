use config::Config;
use override_key_derive::ApplyOverrides;
use override_key_core::ApplyOverrides;

#[derive(Default, ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "def")]
struct DefaultArgs {
    token: Option<String>,
}

#[test]
fn test_default_none_behavior() {
    let args = DefaultArgs::default();
    let cfg = args.apply_overrides(Config::builder()).unwrap().build().unwrap();
    assert!(cfg.get::<String>("def.token").is_err());
}