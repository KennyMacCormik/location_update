use config::Config;
use override_key_derive::ApplyOverrides;
use override_key_core::ApplyOverrides;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "list")]
struct ComplexOption {
    servers: Option<Vec<String>>,
}

#[test]
fn test_complex_option_type() {
    let args = ComplexOption {
        servers: Some(vec!["a".into(), "b".into()]),
    };

    let cfg = args.apply_overrides(Config::builder()).unwrap().build().unwrap();
    let list: Vec<String> = cfg.get_array("list.servers").unwrap()
        .into_iter()
        .map(|v| v.into_string().unwrap())
        .collect();

    assert_eq!(list, vec!["a", "b"]);
}