use crate::detector::{compare_fingerprints, generate_fingerprint, print_report_terminal};
use std::{env, fs, path::PathBuf};

mod detector;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let directory = &args[1];
    let file1 = &args[2];
    let file2 = &args[2];

    const GUARANTEE_THRESHOOLD: usize = 25;
    const NOISE_THRESHOLD: usize = 25;

    let data1 = fs::read_to_string([directory, file1].iter().collect::<PathBuf>())
        .expect("Unable to read file");
    let data2 = fs::read_to_string([directory, file2].iter().collect::<PathBuf>())
        .expect("Unable to read file");

    let fp1 = generate_fingerprint(&data1, "python", GUARANTEE_THRESHOOLD, NOISE_THRESHOLD);
    let fp2 = generate_fingerprint(&data2, "python", GUARANTEE_THRESHOOLD, NOISE_THRESHOLD);

    let (matches1, similarity1, matches2, similarity2) = compare_fingerprints(&fp1, &fp2);
    print_report_terminal(&fp1, &fp2, &matches1, &matches2, similarity1, similarity2);
}
