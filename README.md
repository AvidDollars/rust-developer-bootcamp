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
  -l, --log-level   <log_level>     Specifies a log level [default: info]
  -h, --help                        Print help
```

### Logging:
**tracing** & **tracing-subscriber** crates were used for logging
**Client**: log output -> saved to <client_folder>/logs/<date>.log
**Server**: log output -> stdout

### License
MIT

### TODOs:
```markdown
- run.py -> subprocess.run(..., shell=True, check=True) (check: https://docs.python.org/3/library/subprocess.html)
```