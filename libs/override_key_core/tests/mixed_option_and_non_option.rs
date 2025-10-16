use config::Config;
use override_key_derive::ApplyOverrides;
use override_key_core::ApplyOverrides;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "mix")]
struct MixedArgs {
    retries: u32,
    timeout: Option<String>,
}

#[test]
fn test_mixed_option_and_non_option() {
    let args = MixedArgs { retries: 3, timeout: Some("5s".into()) };

    let cfg = args.apply_overrides(Config::builder()).unwrap().build().unwrap();

    assert_eq!(cfg.get_int("mix.retries").unwrap(), 3);
    assert_eq!(cfg.get_string("mix.timeout").unwrap(), "5s");
}