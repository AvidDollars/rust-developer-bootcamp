import argparse
import subprocess

class CliArgs:
    def __init__(self, mode: str, host: str, port: str, level: str):
        self.mode = self.validate_mode(mode)
        self.host = self.validate_host(host)
        self.port = self.validate_port(port)
        self.log_level = self.validate_log_level(level)

    def __repr__(self):
        return f"CliArgs(mode={self.mode}, host={self.host}, port={self.port}, log_level: {self.log_level})"

    @staticmethod
    def validate_mode(mode: str) -> str:
        if mode is None:
            return "server"

        if mode not in ('client', 'server'):
            raise ValueError("Invalid mode. Allowed are: 'client' | 'server'.")

        return mode

    @staticmethod
    def validate_host(host: str) -> str:
        error = ValueError("Invalid IPv4 address.")

        if host is None:
            return "127.0.0.1"

        bytes_ = host.split(".")

        if len(bytes_) != 4:
            raise error

        try:
            bytes_ = [int(byte) for byte in bytes_]

        except ValueError:
            raise error

        if any(num < 0 or num > 255 for num in bytes_):
            raise error

        return ".".join((str(byte) for byte in bytes_))

    @staticmethod
    def validate_port(port: str) -> int:
        error = ValueError(f"Invalid port. Use port in range of 0 to {2**16 - 1}.")

        if port is None:
            return 11_111

        try:
            port = int(port)

            if port < 0 or port >= 2**16:
                raise error

        except ValueError:
            raise error

        return port

    @staticmethod
    def validate_log_level(level: str) -> str:
        allowed = ("trace", "debug", "info", "warn", "error")
        error = ValueError(f"Invalid log level. Allowed are: {' | '.join(allowed)}.")

        if level is None:
            return "info"

        level = level.strip()

        if level not in allowed:
            raise error
        
        return level.upper()


def call_subprocess(args: CliArgs):
    cargo_run = f"cargo run --manifest-path ./{args.mode}/Cargo.toml --release"
    args = f"--mode {args.mode} --host {args.host} --port {args.port} --log-level {args.log_level}" 
    subprocess.run(" -- ".join((cargo_run, args)))

def parse_cli_args() -> CliArgs:

    parser = argparse.ArgumentParser(
        prog='ChatApp',
        description='TCP Client-Server chat application for broadcasting messages / files / images.',
    )

    parser.add_argument('-m', '--mode') 
    parser.add_argument('-o', '--host')
    parser.add_argument('-p,', '--port')
    parser.add_argument('-l', '--log-level')
    
    args = parser.parse_args()
    return CliArgs(args.mode, args.host, args.port, args.log_level)
    
if __name__ == "__main__":
    try:
        cli_args = parse_cli_args()      
        call_subprocess(cli_args) 

    except ValueError as error:
        print(error)

    except KeyboardInterrupt:
        print("process interrupted")

    except Exception as error:
        print(f"unexpected error occurred: {error}")