pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars = vec![];
    for i in 0..position.len() {
        cars.push((position[i], speed[i]))
    }
    cars.sort();
    let mut stack = vec![];
    for i in (0..position.len()).rev() {
        let car = cars[i];
        if stack.is_empty() {
            stack.push(car);
        } else {
            let cur_car = *stack.last().unwrap();
            let time_remaining_cur: f32 = (target - cur_car.0) as f32 / cur_car.1 as f32;
            let time_remaining = (target - car.0) as f32 / car.1 as f32;
            if time_remaining > time_remaining_cur {
                stack.push(car);
            }
        }
    }
    stack.len() as i32
}
