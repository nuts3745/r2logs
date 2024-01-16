# Cargoでテストとビルドを行うためのMakefile
# 以下のコマンドを実行することで、テストとビルドを行うことができる。

PHONY: test
test:
	make fmt;
	cargo test;

PHONY: install
install:
	make fmt;
	cargo install --path .;

PHONY: build
build:
	make fmt;
	cargo build --release;

PHONY: clean
clean:
	cargo clean;

PHONY: release
release:
	make fmt;
	cargo build --release;

PHONY: run
run:
	make fmt;
	make install;
	r2logs;

PHONY: fmt
fmt:
	cargo clippy;
	cargo fmt;