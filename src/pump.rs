use serialport::*;
use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::Duration;

const FIFO_SIZE: usize = 256;
const SLEEP_TIME: Duration = Duration::from_micros(100);

pub struct Pump {
    pub link: Box<dyn SerialPort>,
    limit: Option<usize>,
}

impl Pump {
    pub fn new(link: Box<dyn SerialPort>, limit: Option<usize>) -> Self {
        Pump { link, limit }
    }

    pub fn transfer(&mut self) -> io::Result<()> {
        let wait_ms = 1_024 / (self.link.baud_rate()? as u64 / 4_096);

        std::io::copy(&mut io::stdin(), &mut self.link)?;
        sleep(Duration::from_millis(wait_ms));
        std::io::copy(&mut self.link, &mut io::stdout())?;

        Ok(())
    }

    pub fn download<W: Write>(&mut self, writer: &mut W) -> io::Result<()> {
        let mut buf = [0; FIFO_SIZE / 4];
        let mut byte_counter = 0;
        loop {
            let rx_fifo = self.link.bytes_to_read()?;
            if self.link.timeout().as_millis() == 0 && rx_fifo == 0 {
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
                    byte_counter += chunk;
                    writer.write_all(&buf[..chunk])?;
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
            if self.link.timeout().as_millis() == 0 && tx_fifo > FIFO_SIZE as u32 / 2 {
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
                    byte_counter += chunk;
                    self.link.write_all(&buf[..chunk])?;
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
