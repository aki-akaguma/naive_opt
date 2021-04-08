pub fn create_data() -> (Vec<String>, usize, &'static str, &'static str, &'static str) {
    let sw = std::env::var("AKI_TEST_DAT").unwrap_or("en.1".to_string());
    match sw.as_str() {
        "ja.1" => create_data_ja_1(),
        _ => create_data_en_1(),
    }
}

pub fn create_data_en_1() -> (Vec<String>, usize, &'static str, &'static str, &'static str) {
    let s1 =
        "abcdefghijk1234567890".repeat(10) + "ErrWarnAlert" + "abcdefghijklmno".repeat(10).as_str();
    let s2 = "abcdefghijk1234567890".repeat(10) + "abcdefghijklmno".repeat(10).as_str();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 500 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = v.len() / 2;
    (
        v,
        match_cnt,
        "ErrWarnAlert",
        "ErrWarnAlert",
        "*ErrWarnAlert*",
    )
}

pub fn create_data_ja_1() -> (Vec<String>, usize, &'static str, &'static str, &'static str) {
    let s1 = "吾輩は猫である".repeat(10) + "夏目漱石" + "坊っちゃん".repeat(10).as_str();
    let s2 = "名前はまだない".repeat(10) + "坊っちゃん".repeat(10).as_str();
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 500 {
            break;
        }
        if i % 2 == 0 {
            v.push(s1.clone());
        } else {
            v.push(s2.clone());
        }
    }
    let match_cnt = v.len() / 2;
    (v, match_cnt, "夏目漱石", "夏目漱石", "*夏目漱石*")
}
