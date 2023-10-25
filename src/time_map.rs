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
            .or_default()
            .push((value, timestamp));
    }

    pub fn get(&mut self, key: String, timestamp: i32) -> String {
        if let Some(time_vec) = self.time_map.get(&key) {
            return match time_vec.binary_search_by_key(&timestamp, |&(_, time)| time) {
                Ok(ind) => time_vec[ind].0.clone(),
                Err(i) if i > 0 => time_vec[i - 1].0.clone(),
                _ => "".to_string(),
            };
        }
        "".to_string()
    }
}
