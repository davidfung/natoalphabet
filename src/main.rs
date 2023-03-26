use std::collections::HashMap;

fn load_alphabet() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("A", "Alpha"),
        ("B", "Bravo"),
        ("C", "Charlie"),
        ("D", "Delta"),
        ("E", "Echo"),
        ("F", "Foxtrot"),
        ("G", "Golf"),
        ("H", "Hotel"),
        ("I", "India"),
        ("J", "Juliett"),
        ("K", "Kilo"),
        ("L", "Lima"),
        ("M", "Mike"),
        ("N", "November"),
        ("O", "Oscar"),
        ("P", "Papa"),
        ("Q", "Quebec"),
        ("R", "Romeo"),
        ("S", "Sierra"),
        ("T", "Tango"),
        ("U", "Uniform"),
        ("V", "Victor"),
        ("W", "Whiskey"),
        ("X", "X-ray"),
        ("Y", "Yankee"),
        ("Z", "Zulu"),
    ])
}

fn main() {
    let mut score = 0;
    let mut count = 0;
    let alphabet = load_alphabet();
    for (k,v) in &alphabet {
        count = count +1;
        println!("{}?", k);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let question = *v;
        let answer = line.trim();
        if question.to_lowercase() == answer.to_lowercase() {
            score = score + 1;
            println!("Correct! Score: {}/{}", score, count);
        } else {
            println!("===> Wrong! Should be {}!  Score:{}/{}", v, score, count);
        }
    }
}
