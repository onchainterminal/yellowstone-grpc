use std::io::{Write, BufWriter};
use std::net::TcpStream;
use std::error::Error;

pub struct TcpCommandClient {
    writer: BufWriter<TcpStream>,
}

impl TcpCommandClient {
    /// Connect to the command server
    pub fn connect(addr: &str) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(addr)?;
        let writer = BufWriter::new(stream);
        Ok(TcpCommandClient { writer })
    }

    /// Send a "go" command
    pub fn send_go(&mut self) -> Result<(), Box<dyn Error>> {
        self.send_command("go")
    }

    /// General send command method
    pub fn send_command(&mut self, cmd: &str) -> Result<(), Box<dyn Error>> {
        writeln!(self.writer, "{}", cmd)?;
        self.writer.flush()?;
        Ok(())
    }
}
