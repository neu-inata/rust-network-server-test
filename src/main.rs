use std::net::TcpListener;
use std::io::Read;
use std::str;


fn main() {
    // リスナーの作成
    let result = TcpListener::bind("127.0.0.1:7777");
    if result.is_err() {
        println!("{}", result.err().unwrap().to_string());
        panic!();
    }
    println!("open!");
    let listener = result.unwrap();

    // 接続待ち
    let result = listener.accept();
    if result.is_err() {
        println!("{}", result.err().unwrap().to_string());
        panic!();
    }
    println!("connect!");
    let (mut stream, _) = result.unwrap();

    loop {
        let mut buf = [0u8; 1024];
        let res = stream.read(&mut buf);
        match res {
            Result::Ok(sz) => {
                // 切断確認
                if sz == 0{
                    println!("close!");
                    break;
                }
                println!("sz={} buf:{}", sz, str::from_utf8(&buf[..sz]).unwrap());
            },
            Result::Err(err) => {
                println!("{}", err.to_string());
                panic!();
            }
        }
    }
}
