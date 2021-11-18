pub fn is_armstrong_number(num: u32) -> bool {
   // unimplemented!("true if {} is an armstrong number", num)
    let digits = num.to_string();
    let numLen = digits.len();
    num == digits.chars().map(|d| d.to_digit(10).unwrap().pow(numLen as u32)).sum()
}
