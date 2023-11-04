use std::mem::swap;

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // [1,2,3] --
    // [1,1,4] --

    let total_len = nums1.len() + nums2.len();
    let partition_len = total_len / 2;

    let (mut A, mut B) = (nums1, nums2);

    if A.len() > B.len() {
        swap(&mut A, &mut B);
    }

    let (mut l, mut r): (i32, i32) = (0, A.len() as i32 - 1);

    while true {
        let mut i = (l + r) / 2;
        if (l + r) < 0 {
            i = -1;
        }
        let j: i32 = partition_len as i32 - i - 2;

        let A_left = if i >= 0 { A[i as usize] } else { i32::MIN };
        let A_right = if (i + 1) < A.len() as i32 {
            A[(i + 1) as usize]
        } else {
            i32::MAX
        };

        let B_left = if j >= 0 { B[j as usize] } else { i32::MIN };
        let B_right = if (j + 1) < B.len() as i32 {
            B[(j + 1) as usize]
        } else {
            i32::MAX
        };

        if (A_left <= B_right) && (B_left <= A_right) {
            if total_len % 2 != 0 {
                return A_right.min(B_right) as f64;
            } else {
                return (A_left.max(B_left) as f64 + A_right.min(B_right) as f64) / 2_f64;
            }
        } else if A_left > B_right {
            r = i - 1;
        } else {
            l = i + 1;
        }
    }

    unreachable!()
}
