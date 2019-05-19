
Rust_Enclave_Name := libenclave.a
Rust_Enclave_Files := $(wildcard src/*.rs)

Rust_target_dir ?= debug

ifeq ($(BACKTRACE), 1)
	Rust_features += " backtrace"
else
	Rust_features := ""
endif

.PHONY: all

all: $(Rust_Enclave_Name)

$(Rust_Enclave_Name): $(Rust_Enclave_Files)
ifeq ($(XARGO_SGX), 1)
	RUST_TARGET_PATH=$(shell pwd) xargo build --target x86_64-unknown-linux-sgx $(CARGO_FLAGS)
	cp ./target/x86_64-unknown-linux-sgx/$(Rust_target_dir)/libenigmacoreenclave.a ../lib/libenclave.a
else
	cargo build $(CARGO_FLAGS) --features $(Rust_features)
	cp ./target/$(Rust_target_dir)/libenigmacoreenclave.a ../lib/libenclave.a
endif