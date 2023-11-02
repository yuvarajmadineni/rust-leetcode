pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums3 = Vec::new();
    let first_len = nums1.len();
    let second_len = nums2.len();
    let (mut i, mut j) = (0, 0);

    while i < first_len && j < second_len {
        if nums1[i] < nums2[j] {
            nums3.push(nums1[i]);
            i += 1;
        } else {
            nums3.push(nums2[j]);
            j += 1;
        }
    }

    for k in i..first_len {
        nums3.push(nums1[k]);
    }

    for k in j..second_len {
        nums3.push(nums2[k]);
    }

    if nums3.len() % 2 == 0 {
        let mid = nums3.len() / 2;
        let first_mid = (nums3.len() / 2) - 1;

        (nums3[mid] + nums3[first_mid]) as f64 / 2_f64
    } else {
        nums3[nums3.len() / 2] as f64
    }
}
