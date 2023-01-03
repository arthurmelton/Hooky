use std::net::TcpStream;
use std::io::Write;

pub fn send(data: &[u8]) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(env!("send_to"))?;
    stream.write(data)?;
    Ok(())
}
