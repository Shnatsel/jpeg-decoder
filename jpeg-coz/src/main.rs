use jpeg_decoder as jpeg;
use coz;
use std::env;
use std::io::{self, BufReader, Write, Read, Cursor};
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
    let mut input_data: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(input_file);
    reader.read_to_end(&mut input_data).unwrap();
    for _ in 1..5000 {
        coz::scope!("decode");
        let mut decoder = jpeg::Decoder::new(Cursor::new( &mut input_data));
        let _data = decoder.decode().expect("Decoding failed. If other software can successfully decode the specified JPEG image, then it's likely that there is a bug in jpeg-decoder");
    }
    //let info = decoder.info().unwrap();
}
