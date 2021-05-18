pub fn build_proverb(list: &[&str]) -> String {
    let mut sentences: Vec<String> = vec![];

    list
        .iter()
        .reduce(|w1, w2| {
            let sentence = format!("For want of a {} the {} was lost.", w1, w2);
            sentences.push(sentence);
            w2
        });

    match list.first() {
        Some(word) => sentences.push(format!("And all for the want of a {}.", word)),
        None => ()
    }

    sentences.join("\n")
}
