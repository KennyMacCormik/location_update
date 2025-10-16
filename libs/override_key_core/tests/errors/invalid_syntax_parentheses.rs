use override_key_derive::ApplyOverrides;

#[derive(ApplyOverrides)]
struct InvalidSyntax {
    // parentheses form not allowed
    #[override_key("iproyal.endpoint")]
    endpoint: Option<String>,
}

fn main() {}