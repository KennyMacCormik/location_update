use override_key_core::ApplyOverrides;
use override_key_derive::ApplyOverrides;
use config::Config;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "mix")]
struct MixedOptionNonOption {
    id: u32,
    name: Option<String>,
}

#[test]
fn mixed_option_non_option_fields_override_correctly() {
    let args = MixedOptionNonOption {
        id: 101,
        name: Some("test".into()),
    };

    let cfg = args
        .apply_overrides(Config::builder())
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(cfg.get_int("mix.id").unwrap(), 101);
    assert_eq!(cfg.get_string("mix.name").unwrap(), "test");
}