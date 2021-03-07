const REPEAT_TIMES: usize = 100_000;
const DATA_LENGTH: usize = 10_000;

use rand::{thread_rng, RngCore};
use sha2::{Digest, Sha256};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let _ = crossbeam::scope(|spawner| {
        (0..REPEAT_TIMES).map(|_| gen_bytes()).for_each(|data| {
            spawner.spawn(move |_| {
                gen_sha(data);
            });
        });
    });

    println!("Measured: {}ms.", now.elapsed().as_millis())
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
