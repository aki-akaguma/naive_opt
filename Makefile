BENCH_STR = --bench=bench-ss-algo --bench=bench-ss-algo-indices
#BENCH_STR = --bench=bench-ss-algo-indices

TARGET_GNU  = --target=x86_64-unknown-linux-gnu
TARGET_MUSL = --target=x86_64-unknown-linux-musl
TSK = taskset -c 2

all:

bench-all: bench-gnu bench-musl

bench-build-all: bench-build-gnu bench-build-musl


bench-gnu: bench.en.1-gnu bench.ja.1-gnu

bench-musl: bench.en.1-musl bench.ja.1-musl

bench-build-gnu:
	cargo bench --no-run $(TARGET_GNU)
	@touch target/stamp.bench-build-gnu

bench-build-musl:
	cargo bench --no-run $(TARGET_MUSL)
	@touch target/stamp.bench-build-musl

bench-clean:
	@rm -fr target/criterion

clean:
	@cargo clean
	@rm -f z.*

report:
	cargo xtask shape_benchmark_results


bench.en.1-gnu: target/stamp.bench-build-gnu
	@rm -f z.gnu.bench.en.1.log
	env AKI_TEST_DAT=en.1 $(TSK) cargo bench $(BENCH_STR) $(TARGET_GNU) -- -n | tee -a z.gnu.bench.en.1.log

bench.ja.1-gnu: target/stamp.bench-build-gnu
	@rm -f z.gnu.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 $(TSK) cargo bench $(BENCH_STR) $(TARGET_GNU) -- -n | tee -a z.gnu.bench.ja.1.log

bench.en.1-musl: target/stamp.bench-build-musl
	@rm -f z.musl.bench.en.1.log
	env AKI_TEST_DAT=en.1 $(TSK) cargo bench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.en.1.log

bench.ja.1-musl: target/stamp.bench-build-musl
	@rm -f z.musl.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 $(TSK) cargo bench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.ja.1.log
