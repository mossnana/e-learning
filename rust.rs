fn longest_palindrome_substring(s: String) -> String {
    let s_len = s.len();
    let (mut max_len, mut max_left, mut max_right) = (0, 0, 0);
    for i in 0..s_len {
      // odd
      let (mut left, mut right) = (i, i);
      while left > 0 && right < s_len && s.chars().nth(left) == s.chars().nth(right) {
        let curr_len = right-left+1;
        if curr_len > max_len {
          (max_len, max_left, max_right) = (curr_len, left, right);
        }
        left -= 1;
        right += 1;
      }
  
      // even
      let (mut left, mut right) = (i, i+1);
      while left > 0 && right < s_len && s.chars().nth(left) == s.chars().nth(right) {
        let curr_len = right-left+1;
        if curr_len > max_len {
          (max_len, max_left, max_right) = (curr_len, left, right);
        }
        left -= 1;
        right += 1;
      }
    }
    
    s[max_left..max_right+1].to_string()
}