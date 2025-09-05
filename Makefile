.PHONY: core cli server all clean

BUILD ?= debug

ifeq ($(BUILD),release)
	CARGO_FLAGS := --release
else
	CARGO_FLAGS :=
endif

all: core cli server

clean:
	rm -rf core/target cli/target server/target

core:
	cargo build $(CARGO_FLAGS) --manifest-path core/Cargo.toml

cli:
	cargo build $(CARGO_FLAGS) --manifest-path cli/Cargo.toml

server:
	cargo build $(CARGO_FLAGS) --manifest-path server/Cargo.toml
