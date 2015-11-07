extern crate image;

use image::{ImageBuffer, Luma};

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::mem;
mod header;

fn main() {
    //let path = Path::new();
    //let h = header::read_header(&path).unwrap();
    //output_slices(&h, path);
    //test_png();
    //println!("{:?}", h);
}

fn test_png() {
    let pix: Vec<u8> = vec![255u8; 100];

    image::save_buffer(&Path::new("test.png"), &pix, 10, 10, image::Gray(8));
}

#[allow(dead_code)]
fn output_slices(header: &header::NiftiHeader, path: &Path) {
    // Not autonomous just yet, requires marked fields
    let mut file = File::open(&path).unwrap();
    let mut offset: Vec<u8> = vec!(0u8; (header.vox_offset as usize));
    let mut slice_buf: Vec<u8> =  vec!(0u8; (header.dim[1] * header.dim[2] * 2) as usize); // 2 is the number of bytes per Voxel
    file.read(&mut offset);
    file.read(&mut slice_buf);
    for x in 0..header.dim[3] {
        file.read(&mut slice_buf);
        let buf_copy = slice_buf.clone();
        let slice_size = (header.dim[1] * header.dim[2]) as usize;
        let mut intensities = vec!(0i16; (header.dim[1] * header.dim[2]) as usize); // i16 is the datatype of each voxel
        intensities = unsafe { mem::transmute(buf_copy) };
        let mut pixvals = Vec::new();
        unsafe {
            for x in intensities.into_iter() {
                let pixval = x as u8; // should be out of 255, eventually will need to be calibrated to cal_max and cal_min
                pixvals.push(pixval);
            }
        }

        image::save_buffer(&Path::new(&format!("img/output{}.png", x)), &pixvals, header.dim[1] as u32, header.dim[2] as u32, image::Gray(8));

    }
}
