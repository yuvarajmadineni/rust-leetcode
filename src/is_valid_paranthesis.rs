pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    let brackets: Vec<char> = s.chars().collect();
    for i in 0..brackets.len() {
        if brackets[i] == '{' || brackets[i] == '(' || brackets[i] == '[' {
            stack.push(brackets[i]);
        }
        if brackets[i] == '}' {
            let top_char = stack.pop();

            if let Some(v) = top_char {
                if v != '{' {
                    return false;
                }
            } else {
                return false;
            }
        }
        if brackets[i] == ')' {
            let top_char = stack.pop();
            if let Some(v) = top_char {
                if v != '(' {
                    return false;
                }
            } else {
                return false;
            }
        }
        if brackets[i] == ']' {
            let top_char = stack.pop();

            if let Some(v) = top_char {
                if v != '[' {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    stack.is_empty()
}
