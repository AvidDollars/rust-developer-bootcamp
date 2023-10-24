use std::sync::Mutex;
use once_cell::sync::Lazy;

fn once_cell_example() {
    static CNT: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

    {
        // .lock(&self) -> interior mutability
        let mut cnt = CNT.lock().unwrap();
        *cnt += 1;
    }

    println!("{:?}", CNT);
}