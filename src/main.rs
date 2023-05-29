use std::{path::Path, fs::File};

use std::io::stdout;

use crossterm::cursor::MoveToColumn;
use crossterm::terminal::ClearType;
use crossterm::{
    execute,
    style::Print,
    terminal::Clear,
};

use clap::Parser;
use pfv_rs::{enc::Encoder, frame::VideoFrame, plane::VideoPlane};
use image::{io::Reader as ImageReader};

#[derive(Parser, Debug)]
#[command(author = "Hazel Stagner <glairedaggers@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Command line utility for encoding PGV video files", long_about = None)]
struct Args {
    #[arg(short = 'i')]
    framepath: String,
    
    #[arg(short = 'n')]
    numframes: u32,

    #[arg(short = 'f')]
    fps: u32,

    #[arg(short = 'q')]
    quality: Option<i32>,

    #[arg(short = 't')]
    threads: Option<i32>,

    #[arg(short = 'o')]
    outpath: String,
}

fn load_frame<Q: AsRef<Path>>(path: Q) -> VideoFrame {
    let src_img = ImageReader::open(path).unwrap().decode().unwrap().into_rgb8();
    
    let yuv_pixels: Vec<[u8;3]> = src_img.pixels().map(|rgb| {
        // https://en.wikipedia.org/wiki/YCbCr - "JPEG Conversion"
        let y = (0.299 * rgb.0[0] as f32) + (0.587 * rgb.0[1] as f32) + (0.114 * rgb.0[2] as f32);
        let u = 128.0 - (0.168736 * rgb.0[0] as f32) - (0.331264 * rgb.0[1] as f32) + (0.5 * rgb.0[2] as f32);
        let v = 128.0 + (0.5 * rgb.0[0] as f32) - (0.418688 * rgb.0[1] as f32) - (0.081312 * rgb.0[2] as f32);
        [y as u8, u as u8, v as u8]
    }).collect();

    // split into three planes
    let y_buffer: Vec<_> = yuv_pixels.iter().map(|x| x[0]).collect();
    let u_buffer: Vec<_> = yuv_pixels.iter().map(|x| x[1]).collect();
    let v_buffer: Vec<_> = yuv_pixels.iter().map(|x| x[2]).collect();

    let y_plane = VideoPlane::from_slice(src_img.width() as usize, src_img.height() as usize, &y_buffer);
    let u_plane = VideoPlane::from_slice(src_img.width() as usize, src_img.height() as usize, &u_buffer);
    let v_plane = VideoPlane::from_slice(src_img.width() as usize, src_img.height() as usize, &v_buffer);

    VideoFrame::from_planes(src_img.width() as usize, src_img.height() as usize, y_plane, u_plane, v_plane)
}

fn main() {
    let cli = Args::parse();

    let q = match cli.quality {
        None => 5,
        Some(v) => {
            if v < 0 || v > 10 {
                println!("Quality must be between 0 and 10. Using default quality (5)");
                5
            } else {
                v
            }
        }
    };

    let threads = match cli.threads {
        None => 8,
        Some(v) => {
            if v < 0 {
                println!("Threads must be >0. Using default threads (8)");
                8
            } else {
                v
            }
        }
    };

    // read first image from path
    let frame0 = load_frame(format!("{}/001.png", cli.framepath));

    let mut enc = Encoder::new(frame0.width, frame0.height, cli.fps, q, threads as usize);

    // encode frames
    enc.encode_iframe(&frame0);

    execute!(
        stdout(),
        Print(format!("Encoded: 1 / {}", cli.numframes)),
    ).unwrap();
    
    for i in 1..cli.numframes {
        let framepath = format!("{}/{:0>3}.png", cli.framepath, i + 1);
        let frame = load_frame(framepath);

        if i % 30 == 0 {
            enc.encode_iframe(&frame);
        } else {
            enc.encode_pframe(&frame);
        }

        execute!(
            stdout(),
            Clear(ClearType::CurrentLine),
            MoveToColumn(0),
            Print(format!("Encoded: {} / {}", i + 1, cli.numframes)),
        ).unwrap();
    }

    execute!(
        stdout(),
        Print("\nFinished encoding!\n"),
    ).unwrap();

    let mut outfile = File::create(cli.outpath).unwrap();
    enc.write(&mut outfile).unwrap();
}
