use config::Config;
use override_key_derive::ApplyOverrides;
use override_key_core::ApplyOverrides;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "opt")]
struct OptionArgs {
    bool_flag: Option<bool>,
    retries: Option<u32>,
    api_url: Option<String>,
}

#[test]
fn test_various_option_types() {
    let args = OptionArgs {
        bool_flag: Some(true),
        retries: Some(5),
        api_url: Some("https://example.org".into()),
    };

    let cfg = args.apply_overrides(Config::builder()).unwrap().build().unwrap();

    assert_eq!(cfg.get_bool("opt.bool.flag").unwrap(), true);
    assert_eq!(cfg.get_int("opt.retries").unwrap(), 5);
    assert_eq!(cfg.get_string("opt.api.url").unwrap(), "https://example.org");
}