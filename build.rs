use std::fs;
extern crate bindgen;

fn main() {
    let mut build = cc::Build::new();

    let languages_dir = "languages";
    let languages = fs::read_dir(languages_dir).unwrap();

    for l in languages {
        let lang = l.unwrap().path();
        if !lang.is_dir() {
            continue;
        }

        let folder = lang.join("src");

        build.include(&folder);

        if folder.join("parser.c").exists() {
            build.file(folder.join("parser.c"));
        }

        if folder.join("scanner.cc").exists() {
            build.file(folder.join("scanner.cc"));
        } else if folder.join("scanner.c").exists() {
            build.file(folder.join("scanner.c"));
        }
    }
    build.compile("ts_grammars");
}
