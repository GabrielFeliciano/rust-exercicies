pub fn raindrops(n: u32) -> String {
    let mut result = String::from("");

    let labels = ["Pling", "Plang", "Plong"];
    let factors = [3, 5, 7];

    for (i, label) in labels.iter().enumerate() {
        if n % factors[i] == 0 {
            result.push_str(label)
        }
    }
    if result.len() == 0 { 
        return n.to_string();
    }

    return result;
}
