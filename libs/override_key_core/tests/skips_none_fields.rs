use override_key_core::ApplyOverrides;
use override_key_derive::ApplyOverrides;
use config::Config;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "iproyal")]
struct CLIArgs {
    #[override_key = "iproyal.endpoint"]
    iproyal_endpoint: Option<String>,

    #[override_key = "netnut.endpoint"]
    netnut_endpoint: Option<String>,

    #[override_key(infer, prefix = "netnut")]
    netnut_token: Option<String>,

    iproyal_timeout: Option<String>,
    region_id: Option<u32>,
    endpoint2: Option<String>,
}

#[test]
fn test_skips_none_fields() {
    let args = CLIArgs {
        iproyal_endpoint: Some("https://api.iproyal.local".into()),
        netnut_endpoint: None,
        netnut_token: None,
        iproyal_timeout: None,
        region_id: Some(1),
        endpoint2: None,
    };

    let cfg = args.apply_overrides(Config::builder()).unwrap().build().unwrap();

    // none of these should exist
    for key in [
        "netnut.endpoint",
        "netnut.netnut.token",
        "iproyal.iproyal.timeout",
        "iproyal.endpoint2",
    ] {
        assert!(cfg.get::<String>(key).is_err(), "key `{key}` unexpectedly present");
    }
}