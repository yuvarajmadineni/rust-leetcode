pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    fn is_possible(piles: &Vec<i32>, h: i32, speed: i32) -> bool {
        if speed == 0 {
            return false;
        }
        let mut c = 0;
        for pile in piles {
            if pile % speed == 0 {
                c += pile / speed;
            } else {
                c = c + 1 + (((pile / speed) as f32).ceil()) as i32;
            }
        }
        c <= h
    }

    let mut high = *piles.iter().max().unwrap();
    let mut low = 0;
    let mut mid;
    let mut possible_val = 0;
    while low <= high {
        mid = (low + high) / 2;
        if is_possible(&piles, h, mid) {
            possible_val = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    possible_val
}
