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
fn test_basic_overrides() {
    let args = CLIArgs {
        iproyal_endpoint: Some("https://api.iproyal.local".into()),
        netnut_endpoint: Some("https://api.netnut.local".into()),
        netnut_token: Some("abc123".into()),
        iproyal_timeout: Some("30s".into()),
        region_id: Some(42),
        endpoint2: Some("https://api.iproyal2.local".into()),
    };

    let builder = Config::builder();
    let builder = args.apply_overrides(builder).unwrap();
    let cfg = builder.build().unwrap();

    assert_eq!(cfg.get_string("iproyal.endpoint").unwrap(), "https://api.iproyal.local");
    assert_eq!(cfg.get_string("netnut.endpoint").unwrap(), "https://api.netnut.local");
    assert_eq!(cfg.get_string("netnut.netnut.token").unwrap(), "abc123");
    assert_eq!(cfg.get_string("iproyal.iproyal.timeout").unwrap(), "30s");
    assert_eq!(cfg.get_int("iproyal.region.id").unwrap(), 42);
    assert_eq!(cfg.get_string("iproyal.endpoint2").unwrap(), "https://api.iproyal2.local");
}


