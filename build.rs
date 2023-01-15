use std::path::PathBuf;

fn main() {
    let mut build = cc::Build::new();

    let languages_dir = "languages";
    let supported_languages = ["tree-sitter-python", "tree-sitter-javascript"];

    for lang in supported_languages.iter() {
        let folder = [languages_dir, lang, "src"].iter().collect::<PathBuf>();

        build.include(&folder);
        build.file(folder.join("parser.c"));

        if folder.join("scanner.cc").exists() {
            build.file(folder.join("scanner.cc"));
        } else if folder.join("scanner.c").exists() {
            build.file(folder.join("scanner.c"));
        }
    }
    build.compile("mybrother");
}
