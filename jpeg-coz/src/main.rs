use jpeg_decoder as jpeg;
use coz;
use std::env;
use std::io::{self, BufReader, Write};
use std::fs::File;


fn usage() -> ! {
    write!(io::stderr(), "usage: jpeg-coz image.jpg").unwrap();
    std::process::exit(1)
}

fn main() {
    coz::thread_init();
    let mut args = env::args().skip(1);
    let input_path = args.next().unwrap_or_else(|| usage());
    let input_file = File::open(input_path).expect("The specified input file could not be opened");
    coz::begin!("decode");
    for _ in 1..100 {
        coz::begin!("sleep");
        std::thread::sleep(std::time::Duration::from_millis(5));
        coz::end!("sleep");
    }
    let mut decoder = jpeg::Decoder::new(BufReader::new(input_file));
    let mut data = decoder.decode().expect("Decoding failed. If other software can successfully decode the specified JPEG image, then it's likely that there is a bug in jpeg-decoder");
    coz::end!("decode");
    //let info = decoder.info().unwrap();
}
