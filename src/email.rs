pub mod email {
    pub fn send(from : String, to : String, title : String, body : String, 
        username : String, password : String, hostname : String) {
        use lettre::{Message, SmtpTransport, Transport};
        use lettre::transport::smtp::authentication::Credentials;

        let email = Message::builder()
            .from(format!("From <{}>", from).parse().unwrap())  // 发件人
            .to(format!("To <{}>", to).parse().unwrap())        // 收件人
            .subject(title)  // 主题
            .body(body.to_string())          // 邮件内容
            .unwrap();

        // 邮件服务器账号：
        // username 需包含 @xxx.com 等后缀
        // password 邮箱 -> 设置，开启 smtp -> 得到 授权密码
        let creds = Credentials::new(username, password);

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&hostname).unwrap().credentials(creds).build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => println!("Could not send email: {:?}", e),
        }
    }
}
