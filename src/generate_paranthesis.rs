pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn generate(s: String, open: i32, close: i32) -> Vec<String> {
        let mut res = vec![];

        if open == 0 && close == 0 {
            return vec![s];
        }

        if open > 0 {
            let mut result = generate(s.clone() + "(", open - 1, close + 1);
            res.append(&mut result)
        }

        if close > 0 {
            res.append(&mut generate(s + ")", open, close - 1));
        }
        res
    }
    generate("".to_string(), n, 0)
}
