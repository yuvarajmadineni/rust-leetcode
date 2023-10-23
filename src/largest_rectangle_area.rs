pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left_stack: Vec<usize> = Vec::new();
    let mut right_stack: Vec<usize> = Vec::new();
    let mut left_area: Vec<i32> = vec![0; heights.len()];
    let mut right_area: Vec<i32> = vec![0; heights.len()];
    for i in 0..heights.len() {
        while !left_stack.is_empty() {
            if let Some(last_ind) = left_stack.last() {
                let height: i32 = heights[*last_ind];
                if height >= heights[i] {
                    left_stack.pop();
                } else {
                    break;
                }
            }
        }
        if left_stack.is_empty() {
            left_area[i] = -1;
        } else {
            let ind = left_stack.last().unwrap();
            left_area[i] = *ind as i32;
        }
        left_stack.push(i);
    }
    for i in (0..heights.len()).rev() {
        while !right_stack.is_empty() {
            if let Some(last_ind) = right_stack.last() {
                let height: i32 = heights[*last_ind];
                if height >= heights[i] {
                    right_stack.pop();
                } else {
                    break;
                }
            }
        }
        if right_stack.is_empty() {
            right_area[i] = heights.len() as i32;
        } else {
            let ind = right_stack.last().unwrap();
            right_area[i] = *ind as i32;
        }
        right_stack.push(i);
    }

    for i in 0..heights.len() {
        max_area = max_area.max((right_area[i] - left_area[i] - 1) * heights[i]);
    }
    max_area
}
