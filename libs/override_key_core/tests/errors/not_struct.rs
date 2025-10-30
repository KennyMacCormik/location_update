use override_key_derive::ApplyOverrides;

// Not a struct at all
#[derive(ApplyOverrides)]
enum BadEnum {
    A,
    B,
}

fn main() {}