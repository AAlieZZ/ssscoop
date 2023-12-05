// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use powershell_script::PsScriptBuilder;
use std::fs::File;
use std::io::BufReader;

static PROXYUP: AtomicBool = AtomicBool::new(false);

#[derive(serde::Serialize, serde::Deserialize)]
struct ProxySet {
    ip: String,
    port: String,
    ciphers: String,
    password: String,
}

#[tauri::command]
fn setproxy(ip: &str, port: &str, ciphers: &str, password: &str) {
    let mut json_path = dirs::config_dir().unwrap();
    json_path.push("ssscoop/config.json");

    let proxyset = ProxySet {
        ip: String::from(ip),
        port: String::from(port),
        ciphers: String::from(ciphers),
        password: String::from(password),
    };

    let contents = serde_json::to_string(&proxyset).unwrap();
    match std::fs::write(&json_path, &contents) {
        Ok(()) => println!("Trying write {}", json_path.display()),
        Err(e) => {
            println!("{}", e);
            let mut mkdir = dirs::config_dir().unwrap();
            mkdir.push("ssscoop");
            std::fs::create_dir_all(mkdir).unwrap();
            std::fs::write(json_path, contents).unwrap()
        },
    }
}

#[tauri::command]
fn upproxy() {
    let mut json_path = dirs::config_dir().unwrap();
    json_path.push("ssscoop/config.json");
    let json_rdr = match File::open(json_path) {
        Ok(i) => BufReader::new(i),
        Err(e) => {
            eprintln!("无法启动代理：{}", e);
            return;
        },
    };
    let ss_set: ProxySet = match serde_json::from_reader(json_rdr) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("无法启动代理：{}", e);
            return;
        },
    };
    thread::spawn(move || {  //创建一个新的线程，并将 ss 从主线程移动到新创建的线程中等待子进程结束。
        let mut ss = Command::new("./ss/sslocal.exe")
            .arg("-b")
            .arg("127.0.0.1:3128")
            .arg("--protocol")
            .arg("http")
            .arg("-s")
            .arg(format!("{}:{}", ss_set.ip, ss_set.port))
            .arg("-m")
            .arg(ss_set.ciphers)
            .arg("-k")
            .arg(ss_set.password)
            .spawn()
            .expect("无法启动代理");
        PROXYUP.store(true, Ordering::Relaxed);
        while PROXYUP.load(Ordering::Relaxed) {}
        ss.kill().expect("警告：无法关闭 Shadow socks ！")
    });
}

#[tauri::command]
fn downproxy() {
    PROXYUP.store(false, Ordering::Relaxed)
}

#[tauri::command]
fn is_proxying() -> bool {
    PROXYUP.load(Ordering::Relaxed)
}

#[tauri::command]
fn pscmd(scmd: &str) -> String {
    let ps = PsScriptBuilder::new()
        .no_profile(false)
        .non_interactive(false)
        .hidden(false)
        .print_commands(true)
        .build();
    match ps.run(scmd) {
        Ok(output) => {
            return output.to_string();
        }
        Err(e) => {
            return format!("Error: {}", e);
        }
    }
}

fn main() {
    upproxy();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![setproxy, upproxy, downproxy, pscmd, is_proxying])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}