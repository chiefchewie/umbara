use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn my_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);

    hasher.finish()
}

pub fn hash_kgrams(code: &[u8], k: usize) -> Vec<u64> {
    let mut hashes: Vec<u64> = Vec::new();
    for i in 0..(code.len() - k + 1) {
        hashes.push(my_hash(&code[i..i + k]));
    }

    hashes
}

pub fn winnow(hashes: &Vec<u64>, window_size: usize) -> Vec<usize> {
    _local_winnow(hashes, window_size)
}

fn _local_winnow(hashes: &Vec<u64>, window_size: usize) -> Vec<usize> {
    let mut selected_indices: Vec<usize> = Vec::new();
    for (idx, window) in hashes.windows(window_size).enumerate() {
        let mut min_idx = idx;
        let mut min_hash = u64::MAX;
        for (i, &h) in window.iter().enumerate() {
            if h <= min_hash {
                min_hash = h;
                min_idx = idx + i;
            }
        }
        selected_indices.push(min_idx);
    }
    selected_indices.sort();
    selected_indices.dedup();
    selected_indices
}

pub fn binary_search<T>(t: &T, a: &Vec<T>) -> usize
where
    T: Ord,
{
    let mut low = 0;
    let mut high = a.len();
    let mut mid = (low + high) / 2;

    while low < high {
        match a[mid].cmp(&t) {
            std::cmp::Ordering::Equal => {
                return mid;
            }
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
        mid = (low + high) / 2;
    }

    return high;
}
