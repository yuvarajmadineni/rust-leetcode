pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    if tokens.len() == 1 {
        return tokens.last().unwrap().parse::<i32>().unwrap();
    }
    let mut stack: Vec<String> = vec![];
    let mut final_res = 0;
    for i in 0..tokens.len() {
        let token = tokens[i].clone();
        if token == '*'.to_string() {
            let second = stack.pop().unwrap();
            let first = stack.pop().unwrap();
            final_res = first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
            stack.push(final_res.to_string());
        } else if token == '+'.to_string() {
            let second = stack.pop().unwrap();
            let first = stack.pop().unwrap();
            final_res = first.parse::<i32>().unwrap() + second.parse::<i32>().unwrap();
            stack.push(final_res.to_string())
        } else if token == '-'.to_string() {
            let second = stack.pop().unwrap();
            let first = stack.pop().unwrap();
            final_res = first.parse::<i32>().unwrap() - second.parse::<i32>().unwrap();
            stack.push(final_res.to_string())
        } else if token == '/'.to_string() {
            let second = stack.pop().unwrap();
            let first = stack.pop().unwrap();
            final_res = first.parse::<i32>().unwrap() / second.parse::<i32>().unwrap();
            stack.push(final_res.to_string())
        } else {
            stack.push(token);
        }
    }
    final_res
}
