use anyhow::{Result};
use rand::Rng;

const SPECIALS: &[u8] = b"!@#%&*?=+:";

#[allow(dead_code)]
pub fn display(text: &str) -> Result<()> {
    println!("{}", text);
    Ok(())
}

#[allow(dead_code)]
pub fn random_specials(count: usize) -> Vec<u8> {
    let mut specials: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 1..count + 1 {
        specials.push(SPECIALS[rng.gen_range(0..SPECIALS.len())])
    }
    specials
}