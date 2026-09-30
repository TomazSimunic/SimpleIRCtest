use native_tls::TlsConnector;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn send(stream: &mut impl Write, msg: &str){
    write!(stream, "{msg}\r\n").unwrap();
    stream.flush().unwrap();
}

fn main() {
    let server = "irc.libera.chat";
    let port = "6697";
    let nick = "mybot";
    let channel = "#test";

    let connector = TlsConnector::new().unwrap();
    let tcp = TcpStream::connect(format!("{server}:{port}")).unwrap();
    let tls = connector.connect(server,tcp).unwrap();
    
    let mut reader = BufReader::new(tls);
    send(reader.get_mut(), &format!("NICK {nick}"));
    send(reader.get_mut(), &format!("USER {nick} 0 * :{nick}"));
    
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() > 0{
        print!("{line}");

        if line.starts_with("PING"){
            let pong = format!("PONG {}", line[4..].trim());
            send(reader.get_mut(), &pong);
        }

        if line.contains(" 001 "){
            send(reader.get_mut(), &format!("JOIN {channel}"));
            send(reader.get_mut(), &format!("PRIVMSG {channel} :Hello from Rust!"));
            send(reader.get_mut(), "QUIT :Bye");
            break;
        }

        line.clear();
    }
}
