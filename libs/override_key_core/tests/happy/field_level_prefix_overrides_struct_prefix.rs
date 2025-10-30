use override_key_core::ApplyOverrides;
use override_key_derive::ApplyOverrides;
use config::Config;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "iproyal")]
struct MixedArgs {
    #[override_key(infer, prefix = "netnut")]
    netnut_token: Option<String>,

    iproyal_timeout: Option<String>,
    region_id: Option<u32>,
}

#[test]
fn field_level_prefix_overrides_struct_prefix() {
    let args = MixedArgs {
        netnut_token: Some("NN123".into()),
        iproyal_timeout: Some("20s".into()),
        region_id: Some(44),
    };

    let cfg = args
        .apply_overrides(Config::builder())
        .unwrap()
        .build()
        .unwrap();

    // netnut_ → "netnut.netnut.token"
    assert_eq!(cfg.get_string("netnut.netnut.token").unwrap(), "NN123");
    // iproyal_timeout → "iproyal.iproyal.timeout"
    assert_eq!(cfg.get_string("iproyal.iproyal.timeout").unwrap(), "20s");
    // region_id → "iproyal.region.id"
    assert_eq!(cfg.get_int("iproyal.region.id").unwrap(), 44);
}