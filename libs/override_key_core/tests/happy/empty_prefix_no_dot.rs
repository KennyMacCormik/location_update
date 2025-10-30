use config::Config;
use override_key_derive::ApplyOverrides;
use override_key_core::ApplyOverrides;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys)]
struct EmptyPrefix {
    #[override_key(infer, prefix = "")]
    timeout: Option<String>,
}

#[test]
fn test_empty_prefix_no_dot() {
    let args = EmptyPrefix { timeout: Some("20s".into()) };
    let cfg = args.apply_overrides(Config::builder()).unwrap().build().unwrap();
    assert!(cfg.get::<String>("timeout").is_ok());
    assert!(cfg.get::<String>(".timeout").is_err());
}