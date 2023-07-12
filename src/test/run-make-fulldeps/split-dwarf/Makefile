-include ../tools.mk

# ignore-e2k64 NIY
# only-linux
# min-llvm-version: 11.0

all:
	$(RUSTC) -Z unstable-options -C split-debuginfo=packed -C debuginfo=2 foo.rs -g
	rm $(TMPDIR)/foo.dwp
	rm $(TMPDIR)/$(call BIN,foo)
