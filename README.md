# TCP Client-Server chat application for broadcasting messages / files / images.

### QuickStart:
```sh
# as server (default host:port -> 127.0.0.1:11111)
python3 run.py

# as server that accepts connection from any IP address & with more verbose logging messages
python3 run.py -l debug -o 0.0.0.0

# as client (default host:port 127.0.0.1:11111)
python3 run.py -m client

# as client connected to different host:port
python3 run.py -m client -o 10.0.0.5 -p 42069
```

### Prerequisities:
| Item | Link |
| ------ | ------ |
| Python 3 | <https://www.python.org/downloads/> |
| Rust | <https://www.rust-lang.org/tools/install> |


### Usage:
```sh
python3 run.py [OPTIONS]

Options:
  -m, --mode        <mode>          Mode of operation: server or client [default: server] [possible values: server, client]
  -o, --host        <host>          IPv4 address of the host [default: 127.0.0.1]
  -p, --port        <port>          Specifies a port [default: 11111]
  -l, --log-level   <log_level>     Specifies a log level [default: info] [possible values: trace, debug, info, warn, error]
  -h, --help                        Print help
```

### Logging:
- **tracing** & **tracing-subscriber** crates were used for logging
- **client**: log output -> saved to <client_crate>/logs/<date>.log
- **server**: log output -> stdout

## Major modifications (compared to previous homework)
- added logging
- code refactoring

### License
MIT
