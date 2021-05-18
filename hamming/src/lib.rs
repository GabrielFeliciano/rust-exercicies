/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() == s2.len() {
        let mut s2_iter = s2.chars().fuse();

        Some(s1
            .chars()
            .fuse()
            .fold(0, |total, next| {
                total + if s2_iter.next().unwrap() != next { 1 } else { 0 }
            })
        )
    } else { None }
}
