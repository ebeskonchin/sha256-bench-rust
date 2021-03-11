use rand::{thread_rng, RngCore};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::time::Instant;

const REPEAT_TIMES: usize = 100_000;
const DATA_LENGTH: usize = 10_000;

fn main() {
    let now = Instant::now();

    print!("Threads count: {}, ", rayon::current_num_threads());

    (0..REPEAT_TIMES)
        .into_par_iter()
        .map(|_| gen_bytes())
        .for_each(|bytes| {
            gen_sha(bytes);
        });

    println!("Measured: {:?}.", now.elapsed());
}

fn gen_sha(from: [u8; DATA_LENGTH]) {
    let mut sha = Sha256::new();
    sha.update(&from);
    sha.finalize();
}

fn gen_bytes() -> [u8; DATA_LENGTH] {
    let mut result = [0u8; DATA_LENGTH];
    thread_rng().fill_bytes(&mut result);
    result
}
