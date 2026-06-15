use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut base = super::e2k_unknown_linux_gnu::target();
    base.cpu = "elbrus-4c".into();
    base
}
