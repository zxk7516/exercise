pub fn check(candidate: &str) -> bool {
    let mut letters: Vec<char> = candidate
        .to_lowercase()
        .chars()
        .filter( |ch| ch.is_alphabetic())
        .collect();
    while let Some(letter) = letters.pop(){
        if letters.contains(&letter){
            return false;
        }
    }

    true
}
