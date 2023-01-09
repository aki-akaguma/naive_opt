BENCH_STR = --bench=bench-ss-algo --bench=bench-ss-algo-indices
#BENCH_STR = --bench=bench-ss-algo-indices

TARGET_GNU  = --target=x86_64-unknown-linux-gnu
TARGET_MUSL = --target=x86_64-unknown-linux-musl
TSK = taskset -c 2

all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test

test-no_std:
	cargo test --no-default-features

clean:
	@cargo clean
	@rm -f z.*

test-gnu:
	cargo test $(TARGET_GNU)

test-musl:
	cargo test $(TARGET_MUSL)

bench-all: bench-gnu bench-musl

bench-build-all: bench-build-gnu bench-build-musl


bench-gnu: bench.en.1-gnu bench.ja.1-gnu

bench-musl: bench.en.1-musl bench.ja.1-musl

bench-build-gnu: target/stamp.bench-build-gnu

target/stamp.bench-build-gnu:
	cargo xbench --no-run $(TARGET_GNU)
	@touch target/stamp.bench-build-gnu

bench-build-musl: target/stamp.bench-build-musl

target/stamp.bench-build-musl:
	cargo xbench --no-run $(TARGET_MUSL)
	@touch target/stamp.bench-build-musl

bench-clean:
	@rm -fr target/criterion

report:
	cargo xtask shape_benchmark_results


bench.en.1-gnu: target/stamp.bench-build-gnu
	@rm -f z.gnu.bench.en.1.log
	env AKI_TEST_DAT=en.1 $(TSK) cargo xbench $(BENCH_STR) $(TARGET_GNU) -- -n | tee -a z.gnu.bench.en.1.log

bench.ja.1-gnu: target/stamp.bench-build-gnu
	@rm -f z.gnu.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 $(TSK) cargo xbench $(BENCH_STR) $(TARGET_GNU) -- -n | tee -a z.gnu.bench.ja.1.log

bench.en.1-musl: target/stamp.bench-build-musl
	@rm -f z.musl.bench.en.1.log
	env AKI_TEST_DAT=en.1 $(TSK) cargo xbench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.en.1.log

bench.ja.1-musl: target/stamp.bench-build-musl
	@rm -f z.musl.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 $(TSK) cargo xbench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.ja.1.log

stochastics:
	(find ~/src/rust/rfcs/text/ -type f -name "*.md" -exec aki-xcat \{\} \; ; find ~/src/data/ietf-rfc/text/ -type f -name "*.txt.gz" -exec aki-xcat \{\} \; ) | aki-stats --map-ascii -X map-ascii-rust-src
