pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    let mut s1_count = [0; 26];
    let mut s2_count = [0; 26];
    let mut matches = 0;

    for i in 0..s1.len() {
        s1_count[s1.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
        s2_count[s2.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
    }

    for i in 0..26 {
        if s1_count[i] == s2_count[i] {
            matches += 1;
        }
    }

    let mut left = 0;

    for j in s1.len()..s2.len() {
        if matches == 26 {
            return true;
        }

        let mut index = s2.chars().nth(j).unwrap() as usize - 'a' as usize;

        s2_count[index] += 1;
        if s1_count[index] == s2_count[index] {
            matches += 1;
        } else if s1_count[index] + 1 == s2_count[index] {
            matches -= 1;
        }

        index = s2.chars().nth(left).unwrap() as usize - 'a' as usize;
        s2_count[index] -= 1;

        if s2_count[index] == s1_count[index] {
            matches += 1;
        } else if s1_count[index] - 1 == s2_count[index] {
            matches -= 1;
        }
        left += 1;
    }

    matches == 26
}
