use std::io::Write;
use std::net::TcpStream;

pub fn send(data: &[u8]) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(env!("send_to"))?;
    let _ = stream.write(data)?;
    Ok(())
}
