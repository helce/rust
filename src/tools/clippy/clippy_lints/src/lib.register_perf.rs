// This file was generated by `cargo dev update_lints`.
// Use that command to update this file and do not edit by hand.
// Manual edits will be overwritten.

store.register_group(true, "clippy::perf", Some("clippy_perf"), vec![
    LintId::of(box_default::BOX_DEFAULT),
    LintId::of(entry::MAP_ENTRY),
    LintId::of(escape::BOXED_LOCAL),
    LintId::of(format_args::FORMAT_IN_FORMAT_ARGS),
    LintId::of(format_args::TO_STRING_IN_FORMAT_ARGS),
    LintId::of(functions::RESULT_LARGE_ERR),
    LintId::of(large_const_arrays::LARGE_CONST_ARRAYS),
    LintId::of(large_enum_variant::LARGE_ENUM_VARIANT),
    LintId::of(loops::MANUAL_MEMCPY),
    LintId::of(loops::MISSING_SPIN_LOOP),
    LintId::of(loops::NEEDLESS_COLLECT),
    LintId::of(manual_retain::MANUAL_RETAIN),
    LintId::of(methods::COLLAPSIBLE_STR_REPLACE),
    LintId::of(methods::EXPECT_FUN_CALL),
    LintId::of(methods::EXTEND_WITH_DRAIN),
    LintId::of(methods::ITER_NTH),
    LintId::of(methods::ITER_OVEREAGER_CLONED),
    LintId::of(methods::MANUAL_STR_REPEAT),
    LintId::of(methods::OR_FUN_CALL),
    LintId::of(methods::SINGLE_CHAR_PATTERN),
    LintId::of(methods::UNNECESSARY_TO_OWNED),
    LintId::of(operators::CMP_OWNED),
    LintId::of(redundant_clone::REDUNDANT_CLONE),
    LintId::of(slow_vector_initialization::SLOW_VECTOR_INITIALIZATION),
    LintId::of(types::BOX_COLLECTION),
    LintId::of(types::REDUNDANT_ALLOCATION),
    LintId::of(vec::USELESS_VEC),
    LintId::of(vec_init_then_push::VEC_INIT_THEN_PUSH),
])
