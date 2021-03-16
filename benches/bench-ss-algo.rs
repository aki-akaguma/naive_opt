use criterion::{black_box, criterion_group, criterion_main, Criterion};
use naive_opt::Search;

fn process_func_str_str(texts: &[&str], pattern: &str) -> usize {
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = naive_opt::string_search(line, pattern) {
            found += 1;
        }
    }
    found
}

fn process_func_string_string(texts: &[String], pattern: &String) -> usize {
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = naive_opt::string_search(line, pattern) {
            found += 1;
        }
    }
    found
}

fn process_trait_str_str(texts: &[&str], pattern: &str) -> usize {
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = line.search(pattern) {
            found += 1;
        }
    }
    found
}

fn process_trait_string_string(texts: &[String], pattern: &String) -> usize {
    let mut found: usize = 0;
    for line in texts {
        if let Some(_n) = line.search(pattern) {
            found += 1;
        }
    }
    found
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, match_cnt, pat_string_s, _pat_regex_s, _pat_glob_s) = create_data::create_data();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    let pat_string = pat_string_s.to_string();
    //
    let n = process_func_str_str(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    let n = process_func_string_string(black_box(&v), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    let n = process_trait_str_str(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    let n = process_trait_string_string(black_box(&v), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    //
    c.bench_function("func_str_str", |b| {
        b.iter(|| {
            let _r = process_func_str_str(black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("func_string_string", |b| {
        b.iter(|| {
            let _r = process_func_string_string(black_box(&v), black_box(&pat_string));
        })
    });
    c.bench_function("trait_str_str", |b| {
        b.iter(|| {
            let _r = process_trait_str_str(black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("trait_string_string", |b| {
        b.iter(|| {
            let _r = process_trait_string_string(black_box(&v), black_box(&pat_string));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
