pub fn series(digits: &str, len: usize) -> Vec<String> {
    // unimplemented!(
    //     "What are the series of length {} in string {:?}",
    //     len,
    //     digits
    // )
    if digits.len() < len {
        vec![]
    }else if len == 0 {
        let mut vect = Vec::new();
        let mut i = digits.len()+1;
        while i != 0 {
            vect.push("".to_string());
            i -= 1;
        }
        vect
    }else{
        let mut vect = Vec::new();
        let mut i = 0;
        loop {
           vect.push(digits[i..i+len].to_string());
           if i+len == digits.len()  {
                break;
            }
            i = i+1;
        }
        vect
    }
}
