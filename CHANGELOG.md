TBD
===
Unreleased changes. Release notes have not yet been written.

0.1.13 (2021-07-06)
=====

* update crates: memx(0.1.14)
* rewrite doc
* update licenses

0.1.12 (2021-06-20)
=====

* update crates: memx(0.1.12) - the important bugs fixed
* add "Cargo.lock" into .gitignore

0.1.11 (2021-06-06)
=====

* faster ascii searching
* add ascii stochastics
* add naive_opt_mc_1st_bytes() and naive_opt_mc_1st_rev_bytes()

0.1.10 (2021-06-03)
=====

* replace new bench data: a match size is more than old.
* add to test: a needle large size.
* add crate memx into depends.
* remove crate libc and crate memchr.

0.1.9 (2021-05-09)
=====

* update depends: memchr(2.4.0)
* split into fallback.rs

0.1.8 (2021-04-20)
=====

* add xxx_bytes()

0.1.7 (2021-04-09)
=====

* add output2() into `task shape_benchmark_results`
* add libc support for 'cfg(not(target_arch = "x86_64"))'

0.1.6 (2021-04-04)
=====

* clippy fix
* add trait SearchIn.is_empty()

0.1.5 (2021-03-20)
=====

* add Search::rsearch()
* add Search::rsearch_indices()
* add string_rsearch()
* add string_rsearch_indices()
* add more docs

0.1.4 (2021-03-19)
=====

* add impl SearchIn for char
* add Search::includes()

0.1.3 (2021-03-18)
=====

* tune up search_indices

0.1.2 (2021-03-17)
=====

* add search_indices

0.1.1 (2021-03-17)
=====

* add docs

0.1.0 (2021-03-17)
=====

first commit
