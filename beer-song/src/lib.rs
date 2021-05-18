pub fn bottle_count_str (n: u32) -> String {
    if n == 0 { String::from("no more") } else { n.to_string() }
}

pub fn should_be_plural (n: u32) -> String {
    String::from(if n == 1 { "" } else { "s" })
}

pub fn build_bottles (n: u32) -> String {
    format!("{} bottle{}", bottle_count_str(n), should_be_plural(n))
}

pub fn verse(n: u32) -> String {
    format!(
        "{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n", 

        build_bottles(n),
        build_bottles(n),
        if n == 1 { "it" } else { "one" },
        build_bottles(n - 1)
    )
}

pub fn sing(start: u32, end: u32) -> String {
    if start < end { panic!("start higher than end") }

    let mut total: Vec<String> = vec![];

    for i in end..(start+1) {
        let n = end + start - i;
        let to_push = if n == 0 {
            String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
        } else {
            verse(n)
        };

        total.push(to_push)
    }

    total.join("\n")
}
