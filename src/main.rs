use crate::detector::{compare_fingerprints, generate_fingerprint};
use std::fs;

mod detector;
mod utils;
fn main() {
    const GUARANTEE_THRESHOOLD: usize = 25;
    const NOISE_THRESHOLD: usize = 25;

    let data1 = fs::read_to_string("test\\s1.py").expect("Unable to read file");
    let data2 = fs::read_to_string("test\\s2.py").expect("Unable to read file");

    let fp1 = generate_fingerprint(&data1, "python", GUARANTEE_THRESHOOLD, NOISE_THRESHOLD);
    let fp2 = generate_fingerprint(&data2, "python", GUARANTEE_THRESHOOLD, NOISE_THRESHOLD);

    compare_fingerprints(&fp1, &fp2);
}
