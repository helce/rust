use crate::spec::{cvs, Target};

pub fn target() -> Target {
    let mut base = super::e2k_unknown_linux_gnu::target();
    base.cpu = "elbrus-v6".into();
    base.llvm_args = cvs!["-enable-elbrus-fma-legalization=0"];
    base
}
