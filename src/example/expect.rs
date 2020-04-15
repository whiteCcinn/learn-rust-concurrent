// 线程异常
/*
默认情况下，主线程并不会等待子线程执行完毕。如果主线程创建完衍生线程就立即退出的话，那么子线程可能根本没机会开始运行或者执行完毕，

为了避免这种情况，我们可以让主线程等待子线程执行完毕然后再继续执行。

Rust 标准库提供了 join() 方法用于把子线程加入主线程等待队列。
*/
use std::thread;

#[allow(dead_code)]
pub fn handle() {
    let result = thread::spawn(move || {
        panic!("异常!");
    }).join();

    assert!(result.is_err());
}