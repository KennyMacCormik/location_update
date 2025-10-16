use override_key_core::ApplyOverrides;
use override_key_derive::ApplyOverrides;
use config::Config;

#[derive(ApplyOverrides)]
struct NonOptionField {
    #[override_key = "system.version"]
    version: String,
}

#[test]
fn non_option_field_always_overrides() {
    let args = NonOptionField {
        version: "1.2.3".into(),
    };

    let cfg = args
        .apply_overrides(Config::builder())
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(cfg.get_string("system.version").unwrap(), "1.2.3");
}