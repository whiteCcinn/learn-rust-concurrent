use std::thread;

pub fn handle() {
    // 启动子线程

    thread::spawn(|| {
        println!("Hello from a thread!");
    });

    // 死循环卡住主线程，否则主线程挂掉，上面的子线程也跟着挂掉
    // 也就无法看到子线程的输出
    loop {}
}