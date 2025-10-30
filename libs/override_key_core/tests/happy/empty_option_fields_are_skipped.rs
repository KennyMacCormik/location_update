use override_key_core::ApplyOverrides;
use override_key_derive::ApplyOverrides;
use config::Config;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "skip")]
struct SkipEmpty {
    name: Option<String>,
    value: Option<String>,
}

#[test]
fn empty_option_fields_are_skipped() {
    let args = SkipEmpty {
        name: Some("A".into()),
        value: None,
    };
    let cfg = args
        .apply_overrides(Config::builder())
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(cfg.get_string("skip.name").unwrap(), "A");
    // should not exist
    assert!(cfg.get::<String>("skip.value").is_err());
}