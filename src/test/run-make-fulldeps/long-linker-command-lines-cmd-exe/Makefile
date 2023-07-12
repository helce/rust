-include ../tools.mk

all:
	$(RUSTC) foo.rs
	cp foo.bat $(TMPDIR)/
	OUT_DIR="$(TMPDIR)" RUSTC="$(RUSTC_ORIGINAL)" $(call RUN,foo)
