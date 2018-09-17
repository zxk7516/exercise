pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut r = vec![];
    let digits_length = digits.len();
    if len == 0 {
        for _ in 0..digits_length+1 {
            r.push("".to_string());
        }
    } else if digits_length < len{
        
    }else{
        for i in 0..(digits_length-len+1) {
            r.push(digits.chars().skip(i).take(len).collect::<String>());
        }
        
    }

    r
    
}
