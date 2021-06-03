use std::net::{TcpStream, TcpListener};
use std::io::Read;
use std::str;
use std::thread;
use std::sync::{Arc, Mutex};
use std::mem;

fn main() -> std::io::Result<()> {
    
    let stream: TcpStream;
    unsafe{
        stream = mem::uninitialized();
    }
    let stream = Arc::new(Mutex::new(stream));
    let stream_clone = stream.clone();

    // 接続待ち
    let handle = thread::spawn(move ||{
        // リスナーの作成
        let listener = TcpListener::bind("127.0.0.1:7777");
        let listener = listener.unwrap();

        let mut s = stream_clone.lock().unwrap();
        let result = listener.accept();
        *s = result.unwrap().0;
    });
    let _ = handle.join();
    let mut stream = stream.as_ref().lock().unwrap();

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
                break;
            }
        }
    }
    Ok(())
}
