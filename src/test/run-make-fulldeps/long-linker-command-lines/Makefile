-include ../tools.mk

# ignore-e2k64 incorrect env while running compiler

all:
	$(RUSTC) foo.rs -O
	RUSTC="$(RUSTC_ORIGINAL)" $(call RUN,foo)
