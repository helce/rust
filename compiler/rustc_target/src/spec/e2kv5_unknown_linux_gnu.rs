use crate::spec::Target;

pub fn target() -> Target {
    let mut base = super::e2k64_unknown_linux_gnu::target();
    base.options.cpu = "elbrus-v5".into();
    base
}
