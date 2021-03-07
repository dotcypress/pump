# pump

⛽️ Serial port pump.

## Installation

```bash
$ cargo install pump
```

## Usage

```bash
$ pump --help
```

```
USAGE:
    pump [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    download    Download from serial port [aliases: down]
    help        Prints this message or the help of the given subcommand(s)
    list        List available ports [aliases: ls]
    upload      Upload to serial port [aliases: up]
```

### List serial ports

```bash
$ pump list --help
```

```
List available ports

USAGE:
    pump list [FLAGS]

FLAGS:
    -i, --info       Prints detailed ports information
    -h, --help       Prints help information
    -V, --version    Prints version information
```

### Upload to serial port

```bash
$ pump upload --help
```

```
Upload to serial port

USAGE:
    pump upload [OPTIONS] <PORT> [BAUDRATE]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --data-bits <DATA_BITS>    Sets the data bits [default: 8]  [possible values: 5, 6, 7, 8]
    -f, --flow <FLOW>              Sets the flow control [env: PUMP_FLOW=]  [default: off]  [possible values: off, soft]
    -i, --input <INPUT>            Sets the input file  [default: stdin]
    -l, --limit <LIMIT>            Sets the data limit in bytes
    -p, --parity <PARITY>          Sets the parity [default: none]  [possible values: none, odd, even]
    -s, --stop-bits <STOP_BITS>    Sets the stop bits [default: 1]  [possible values: 1, 2]
    -t, --timeout <TIMEOUT>        Sets the timeout in milliseconds [default: 0]

ARGS:
    <PORT>        Sets thes port name [env: PUMP_PORT=]
    <BAUDRATE>    Sets the baudrate [env: PUMP_BAUDRATE=]  [default: 115200]
```

### Download from serial port

```bash
$ pump download --help
```

```
Download from serial port

USAGE:
    pump download [OPTIONS] <PORT> [BAUDRATE]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --data-bits <DATA_BITS>    Sets the data bits [default: 8]  [possible values: 5, 6, 7, 8]
    -f, --flow <FLOW>              Sets the flow control [env: PUMP_FLOW=]  [default: off]  [possible values: off, soft]
    -o, --output <OUTPUT>          Sets the output file  [default: stdout]
    -l, --limit <LIMIT>            Sets the data limit in bytes
    -p, --parity <PARITY>          Sets the parity [default: none]  [possible values: none, odd, even]
    -s, --stop-bits <STOP_BITS>    Sets the stop bits [default: 1]  [possible values: 1, 2]
    -t, --timeout <TIMEOUT>        Sets the timeout in milliseconds [default: 0]

ARGS:
    <PORT>        Sets thes port name [env: PUMP_PORT=]
    <BAUDRATE>    Sets the baudrate [env: PUMP_BAUDRATE=]  [default: 115200]
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.