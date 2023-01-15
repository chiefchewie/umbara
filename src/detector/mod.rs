use crate::detector::algorithms::binary_search;
use crate::detector::algorithms::hash_kgrams;
use crate::detector::algorithms::winnow;
use crate::utils::{
    get_language,
    tokenize::{tokenize_tree, TokenKind},
};
use tree_sitter::{Parser, Tree};

mod algorithms;
pub struct DocumentFingerprint {
    pub code: String,
    filtered_code: String,
    offsets: Vec<(usize, usize)>,
    k: usize,
    hashes: Vec<u64>,
    selected_indices: Vec<usize>,
}

impl DocumentFingerprint {
    fn selected_coverage(&self) -> usize {
        let mut coverage = vec![0; self.filtered_code.as_bytes().len()];
        for idx in self.selected_indices.iter() {
            for j in 0..self.k {
                coverage[idx + j] = 1;
            }
        }
        coverage.iter().sum()
    }
}

pub fn generate_fingerprint(
    code: &str,
    language: &str,
    guarantee_threshold: usize,
    noise_threshold: usize,
) -> DocumentFingerprint {
    let t = guarantee_threshold;
    let k = noise_threshold;
    let window_size = t - k + 1;

    let lang = get_language(language).unwrap();

    let mut parser = Parser::new();
    parser.set_language(lang).unwrap();

    let tree = parser.parse(code, None).unwrap();

    let (filtered_code, offsets) = filter_code(&tree, code.as_bytes());
    let hashes = hash_kgrams(filtered_code.as_bytes(), k);
    let selected_indices = winnow(&hashes, window_size);

    DocumentFingerprint {
        code: String::from(code),
        filtered_code,
        offsets,
        k,
        hashes,
        selected_indices,
    }
}

// Detection algorithm starts
pub fn compare_fingerprints(
    fp1: &DocumentFingerprint,
    fp2: &DocumentFingerprint,
) -> (Vec<(usize, usize)>, f32, Vec<(usize, usize)>, f32) {
    assert_eq!(
        fp1.k, fp2.k,
        "Error: Fingerprints must be generated with the same noise threshold!"
    );

    let k = fp1.k;

    let (matched_indices1, matched_indices2) = find_matching_hashes(
        &fp1.hashes,
        &fp1.selected_indices,
        &fp2.hashes,
        &fp2.selected_indices,
    );

    let (slices1_begin, slices1_end) = get_copied_slices(&matched_indices1, k);
    let (slices2_begin, slices2_end) = get_copied_slices(&matched_indices2, k);

    let mut matches1: Vec<(usize, usize)> = Vec::new();
    for (&s, &e) in slices1_begin.iter().zip(slices1_end.iter()) {
        let s_i = binary_search(&(s, 0), &fp1.offsets).min(fp1.offsets.len() - 1);
        let e_i = binary_search(&(e, 0), &fp1.offsets).min(fp1.offsets.len() - 1);

        let ns = fp1.offsets[s_i].1;
        let ne = fp1.offsets[e_i].1;
        matches1.push((ns, ne));
    }

    let similarity1 = calc_similarity(fp1, &slices1_begin, &slices1_end);

    let mut matches2: Vec<(usize, usize)> = Vec::new();
    for (&s, &e) in slices2_begin.iter().zip(slices2_end.iter()) {
        let s_i = binary_search(&(s, 0), &fp2.offsets).min(fp2.offsets.len() - 1);
        let e_i = binary_search(&(e, 0), &fp2.offsets).min(fp2.offsets.len() - 1);

        let ns = fp2.offsets[s_i].1;
        let ne = fp2.offsets[e_i].1;

        matches2.push((ns, ne));
    }
    let similarity2 = calc_similarity(fp2, &slices2_begin, &slices2_end);

    (matches1, similarity1, matches2, similarity2)
}

fn filter_code(tree: &Tree, code: &[u8]) -> (String, Vec<(usize, usize)>) {
    let mut out_code = String::new();
    let mut offsets: Vec<(usize, usize)> = vec![(0, 0)];

    for token in tokenize_tree(tree, code) {
        match token.kind {
            TokenKind::NameFunction => out_code.push('F'),
            TokenKind::NameClass => out_code.push('O'),
            TokenKind::Name => out_code.push('V'),
            TokenKind::Text => out_code.push_str(token.text),
        }
        offsets.push((out_code.len(), token.end_byte));
    }

    (out_code, offsets)
}

fn find_matching_hashes(
    hashes1: &Vec<u64>,
    indices1: &Vec<usize>,
    hashes2: &Vec<u64>,
    indices2: &Vec<usize>,
) -> (Vec<usize>, Vec<usize>) {
    let mut matched_indices1: Vec<usize> = Vec::new();
    for &i in indices1.iter() {
        let h = &hashes1[i];
        if hashes2.contains(h) {
            matched_indices1.push(i);
        }
    }

    let mut matched_indices2: Vec<usize> = Vec::new();
    for &i in indices2.iter() {
        let h = &hashes2[i];
        if hashes1.contains(h) {
            matched_indices2.push(i);
        }
    }
    (matched_indices1, matched_indices2)
}

fn get_copied_slices(idx: &Vec<usize>, k: usize) -> (Vec<usize>, Vec<usize>) {
    let mut sorted_idx = idx.clone();
    sorted_idx.sort();

    let mut skips: Vec<usize> = Vec::new();
    for (i, window) in sorted_idx.windows(2).enumerate() {
        let (current, next) = (window[0], window[1]);
        if next - current > k - 1 {
            skips.push(i);
        }
    }

    let mut slice_starts: Vec<usize> = Vec::new();
    let mut slice_ends: Vec<usize> = Vec::new();

    slice_starts.push(idx[0]);
    for i in skips.iter() {
        slice_starts.push(idx[i + 1]);
    }

    for i in skips.iter() {
        slice_ends.push(idx[*i] + k);
    }

    slice_ends.push(idx.last().unwrap() + k);

    (slice_starts, slice_ends)
}

fn calc_similarity(
    fp: &DocumentFingerprint,
    slice_begins: &Vec<usize>,
    slice_ends: &Vec<usize>,
) -> f32 {
    let selected_code_count = fp.selected_coverage() as f32;
    let it = slice_begins.iter().zip(slice_ends.iter());
    let matched_code_count = it.fold(0, |acc, (start, end)| acc + (end - start)) as f32;

    matched_code_count / selected_code_count
}

// More ergonomic stuff -  a Detector & graphical outputs

#[allow(unused)]
pub fn html_report(
    fp1: &DocumentFingerprint,
    fp2: &DocumentFingerprint,
    matches1: Vec<(usize, usize)>,
    matches2: Vec<(usize, usize)>,
    similarity1: f32,
    similarity2: f32,
) {
    todo!()
}

pub fn print_report_terminal(
    fp1: &DocumentFingerprint,
    fp2: &DocumentFingerprint,
    matches1: &Vec<(usize, usize)>,
    matches2: &Vec<(usize, usize)>,
    similarity1: f32,
    similarity2: f32,
) {
    println!("Similarity rating for file 1: {}", similarity1);
    println!("Matches found in file 1: ");
    for &(start, end) in matches1.iter() {
        println!(">>>");
        println!("{}", &fp1.code[start..end]);
        println!("<<<");
    }

    println!("\n-----------------------------------\n");

    println!("Similarity rating for file 2: {}", similarity2);
    println!("Matches found in file 1: ");
    for &(start, end) in matches2.iter() {
        println!(">>>");
        println!("{}", &fp2.code[start..end]);
        println!("<<<");
    }
}
