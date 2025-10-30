mod happy {
    mod complex_option_type;
    mod default_none_behavior;
    mod derive_macro_basic;
    mod double_option;
    mod empty_option_fields_are_skipped;
    mod empty_prefix_does_not_create_leading_dot;
    mod empty_prefix_no_dot;
    mod explicit_keys_are_applied_verbatim;
    mod field_level_prefix_overrides_struct_prefix;
    mod mixed_option_and_non_option;
    mod mixed_option_non_option_fields_override_correctly;
    mod non_option_field_always_overrides;
    mod skips_none_fields;
    mod struct_level_infer_with_prefix_applies_to_all_fields;
    mod underscores_are_replaced_with_dots;
    mod various_option_types;
}
