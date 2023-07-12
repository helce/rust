use crate::spec::Target;

pub fn target() -> Target {
    let mut base = super::e2k64_unknown_linux_gnu::target();
    base.options.cpu = "elbrus-v6".to_string();
    base
}
