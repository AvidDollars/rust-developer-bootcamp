# to format Rust crates all at one

import os
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).parent

root_dirs = filter(lambda entry: not entry.startswith("."), next(os.walk(ROOT))[1])

for dir in root_dirs:
    cargo_fmt = f"cargo fmt --manifest-path ./{dir}/Cargo.toml" 
    subprocess.run(cargo_fmt)