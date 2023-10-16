pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![0; temperatures.len()];
    let mut stack: Vec<i32> = vec![];
    stack.push(0);
    for i in 1..temperatures.len() {
        while !stack.is_empty() {
            let last_ind = *stack.last().unwrap();
            let last_ele = temperatures[last_ind as usize];
            if temperatures[i] <= last_ele {
                break;
            }
            res[last_ind as usize] = i as i32 - last_ind;
            stack.pop();
        }
        stack.push(i as i32);
    }
    res
}
