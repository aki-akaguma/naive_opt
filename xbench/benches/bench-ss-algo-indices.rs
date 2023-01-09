use criterion::{black_box, criterion_group, criterion_main, Criterion};
use naive_opt::Search;

#[inline(never)]
fn process_std_str_str(texts: &[&str], pattern: &str) -> usize {
    let mut found: usize = 0;
    for line in texts {
        for _m in line.match_indices(pattern) {
            found += 1;
        }
    }
    found
}

#[inline(never)]
fn process_func_str_str(texts: &[&str], pattern: &str) -> usize {
    let mut found: usize = 0;
    for line in texts {
        for _m in naive_opt::string_search_indices(line, pattern) {
            found += 1;
        }
    }
    found
}

#[inline(never)]
fn process_trait_str_str(texts: &[&str], pattern: &str) -> usize {
    let mut found: usize = 0;
    for line in texts {
        for _m in line.search_indices(pattern) {
            found += 1;
        }
    }
    found
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, match_cnt, pat_string_s, _pat_regex_s, _pat_glob_s) = create_data::create_data();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //
    let n = process_std_str_str(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    let n = process_func_str_str(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    let n = process_trait_str_str(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    //
    c.bench_function("std_indices", |b| {
        b.iter(|| {
            let _r = process_std_str_str(black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("func_indices", |b| {
        b.iter(|| {
            let _r = process_func_str_str(black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("trait_indices", |b| {
        b.iter(|| {
            let _r = process_trait_str_str(black_box(&vv), black_box(pat_string_s));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(4000));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
