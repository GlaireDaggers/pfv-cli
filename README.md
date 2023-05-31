# pfv-cli
CLI tool for encoding PFV video files

## Installation

```
cargo install pfv-cli
```

## Usage

Takes as input a folder containing named PNG files (as 001.png, 002.png, etc) and produces an encoded PFV video file

```
Usage: pfv-cli.exe [OPTIONS] -i <FRAMEPATH> -n <NUMFRAMES> -f <FPS> -o <OUTPATH>

Options:
  -i <FRAMEPATH>
  -n <NUMFRAMES>
  -f <FPS>
  -q <QUALITY>
  -k <KEYFRAME_INTERVAL>
  -t <THREADS>
  -o <OUTPATH>
  -h, --help          Print help
  -V, --version       Print version
```