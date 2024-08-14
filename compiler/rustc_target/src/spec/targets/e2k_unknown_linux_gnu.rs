use crate::spec::{base, Cc, LinkerFlavor, Lld, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "e2k-unknown-linux-gnu".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64-f64:64-f80:128:128-n32:64-S128".into(),
        arch: "e2k".into(),
        options: TargetOptions {
            max_atomic_width: Some(64),
            mcount: "_mcount".into(),
            post_link_args: TargetOptions::link_args(
                LinkerFlavor::Gnu(Cc::Yes, Lld::No),
                &["-llcc"],
            ),
            ..base::linux_gnu::opts()
        },
    }
}
