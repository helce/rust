use crate::spec::Target;

pub fn target() -> Target {
    let mut base = super::e2k_unknown_linux_gnu::target();
    base.cpu = "elbrus-8v7".into();
    base
}
