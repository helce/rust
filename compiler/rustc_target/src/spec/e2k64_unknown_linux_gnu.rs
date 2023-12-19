use crate::spec::{LinkArgs, LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::linux_gnu_base::opts();
    base.max_atomic_width = Some(64);
    base.pre_link_args.entry(LinkerFlavor::Gcc).or_default().push("-fPIC".into());
    let mut post_args = LinkArgs::new();
    post_args.insert(
        LinkerFlavor::Gcc,
        vec![
            "-L/usr/lib/lccrt/lib/e2k64/".into(),
            "-Wl,-rpath=/usr/lib/lccrt/lib/e2k64/".into(),
            "-llccrt_s".into(),
            "-llcc".into(),
            "-lm".into(),
        ],
    );
    Target {
        llvm_target: "e2k64-unknown-linux-gnu".into(),
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64:64-f80:128:128-n32:64-S128".into(),
        arch: "e2k64".into(),

        options: TargetOptions {
            linker: Some(option_env!("ECC").unwrap_or("/usr/bin/gcc").into()),
            position_independent_executables: false,
            post_link_args: post_args,
            mcount: "\u{1}_mcount".into(),
            ..base
        },
    }
}
