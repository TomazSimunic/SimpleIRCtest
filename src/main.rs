use native_tls::TlsConnector;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct IrcMessage{
    prefix: Option<String>,
    command: String,
    params: Vec<String>,
    trailing: Option<String>
}

fn parse_irc_message(line: &str) -> Option<IrcMessage>{
    let mut s = line.trim_end_matches(['\r','\n']);

    let prefix: Option<String>;
    if s.starts_with(':') {
        let end = s.find(' ')?;
        prefix = Some(s[1..end].to_string());
        s = &s[end+1..];
    } else {
        prefix = None;
    };

    let trailing: Option<String>;
    if let Some(pos) = s.find(" :"){
        trailing  = Some(s[pos+2..].to_string());
        s=&s[..pos];
    } else{
        trailing = None;
    };

    let mut parts = s.split_whitespace();

    let command = parts.next()?.to_string();
    let params = parts.map(String::from).collect();

    Some(IrcMessage{
        prefix,
        command,
        params,
        trailing,
    })

}
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

        if let Some(msg) = parse_irc_message(&line){
            println!("Parsed message: {msg:?}");

        }

        line.clear();
    }
}
