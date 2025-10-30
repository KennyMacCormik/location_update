use override_key_derive::ApplyOverrides;

#[derive(ApplyOverrides)]
#[apply_overrides(bad_token, prefixx = "oops")]
struct InvalidMeta {
    #[override_key = "key.value"]
    field: Option<String>,
}

fn main() {}