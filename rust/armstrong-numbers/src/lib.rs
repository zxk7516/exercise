pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
     num_str.to_string().chars().fold(0, |acc, ch| {
         let ch_num = (ch as u8 - '0' as u8) as u32;
         acc + ch_num.pow(num_str.len() as u32)
     }) == num
}
