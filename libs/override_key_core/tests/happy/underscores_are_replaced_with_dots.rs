use override_key_core::ApplyOverrides;
use override_key_derive::ApplyOverrides;
use config::Config;

#[derive(ApplyOverrides)]
#[apply_overrides(infer_keys, prefix = "deep")]
struct NestedKeys {
    database_connection_url: Option<String>,
    cache_ttl_seconds: Option<u64>,
}

#[test]
fn underscores_are_replaced_with_dots() {
    let args = NestedKeys {
        database_connection_url: Some("postgres://".into()),
        cache_ttl_seconds: Some(90),
    };
    let cfg = args
        .apply_overrides(Config::builder())
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(
        cfg.get_string("deep.database.connection.url").unwrap(),
        "postgres://"
    );
    assert_eq!(cfg.get_int("deep.cache.ttl.seconds").unwrap(), 90);
}