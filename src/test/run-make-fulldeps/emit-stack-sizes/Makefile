include ../tools.mk

# ignore-windows
# ignore-macos
# ignore-e2k64 stack-sizes not implemented in e2k
#
# This feature only works when the output object format is ELF so we ignore
# macOS and Windows

# check that the .stack_sizes section is generated
all:
	$(RUSTC) -C opt-level=3 -Z emit-stack-sizes --emit=obj foo.rs
	size -A $(TMPDIR)/foo.o | $(CGREP) .stack_sizes
