const REPEAT_TIMES: usize = 100_000;
const DATA_LENGTH: usize = 10_000;

use rand::{thread_rng, RngCore};
use sha2::{Digest, Sha256};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    rayon::scope(|spawner| {
        (0..REPEAT_TIMES).for_each(|_| {
            spawner.spawn(move |_| {
                let bytes = gen_bytes();
                gen_sha(bytes);
            });
        });
    });

    println!("Measured: {}ms.", now.elapsed().as_millis());
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
