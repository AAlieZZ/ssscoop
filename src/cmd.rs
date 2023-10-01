use std::process::Command;
use std::thread;
use powershell_script::PsScriptBuilder;

pub fn init_ss() -> thread::JoinHandle<std::process::Child> {
    thread::spawn( || {  //创建一个新的线程，并将 ss 从主线程移动到新创建的线程中等待子进程结束。
        Command::new("./ss/sslocal.exe")
        .arg("-b")
        .arg("127.0.0.1:3128")
        .arg("--protocol")
        .arg("http")
        .arg("-s")
        .arg(crate::SS_ADDR)
        .arg("-m")
        .arg("aes-128-gcm")
        .arg("-k")
        .arg(crate::SS_KEY)
        .spawn()
        .expect("无法启动代理")
    })

    // ss_handle.join().expect("线程错误");    //在主程序退出之前使用 join 方法等待新创建的线程结束。
                             
    //ss.kill().expect("警告：无法关闭 Shadow socks ！")
}

pub fn pscmd(cmd: &str) {
    let ps = PsScriptBuilder::new()
        .no_profile(false)
        .non_interactive(false)
        .hidden(false)
        .print_commands(true)
        .build();
    match ps.run(cmd) {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}