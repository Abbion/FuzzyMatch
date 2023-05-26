mod fuzzy_match;

fn main() {
    let a = fuzzy_match::ratio("This is a string", "That is a string", None);
    let b = fuzzy_match::partial_ratio("Boston dynamic builds robots", "roboty", None);
    let c = fuzzy_match::token_sort_ratio("Nasa landed a man on the moon", "A man landed on the moon due to Nasa", None);
    let d = fuzzy_match::token_set_ratio("Todays match is the first match between Lakers and Galaxis", "Lakers v Galaxis", None);

    println!("{a}\n{b}\n{c}\n{d}\n\n");

    let a = fuzzy_ratio!("This is a string", "That is a string");
    let b = fuzzy_partial_ratio!("Boston dynamic builds robots", "roboty");
    let c = fuzzy_token_sort_ratio!("Nasa landed a man on the moon", "A man landed on the moon due to Nasa");
    let d = fuzzy_token_set_ratio!("Todays match is the first match between Lakers and Galaxis", "Lakers v Galaxis");

    println!("{a}\n{b}\n{c}\n{d}");
}
