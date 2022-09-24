fn main() {
    pub fn reverse_only_letters(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            if !s[l].is_ascii_alphabetic() {
                l += 1;
                continue;
            }
            if !s[r].is_ascii_alphabetic() {
                r -= 1;
                continue;
            }
            s.swap(l, r);
            l += 1;
            r -= 1;
        }
        s.iter().collect()
    }

reverse_only_letters("ab-c".to_string());
}
