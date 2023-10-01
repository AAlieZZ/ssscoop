mod menu;
mod cmd;

use std::io::{self};
use menu::Menu;
use cmd::*;

pub const SS_ADDR: &str = "[::1]:8388";
pub const SS_KEY: &str = "hello-kitty";

fn get_options(i: &mut String) {
    println!("输入数字选项并回车：");
    io::stdin().read_line(i).unwrap();
}

fn get_name(i: &mut String) {
    println!("输入软件包名并回车：");
    io::stdin().read_line(i).unwrap();
}

fn package(p: &str) {
    let name = &mut String::new();
    get_name(name);
    pscmd(r#"scoop config proxy 127.0.0.1:3128"#);
    pscmd(&format!(r#"scoop {} {}"#, p, name.trim()));
    pscmd(r#"scoop config rm proxy"#);
}

fn main() {
    let mut input = String::new();
    init_ss();
    loop {
        let main_menu = Menu::new(vec![
            "搜索软件包".to_string(),
            "安装软件包".to_string(),
            "更新软件包".to_string(),
            "查看已安装的软件包".to_string(),
            "卸载软件包".to_string(),
            "添加 bucket".to_string(),
            "安装 scoop 包管理器".to_string(),
            "Install Scoop For Admin".to_string(),
        ]);
        main_menu.print_menu();
        get_options(&mut input);
        match &input.trim().parse().expect("非数字输入") {  // 用户输入可能会含有一些额外字符 如换行符 所以要用 trim 方法去除
            1 => package("search"),
            2 => package("install"),
            3 => package("update"),
            4 => pscmd(r#"scoop list"#),
            5 => package("uninstall"),
            6 => package(r#"bucket add"#),
            7 => {
                pscmd(r#"irm get.scoop.sh -Proxy 'http://127.0.0.1:3128' | iex"#);
                pscmd(r#"scoop config proxy 127.0.0.1:3128"#);
                pscmd(r#"scoop bucket add extras"#);
                pscmd(r#"scoop bucket add dorado"#);
                pscmd(r#"scoop update"#);
                pscmd(r#"scoop config rm proxy"#);
                println!("请退出并重新打开终端");
                break;
            },
            8 => {
                pscmd(r#"iex "& {$(irm get.scoop.sh -Proxy 'http://127.0.0.1:3128')} -RunAsAdmin""#);
                pscmd(r#"scoop config proxy 127.0.0.1:3128"#);
                pscmd(r#"scoop bucket add extras"#);
                pscmd(r#"scoop bucket add dorado https://github.com/chawyehsu/dorado"#);
                pscmd(r#"scoop update"#);
                pscmd(r#"scoop config rm proxy"#);
                println!("请退出并重新打开终端");
                break;
            },
            0 => break,
            _ => println!("选项不正确：{}", &input),
        }
        input.clear();
    }
}
