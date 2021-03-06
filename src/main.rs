use clap::{App, Arg};
use core::time::Duration;
use serialport::{DataBits, FlowControl, Parity, SerialPortType, StopBits};
use std::fs::File;
use std::io::{self, Error, ErrorKind};

mod pump;
use pump::*;

fn main() -> io::Result<()> {
    let serial_args = vec![
        Arg::with_name("PORT")
            .help("Sets thes port name")
            .value_name("PORT")
            .env("PUMP_PORT")
            .required(true),
        Arg::with_name("BAUDRATE")
            .help("Sets the baudrate")
            .value_name("BAUDRATE")
            .env("PUMP_BAUDRATE")
            .default_value("115200"),
        Arg::with_name("TIMEOUT")
            .help("Sets the timeout in milliseconds")
            .value_name("TIMEOUT")
            .default_value("0")
            .long("timeout")
            .short("t"),
        Arg::with_name("FLOW")
            .help("Sets the flow control")
            .value_name("FLOW")
            .env("PUMP_FLOW")
            .default_value("off")
            .possible_values(&["off", "soft"])
            .long("flow")
            .short("f"),
        Arg::with_name("PARITY")
            .help("Sets the parity")
            .value_name("PARITY")
            .default_value("none")
            .possible_values(&["none", "odd", "even"])
            .long("parity")
            .short("p"),
        Arg::with_name("DATA_BITS")
            .help("Sets the data bits")
            .value_name("DATA_BITS")
            .default_value("8")
            .possible_values(&["5", "6", "7", "8"])
            .long("data-bits")
            .short("d"),
        Arg::with_name("STOP_BITS")
            .help("Sets the stop bits")
            .value_name("STOP_BITS")
            .default_value("1")
            .possible_values(&["1", "2"])
            .long("stop-bits")
            .short("s"),
        Arg::with_name("LIMIT")
            .help("Sets the data limit in bytes")
            .value_name("LIMIT")
            .long("limit")
            .short("l"),
    ];

    let mut app = App::new("pump")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Vitaly Domnikov <oss@vitaly.codes>")
        .about("Serial port pump")
        .subcommand(
            App::new("list")
                .visible_alias("ls")
                .about("List available ports")
                .arg(
                    Arg::with_name("PHY")
                        .help("Prints detailed ports information")
                        .long("info")
                        .short("i"),
                ),
        )
        .subcommand(
            App::new("upload")
                .visible_alias("up")
                .about("Upload to serial port")
                .args(&serial_args)
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file")
                        .value_name("INPUT")
                        .long("input")
                        .short("i"),
                ),
        )
        .subcommand(
            App::new("download")
                .visible_alias("down")
                .about("Download from serial port")
                .args(&serial_args)
                .arg(
                    Arg::with_name("OUTPUT")
                        .help("Sets the output file")
                        .value_name("OUTPUT")
                        .long("output")
                        .short("o"),
                ),
        );

    let res = match app.clone().get_matches().subcommand() {
        ("list", Some(args)) => list_ports(args.is_present("PHY")),
        ("upload", Some(args)) => match create_pump(args) {
            Ok(mut pump) => match args.value_of("INPUT") {
                Some(path) => pump.upload(&mut File::open(path)?),
                None => pump.upload(&mut io::stdin()),
            },
            Err(err) => Err(Error::new(ErrorKind::NotFound, err)),
        },
        ("download", Some(args)) => match create_pump(args) {
            Ok(mut pump) => match args.value_of("OUTPUT") {
                Some(path) => pump.download(&mut File::create(path)?),
                None => pump.download(&mut io::stdout()),
            },
            Err(err) => Err(Error::new(ErrorKind::NotFound, err)),
        },
        _ => app
            .print_long_help()
            .map_err(|err| Error::new(ErrorKind::Other, err)),
    };

    match res {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            ErrorKind::BrokenPipe => Ok(()),
            _ => Err(err),
        },
    }
}

fn list_ports(print_phy: bool) -> io::Result<()> {
    let ports =
        serialport::available_ports().map_err(|err| Error::new(ErrorKind::NotFound, err))?;
    for p in ports {
        if print_phy {
            let phy_info = match p.port_type {
                SerialPortType::BluetoothPort => "  - Port Type: Bluetooth".to_string(),
                SerialPortType::PciPort => "  - Port Type: PCI".to_string(),
                SerialPortType::Unknown => "  - Port Type: Unknown".to_string(),
                SerialPortType::UsbPort(usb) => format!(
          "  - Port Type: USB [VID: {}, PID: {}]\n  - Manufacturer: {}\n  - Product: {}\n  - Serial: {}",
          usb.vid,
          usb.pid,
          usb.manufacturer.unwrap_or_default(),
          usb.product.unwrap_or_default(),
          usb.serial_number.unwrap_or_default(),
        ),
            };
            println!("{}\n{}\n", p.port_name, phy_info);
        } else {
            println!("{}", p.port_name);
        };
    }
    Ok(())
}

fn create_pump(args: &clap::ArgMatches) -> Result<Pump, String> {
    let port_name = args.value_of("PORT").ok_or("Invalid port name")?;
    let baud_rate = args
        .value_of("BAUDRATE")
        .unwrap()
        .parse::<u32>()
        .map_err(|err| format!("Invalid baudrate: {}", err))?;
    let timeout = args
        .value_of("TIMEOUT")
        .unwrap()
        .parse::<u64>()
        .map_err(|err| format!("Invalid timeout: {}", err))?;
    let flow_control = args
        .value_of("FLOW")
        .map(|f| match f {
            "soft" | "s" => FlowControl::Software,
            _ => FlowControl::None,
        })
        .unwrap();
    let parity = args
        .value_of("PARITY")
        .map(|f| match f {
            "odd" => Parity::Odd,
            "even" => Parity::Even,
            _ => Parity::None,
        })
        .unwrap();
    let data_bits = args
        .value_of("DATA_BITS")
        .map(|f| match f {
            "5" => DataBits::Five,
            "6" => DataBits::Six,
            "7" => DataBits::Seven,
            _ => DataBits::Eight,
        })
        .unwrap();
    let stop_bits = args
        .value_of("STOP_BITS")
        .map(|f| match f {
            "2" => StopBits::Two,
            _ => StopBits::One,
        })
        .unwrap();

    let limit = match args.value_of("LIMIT") {
        None => None,
        Some(lim) => {
            let limit = lim
                .parse::<u64>()
                .map_err(|err| format!("Invalid limit: {}", err))?;
            Some(limit as usize)
        }
    };

    serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(timeout))
        .flow_control(flow_control)
        .parity(parity)
        .data_bits(data_bits)
        .stop_bits(stop_bits)
        .open()
        .map(|link| Pump::new(link, limit))
        .map_err(|err| format!("Failed to open serial port: {}", err))
}
