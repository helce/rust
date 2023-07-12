use crate::spec::{LinkArgs, Target, TargetOptions, LinkerFlavor};

pub fn target() -> Target {
    let mut base = super::linux_gnu_base::opts();
    base.max_atomic_width = Some(64);
    base.pre_link_args
        .entry(LinkerFlavor::Gcc)
        .or_default()
        .push("-fPIC".to_string());
    let mut post_args = LinkArgs::new();
    post_args.insert(
        LinkerFlavor::Gcc,
        vec![
            "-L/usr/lib/lccrt/lib/e2k64/".to_string(),
            "-Wl,-rpath=/usr/lib/lccrt/lib/e2k64/".to_string(),
            "-llccrt_s".to_string(),
            "-llcc".to_string(),
            "-lm".to_string(),
        ],
    );
    Target {
        llvm_target: "e2k64-unknown-linux-gnu".to_string(),
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64:64-f80:128:128-n32:64-S128".to_string(),
        arch: "e2k64".to_string(),

        options: TargetOptions {
            linker: Some(option_env!("ECC").unwrap_or("/usr/bin/gcc").to_string()),
            position_independent_executables: false,
            post_link_args: post_args,
            mcount: "\u{1}_mcount".to_string(),
            ..base
        },
    }
}
