use serialport::SerialPort;
use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::Duration;

const FIFO_SIZE: usize = 256;
const SLEEP_TIME: Duration = Duration::from_micros(100);

pub struct Pump {
    link: Box<dyn SerialPort>,
    limit: Option<usize>,
}

impl Pump {
    pub fn new(link: Box<dyn SerialPort>, limit: Option<usize>) -> Self {
        Pump { link, limit }
    }

    pub fn download<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let mut buf = [0; FIFO_SIZE / 4];
        let mut byte_counter = 0;
        loop {
            let rx_fifo = self.link.bytes_to_read()?;
            if rx_fifo == 0 {
                sleep(SLEEP_TIME);
                continue;
            }
            match self.link.read(&mut buf) {
                Ok(0) => continue,
                Ok(n) => {
                    let chunk = match self.limit {
                        Some(limit) => usize::min(n, limit - byte_counter),
                        _ => n,
                    };
                    writer.write_all(&buf[..chunk])?;
                    byte_counter += chunk;
                    writer.flush()?
                }
                Err(err) => return Err(err),
            }
            match self.limit {
                Some(limit) if limit == byte_counter => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }

    pub fn upload<R: Read>(&mut self, reader: &mut R) -> io::Result<()> {
        let mut buf = [0; FIFO_SIZE / 4];
        let mut byte_counter = 0;
        loop {
            let tx_fifo = self.link.bytes_to_write()?;
            if tx_fifo > FIFO_SIZE as u32 / 2 {
                sleep(SLEEP_TIME);
                continue;
            }
            match reader.read(&mut buf) {
                Ok(0) => return Ok(()),
                Ok(n) => {
                    let chunk = match self.limit {
                        Some(limit) => usize::min(n, limit - byte_counter),
                        _ => n,
                    };
                    self.link.write_all(&buf[..chunk])?;
                    byte_counter += chunk;
                    self.link.flush()?
                }
                Err(err) => return Err(err),
            }
            match self.limit {
                Some(limit) if limit == byte_counter => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}
