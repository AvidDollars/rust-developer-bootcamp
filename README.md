### Prerequisities:
| Item | Link |
| ------ | ------ |
| Python 3 | <https://www.python.org/downloads/> |
| Rust | <https://www.rust-lang.org/tools/install> |


### Usage:
```sh
python3 run.py [OPTIONS]

Options:
    -m, --mode <mode>  Mode of operation [possible values: client, server (default)].
    -o, --host <host>  IPv4 address of the host [default: 127.0.0.1].
    -p, --port <port>  Specifies a port [default: 11111].
    -h, --help         Print help
```

## License
MIT

### TODOs:
```markdown
- run.py -> subprocess.run(..., shell=True, check=True) (check: https://docs.python.org/3/library/subprocess.html)
- client -> files & images folder to be created inside client folder
- shared -> remake TryFrom<&mut TcpStream> for Message
```