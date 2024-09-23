use crate::spec::Target;

pub fn target() -> Target {
    let mut base = super::e2k_unknown_linux_gnu::target();
    base.cpu = "elbrus-48c".into();
    base
}
