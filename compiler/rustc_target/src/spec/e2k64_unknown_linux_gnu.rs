use crate::spec::{Cc, LinkerFlavor, Lld, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::linux_gnu_base::opts();
    base.max_atomic_width = Some(64);
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-fPIC"]);
    base.add_post_link_args(
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        &[
            "-L/usr/lib/lccrt/lib/e2k64/",
            "-Wl,-rpath=/usr/lib/lccrt/lib/e2k64/",
            "-llccrt_s",
            "-llcc",
            "-lm",
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
            mcount: "\u{1}_mcount".into(),
            ..base
        },
    }
}
