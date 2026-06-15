use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut base = super::e2k_unknown_linux_gnu::target();
    base.cpu = "elbrus-8c2".into();
    base.metadata = crate::spec::TargetMetadata {
        description: Some("64-bit Linux (kernel 5.10+, glibc 2.35+)".into()),
        tier: Some(2),
        host_tools: Some(true),
        std: Some(true),
    };
    base
}
