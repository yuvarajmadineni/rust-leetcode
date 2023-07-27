#[derive(Debug)]
pub struct Minstack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl Minstack {
    pub fn new() -> Self {
        Minstack {
            stack: vec![],
            min_stack: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        let last_min = self.min_stack.last().unwrap_or(&val);
        self.min_stack.push(val.min(*last_min));
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    pub fn top(&self) -> i32 {
        let top_val = self.stack.last().unwrap();
        return *top_val;
    }

    pub fn get_min(&self) -> i32 {
        let min_val = self.min_stack.last().unwrap();
        *min_val
    }
}
