// 只有实现Send接口的数据，才能够在线程间转移所有权
use std::sync::{Arc, Mutex};
use std::thread;
#[allow(dead_code)]
pub fn handle() {
    // Arc -> Mutex -> Vec
    // Arc 丝线了 Send 接口，所以可以被其他子线程转移所有权
    let data = Arc::new(Mutex::new(vec![1u32, 2, 3]));

    // 启动3个线程，修改外部的可变数据
    for i in 0..3 {
        // 先拷贝一份
        let data = data.clone();

        // 子线程修改外的可变数据
        thread::spawn(move || {
            // 进入临界区加锁
            let mut data = data.lock().unwrap();

            // 修改临界区的数据
            data[i] += 1;
        }).join().unwrap();
    }

    // 等待线程执行完毕，打印被修改的data数据
    thread::sleep(core::time::Duration::from_millis(50));
    println!("data = {:?}", data);
}