use rand::Rng;

pub fn coin_toss() -> &'static str {
    let toss = rand::thread_rng().gen_range(0..2);
    if toss == 0 {
        "Heads"
    } else {
        "Tails"
    }
}
