use config::Config;
use override_key_derive::ApplyOverrides;
use override_key_core::ApplyOverrides;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "nested")]
struct NestedOptionArgs {
    #[override_key = "nested.value"]
    deep_option: Option<Option<String>>,
}

#[test]
fn test_double_option() {
    // Inner Some
    let args = NestedOptionArgs { deep_option: Some(Some("inner".into())) };
    let cfg = args.apply_overrides(Config::builder()).unwrap().build().unwrap();
    assert_eq!(cfg.get_string("nested.value").unwrap(), "inner");

    // Inner None
    let args = NestedOptionArgs { deep_option: Some(None) };
    let cfg = args.apply_overrides(Config::builder()).unwrap().build().unwrap();
    assert!(cfg.get::<String>("nested.value").is_err());
}