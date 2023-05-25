# pfv-cli
CLI tool for encoding PFV video files

## Installation

```
cargo install pfv-cli
```

## Usage

Takes as input a folder containing named PNG files (as 001.png, 002.png, etc) and an optional WAV audio track, and produces an encoded PFV video file

```
Usage: pfv-cli.exe [OPTIONS] -i <FRAMEPATH> -n <NUMFRAMES> -f <FPS> -o <OUTPATH>

Options:
  -i <FRAMEPATH>
  -n <NUMFRAMES>
  -f <FPS>
  -a <AUDIOPATH>
  -q <QUALITY>
  -t <THREADS>
  -o <OUTPATH>
  -h, --help          Print help
  -V, --version       Print version
```