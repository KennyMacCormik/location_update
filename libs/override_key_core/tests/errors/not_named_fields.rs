use override_key_derive::ApplyOverrides;

// Tuple struct, no field names
#[derive(ApplyOverrides)]
struct TupleArgs(String, String);

fn main() {}