struct Solution;

impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        // Reverse the entire vector
        s.reverse();
        let mut prev = 0;
        for i in 0..s.len() {
            // Traverse till a space is found, then reverse from the previous space to the current space
            if s[i] == ' ' {
                s[prev..i].reverse();
                prev = i + 1;
            }
        }
        let slen = s.len();
        // Reverse the last word
        s[prev..slen].reverse();
    }
}

fn main() {
    let mut input = vec!['t', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e'];
    Solution::reverse_words(&mut input);
    println!("Result: {:?}", input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse_words_test1() {
        let mut input = vec!['t', 'h', 'i', 's', ' ', 'i', 's', ' ', 't', 'h', 'e', ' ', 's', 't', 'a', 'r', 't'];
        let expected_outut = vec!['s', 't', 'a', 'r', 't', ' ', 't', 'h', 'e', ' ', 'i', 's', ' ', 't', 'h', 'i', 's'];
        Solution::reverse_words(&mut input);
        assert_eq!(input, expected_outut);
    }
}
