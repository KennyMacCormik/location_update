use override_key_core::ApplyOverrides;
use override_key_derive::ApplyOverrides;
use config::Config;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "iproyal")]
struct InferArgs {
    endpoint: Option<String>,
    token: Option<String>,
    timeout_seconds: Option<u32>,
}

#[test]
fn struct_level_infer_with_prefix_applies_to_all_fields() {
    let args = InferArgs {
        endpoint: Some("https://api.iproyal.local".into()),
        token: Some("abc".into()),
        timeout_seconds: Some(30),
    };

    let cfg = args
        .apply_overrides(Config::builder())
        .unwrap()
        .build()
        .unwrap();

    // underscores become dots, prefix prepended
    assert_eq!(
        cfg.get_string("iproyal.endpoint").unwrap(),
        "https://api.iproyal.local"
    );
    assert_eq!(cfg.get_string("iproyal.token").unwrap(), "abc");
    assert_eq!(cfg.get_int("iproyal.timeout.seconds").unwrap(), 30);
}