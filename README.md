# pfv-cli
CLI tool for encoding PFV video files

## Installation

```
cargo install pfv-cli
```

## Usage

Takes as input a .Y4M video and produces an encoded PFV video file

Currently only 8bpp input supported, and only whole number framerates supported.

```
Usage: pfv-cli.exe [OPTIONS] -i <FRAMEPATH> -n <NUMFRAMES> -f <FPS> -o <OUTPATH>

Options:
  -i <INPATH>
  -q <QUALITY>
  -k <KEYFRAME_INTERVAL>
  -t <THREADS>
  -o <OUTPATH>
  -h, --help          Print help
  -V, --version       Print version
```