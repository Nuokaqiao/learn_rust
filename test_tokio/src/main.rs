use std::time::Duration;

use chrono::Local;

use tokio::runtime::Builder;
fn main() {
    let rt = tokio::runtime::Runtime::new();
    std::thread::sleep(std::time::Duration::from_secs(10));

    // test_macro("3".to_string()).await; 没有tokio::main 宏的话，这种方式会出错
    // test_macro("3".to_string());
    // test_macro_mutil("no".to_string());
    // base_tokio_runtime();
}

fn base_tokio_runtime() {
    /*
    注意 enable_all 作用
    enable_all同时启用了IO和时间驱动程序，可通过enable_io、enable_time分别去启用它们
    如果在不启用时间驱动程序的运行时中调用tokio::time::sleep将发生 panic
     */
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S.%f");
    println!("0 Hello base tokio runtime {}", now);
    runtime.block_on(async {
        println!("will sleep 1 s");
        tokio::time::sleep(Duration::from_secs(1)).await;
        test_print("1".to_string()).await
    });
    runtime.block_on(async { test_print("2".to_string()).await });
}
// tokio::spawn生成一个新的异步任务，它将立即在后台运行，返回一个JoinHandle
// 不能保证生成的任务执行到完成。当运行时关闭时，不管该任务的生命周期如何，所有未完成的任务都将被丢弃
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn test_macro_mutil(n: String) {
    tokio::spawn(async {
        test_print_sleep("sleep".to_string()).await;
    });
    tokio::spawn(async {
        test_print("no sleep".to_string()).await;
    });
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("finish ")
}
#[tokio::main]
async fn test_macro(n: String) {
    test_print(n).await;
}

async fn test_print_sleep(n: String) {
    tokio::time::sleep(Duration::from_secs(10)).await;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S.%f");
    println!("{} Hello base tokio runtime {}", n, now);
}
async fn test_print(n: String) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S.%f");
    println!("{} Hello base tokio runtime {}", n, now);
}
