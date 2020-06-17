extern crate img_parts;

use img_parts::jpeg::Jpeg;
use img_parts::ImageICC;
use img_parts::ImageEXIF;

use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

fn main() {	
    let input = if env::args().count() >= 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let output = if env::args().count() == 3 {
        env::args().nth(2).unwrap()
    } else {
        input.clone()
    };

    let file = fs::read(format!("tests/images/{}", input)).expect("read jpeg");

	let jpeg = Jpeg::read(&mut &file[..]).expect("parse jpeg");

	let icc_profile = jpeg.icc_profile();
	println!("EXIF: {:?}", icc_profile);

	let exif_meta = jpeg.exif();
	println!("ICC: {:?}", exif_meta);

    let file_stem = Path::new(&input).file_stem();
    let out_path = format!("{}{}.jpg", output, file_stem.unwrap().to_str().unwrap());
	println!("{}", out_path);
	
	let fout = &mut File::create(&Path::new(&out_path)).unwrap();
	jpeg.write_to(fout).expect("write jpeg");

}