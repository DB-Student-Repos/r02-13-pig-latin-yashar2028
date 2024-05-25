const VOWEL_SOUNDS: [&str; 7] = ["a", "e", "i", "o", "u", "xr", "yt"];

pub fn translate(input: &str) -> String {
    let words = input.split_whitespace().filter(|x| !x.is_empty());
    let mut string = String::new();
    for word in words {
        let mut w = word.to_string();
        
        if !VOWEL_SOUNDS.iter().any(|c| w.starts_with(c)) {
            let range = match w.chars().collect::<Vec<char>>()[..3] {
                [ _ , 'q', 'u'] => 3,
                [ _ , 'c', 'h'] => 3, 
                ['t', 'h', 'r'] => 3,
                ['q', 'u',  _ ] => 2,
                [ _ , 'h',  _ ] => 2,
                _ => 1
            };
            for _ in 0..range {
                let c = w.remove(0);
                w.push(c);
            }
        }
        
        w.push_str("ay ");
        string.push_str(&w);
    }
    string.trim().to_string()
}
