pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    fn search(nums: &Vec<i32>, target: i32) -> bool {
        let n: usize = nums.len();
        let mut low: i32 = 0;
        let mut high: i32 = (n - 1) as i32;
        let mut mid = (low + high) / 2;

        while low <= high {
            if nums[mid as usize] == target {
                return true;
            }
            if nums[mid as usize] > target {
                high = mid - 1;
            } else {
                low = mid + 1;
            }

            mid = (low + high) / 2;
        }

        false
    }
    let mut lower_row: i32 = 0;
    let mut upper_row = (matrix.len() - 1) as i32;

    let mut mid_row = (lower_row + upper_row) / 2;

    while lower_row <= upper_row {
        if matrix[mid_row as usize][0] == target {
            return true;
        }
        if matrix[mid_row as usize][0] > target {
            upper_row = mid_row - 1;
        } else {
            if *matrix[mid_row as usize].last().unwrap() >= target {
                break;
            }
            lower_row = mid_row + 1;
        }

        mid_row = (lower_row + upper_row) / 2;
    }

    search(matrix[mid_row as usize].as_ref(), target)
}
