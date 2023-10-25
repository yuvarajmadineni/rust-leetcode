use std::collections::HashMap;

pub struct TimeMap {
    time_map: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMap {
    pub fn new() -> Self {
        TimeMap {
            time_map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.time_map
            .entry(key)
            .and_modify(|c| c.push((value.clone(), timestamp)))
            .or_insert([(value.clone(), timestamp)].to_vec());
    }

    pub fn get(&mut self, key: String, timestamp: i32) -> String {
        let mut next_close_val = "".to_string();

        if let Some(time_vec) = self.time_map.get(&key) {
            let mut low: i32 = 0;
            let mut high = time_vec.len() as i32 - 1;
            let mut next_closest = i32::MIN;

            while low <= high {
                let mid = (low + high) / 2;

                if timestamp == time_vec[mid as usize].1 {
                    return time_vec[mid as usize].0.clone();
                }

                if timestamp > time_vec[mid as usize].1 {
                    if next_closest < time_vec[mid as usize].1 {
                        next_close_val = time_vec[mid as usize].0.clone();
                    }
                    next_closest = next_closest.max(time_vec[mid as usize].1);
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }
        next_close_val
    }
}
