-include ../tools.mk

# only-linux

all:
	$(RUSTC) foo.rs
	$(CC) foo.c -lfoo -L $(TMPDIR) -Wl,--gc-sections $(EXTRACFLAGS) -o $(TMPDIR)/foo
	$(call RUN,foo)
	$(CC) foo.c -lfoo -L $(TMPDIR) -Wl,--gc-sections $(EXTRACFLAGS) -pie -fPIC -o $(TMPDIR)/foo
	$(call RUN,foo)
