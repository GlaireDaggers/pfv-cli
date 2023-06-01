use std::fs::File;

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
use resize::Pixel::Gray8;
use rgb::FromSlice;

#[derive(Parser, Debug)]
#[command(author = "Hazel Stagner <glairedaggers@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Command line utility for encoding PFV video files", long_about = None)]
struct Args {
    #[arg(short = 'i')]
    inpath: String,
    
    #[arg(short = 'q')]
    quality: Option<i32>,

    #[arg(short = 'k')]
    keyframe_interval: Option<u32>,

    #[arg(short = 't')]
    threads: Option<i32>,

    #[arg(short = 'o')]
    outpath: String,
}

fn main() -> Result<(), ()> {
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

    let keyframe_interval = match cli.keyframe_interval {
        None => 30,
        Some(v) => v
    };

    let input_file = File::open(cli.inpath).unwrap();
    let mut y4m_dec = y4m::decode(input_file).unwrap();

    match y4m_dec.get_bit_depth() {
        8 => {}
        _ => {
            println!("Only 8bpp input supported");
            return Err(());
        }
    }

    let luma_w = y4m_dec.get_width();
    let luma_h = y4m_dec.get_height();

    let (chroma_w, chroma_h) = match y4m_dec.get_colorspace() {
        y4m::Colorspace::C420jpeg => {
            (luma_w / 2, luma_h / 2)
        }
        y4m::Colorspace::C420paldv => {
            (luma_w / 2, luma_h / 2)
        }
        y4m::Colorspace::C420mpeg2 => {
            (luma_w / 2, luma_h / 2)
        }
        y4m::Colorspace::C420 => {
            (luma_w / 2, luma_h / 2)
        }
        y4m::Colorspace::C422 => {
            (luma_w / 2, luma_h)
        }
        y4m::Colorspace::C444 => {
            (luma_w, luma_h)
        }
        _ => {
            println!("Only 4:2:0, 4:2:2, or 4:4:4 input supported");
            return Err(());
        }
    };
    
    let mut chroma_resizer = resize::new(chroma_w, chroma_h,
        luma_w, luma_h,
        Gray8, resize::Type::Lanczos3).unwrap();

    let fr = y4m_dec.get_framerate();

    if fr.num % fr.den != 0 {
        println!("Fractional framerates not supported");
        return Err(());
    }

    let framerate = (fr.num / fr.den) as u32;

    let outfile = File::create(cli.outpath).unwrap();
    let mut enc = Encoder::new(outfile, luma_w, luma_h, framerate, q, threads as usize).unwrap();

    let mut tmp_buf_u = vec![0;luma_w * luma_h];
    let mut tmp_buf_v = vec![0;luma_w * luma_h];

    let mut outframe = 0;

    loop {
        match y4m_dec.read_frame() {
            Ok(frame) => {
                chroma_resizer.resize(frame.get_u_plane().as_gray(), &mut tmp_buf_u.as_gray_mut()).unwrap();
                chroma_resizer.resize(frame.get_v_plane().as_gray(), &mut tmp_buf_v.as_gray_mut()).unwrap();

                let plane_y = VideoPlane::from_slice(luma_w, luma_h, frame.get_y_plane());
                let plane_u = VideoPlane::from_slice(luma_w, luma_h, &tmp_buf_u);
                let plane_v = VideoPlane::from_slice(luma_w, luma_h, &tmp_buf_v);

                let fr = VideoFrame::from_planes(luma_w, luma_h, plane_y, plane_u, plane_v);

                if (outframe % keyframe_interval) == 0 {
                    enc.encode_iframe(&fr).unwrap();
                } else {
                    enc.encode_pframe(&fr).unwrap();
                }

                outframe += 1;

                execute!(
                    stdout(),
                    Clear(ClearType::CurrentLine),
                    MoveToColumn(0),
                    Print(format!("Encoded: {}", outframe)),
                ).unwrap();
            }
            Err(e) => match e {
                y4m::Error::EOF => {
                    break;
                }
                _ => {
                    println!("Error reading input: {:?}", e);
                    return Err(());
                }
            }
        }
    }

    execute!(
        stdout(),
        Print("\nFinished encoding!\n"),
    ).unwrap();

    enc.finish().unwrap();
    return Ok(());
}
