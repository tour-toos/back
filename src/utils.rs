use rand::Rng;

/// Fisher–Yates shuffle
pub fn shuffle<T>(vec: Vec<T>) -> Vec<T> {
    let mut result = vec;
    for i in (1..result.len() - 1).rev() {
        result.swap(rand::thread_rng().gen_range(0..i), i);
    }
    result.into()
}
