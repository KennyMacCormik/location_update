use override_key_core::ApplyOverrides;
use override_key_derive::ApplyOverrides;
use config::Config;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys)]
struct EmptyPrefix {
    #[override_key(infer, prefix = "")]
    timeout: Option<String>,
}

#[test]
fn empty_prefix_does_not_create_leading_dot() {
    let args = EmptyPrefix {
        timeout: Some("10s".into()),
    };
    let cfg = args
        .apply_overrides(Config::builder())
        .unwrap()
        .build()
        .unwrap();

    assert!(cfg.get::<String>("timeout").is_ok());
    assert!(cfg.get::<String>(".timeout").is_err());
}