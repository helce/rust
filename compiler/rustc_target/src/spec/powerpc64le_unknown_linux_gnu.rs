use crate::spec::{LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::linux_gnu_base::opts();
    base.cpu = "ppc64le".into();
    base.add_pre_link_args(LinkerFlavor::Gcc, &["-m64"]);
    base.max_atomic_width = Some(64);

    Target {
        llvm_target: "powerpc64le-unknown-linux-gnu".into(),
        pointer_width: 64,
        data_layout: "e-m:e-i64:64-n32:64-S128-v256:256:256-v512:512:512".into(),
        arch: "powerpc64".into(),
        options: TargetOptions { mcount: "_mcount".into(), ..base },
    }
}
