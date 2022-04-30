pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num)
    let mut result = 0;
    let mut num_copy:u32 = num;
    let power_len:u32 = num.to_string().len() as u32;
    while num_copy > 0 {
      let tmp = num_copy % 10;
      result += tmp.pow(power_len);
      num_copy /= 10;
    }
    result == num
}
