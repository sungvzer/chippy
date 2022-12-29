use rand::Rng;

pub fn random_byte() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=255).try_into().unwrap()
}
