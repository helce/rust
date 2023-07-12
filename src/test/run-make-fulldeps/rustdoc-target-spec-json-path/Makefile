include ../tools.mk

# ignore-e2k64
# Test that rustdoc will properly canonicalize the target spec json path just like rustc

OUTPUT_DIR := "$(TMPDIR)/rustdoc-target-spec-json-path"

all:
	$(RUSTC) --crate-type lib dummy_core.rs --target target.json
	$(RUSTDOC) -o $(OUTPUT_DIR) -L $(TMPDIR) my_crate.rs --target target.json
