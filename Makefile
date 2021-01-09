docker-build:
	docker build -t lol:dev --build-arg USER=${USER} --build-arg UID=`id -u` - < Dockerfile.dev
	docker build -t lol:cov --build-arg USER=${USER} --build-arg UID=`id -u` - < Dockerfile.cov

.PHONY: install
install:
	cargo install --path kvs
	cargo install --path lol-admin
	cargo install --path lol-monitor

test: install
	cargo test -- --test-threads=1

bench: install
	cargo +nightly bench

.PHONY: cov
cov:
	cargo clean
	RUSTFLAGS="-Zinstrument-coverage" cargo +nightly install --debug --path kvs
	cargo install --path lol-admin
	cargo install --path lol-monitor
	cargo test -- --test-threads=1