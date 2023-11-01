mod email;

use std::env;
use base64::{Engine as _, engine::general_purpose};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 8 {
        panic!("用法: send_email <发送方邮箱> <接收方邮箱> <标题> <内容> <用户名> <密码(base64编码)> <smtp服务器(smtp.163.com)>");
    }
    let pw = general_purpose::STANDARD_NO_PAD.decode(args[6].clone()).unwrap();
    email::email::send(args[1].clone(),
        args[2].clone(),
        args[3].clone(),
        args[4].clone(),
        args[5].clone(),
        String::from_utf8(pw).unwrap(),
        args[7].clone());
}

// fn main() {
    // email::email::send("18609163202@163.com".to_string(),
    //     "1475875391@qq.com".to_string(),
    //     "Happy new year".to_string(),
    //     "Be happy!".to_string(),
    //     "18609163202@163.com".to_string(),
    //     "232425bb.".to_string(),
    //     "smtp.163.com".to_string());
// }
