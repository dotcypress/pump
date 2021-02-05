use serialport::SerialPort;
use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::Duration;

const FIFO_SIZE: usize = 256;
const SLEEP_TIME: Duration = Duration::from_micros(100);

pub struct Pump {
    link: Box<dyn SerialPort>,
}

impl Pump {
    pub fn new(link: Box<dyn SerialPort>) -> Self {
        Pump { link }
    }

    pub fn download<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let mut buf = [0; FIFO_SIZE / 4];
        loop {
            let rx_fifo = self.link.bytes_to_read()?;
            if rx_fifo == 0 {
                sleep(SLEEP_TIME);
                continue;
            }
            match self.link.read(&mut buf) {
                Ok(0) => continue,
                Ok(n) => {
                    writer.write_all(&buf[..n])?;
                    writer.flush()?
                }
                Err(err) => return Err(err),
            }
        }
    }

    pub fn upload<R: Read>(&mut self, reader: &mut R) -> io::Result<()> {
        let mut buf = [0; FIFO_SIZE / 4];
        loop {
            let tx_fifo = self.link.bytes_to_write()?;
            if tx_fifo > FIFO_SIZE as u32 / 2 {
                sleep(SLEEP_TIME);
                continue;
            }
            match reader.read(&mut buf) {
                Ok(0) => return Ok(()),
                Ok(n) => {
                    self.link.write_all(&buf[..n])?;
                    self.link.flush()?
                }
                Err(err) => return Err(err),
            }
        }
    }
}
