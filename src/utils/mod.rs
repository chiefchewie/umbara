pub mod tokenize;
mod traverse;

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_python() -> Language;
}
extern "C" {
    fn tree_sitter_javascript() -> Language;
}

pub fn get_language(language_name: &str) -> Option<tree_sitter::Language> {
    match language_name {
        "python" => unsafe { Some(tree_sitter_python()) },
        "py" => unsafe { Some(tree_sitter_python()) },
        "javascript" => unsafe { Some(tree_sitter_javascript()) },
        "js" => unsafe { Some(tree_sitter_javascript()) },
        _ => None,
    }
}
