use header;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::Cursor;
use std::mem;
use std::iter::Iterator;
use std::ops::{Sub, Div, Mul};

extern crate image;

extern crate byteorder;
use self::byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Default)]
pub struct Image {
    pub header: header::NiftiHeader,
    pub data: Vec<u8>,
}

// #[allow(dead_code)]
impl Image {
    pub fn new() -> Image {
        Image::default()
    }
    pub fn read_data(&mut self, path: &Path) {

        let mut file = File::open(&path).unwrap();
        file.seek(SeekFrom::Start(self.header.vox_offset as u64));

        let bitvox_custom = 16 as i64;
        let tot_vox = self.header.dim[1] * self.header.dim[2] * self.header.dim[3] * self.header.dim[4] * self.header.dim[5] * self.header.dim[6] * self.header.dim[7];

        let mut buffer: Vec<u8> = vec!(0u8; (tot_vox * bitvox_custom / 8) as usize);

        let mut voxs = vec![];
        for x in 0..tot_vox {
            match file.read_i16::<LittleEndian>() {
                Ok(n) => voxs.push(n),
                Err(_) => break,
            };
        }
        // self.data = voxs;
    }

    pub fn save_z_slice(&mut self, z: usize) {
        let slice_size = (self.header.dim[1] * self.header.dim[2]) as usize;
        let offset = slice_size * z;
        let c = &self.data[offset..(offset + slice_size)];
        let mut pixvals = vec![];
        for val in c {
            pixvals.push(*val as u8);
        }
        image::save_buffer(&Path::new(&format!("img/zslice{}.png", z)), &pixvals, self.header.dim[1] as u32, self.header.dim[2] as u32, image::Gray(8));
    }

    pub fn save_y_slice(&mut self, y: usize) {
        let mut poses = vec![];
        for x in 0..self.header.dim[1] {
            for z in 0..self.header.dim[3] {
                let pos = x as usize + y as usize * self.header.dim[1] as usize + z as usize * self.header.dim[1] as usize * self.header.dim[2] as usize;
                poses.push(self.data[pos] as u8)
            }

        }
        image::save_buffer(&Path::new(&format!("img/yslice{}.png", y)), &poses, self.header.dim[3] as u32, self.header.dim[1] as u32, image::Gray(8));
    }

    pub fn save_x_slice(&mut self, x: usize) {
        let mut poses = vec![];
        for y in 0..self.header.dim[2] {
            for z in 0..self.header.dim[3] {
                let pos = x as usize + y as usize * self.header.dim[1] as usize + z as usize * self.header.dim[1] as usize * self.header.dim[2] as usize;
                poses.push(self.data[pos] as u8)
            }

        }
        image::save_buffer(&Path::new(&format!("img/xslice{}.png", x)), &poses, self.header.dim[3] as u32, self.header.dim[2] as u32, image::Gray(8));
    }

    pub fn max_intensity(&self) -> u8 {
        *self.data.iter().max().unwrap()
    }
    pub fn min_intensity(&self) -> u8 {
        *self.data.iter().min().unwrap()
    }
    // pub fn normalise_data<T: Ord + Sub + Copy + Div>(&mut self, raw: Vec<T>) -> Vec<u8> {
    //     let max = raw.iter().max().unwrap();
    //     let min = raw.iter().min().unwrap();
    //     let range = *max - *min;
    //     for x in raw {
    //         let val = (((x - *min) / range) * 255) as u8;
    //         let a = x - *min;
    //         a = a / range;
    //     }
    //     vec![] //Placeholder
    // }

}

fn normalise_data<T>(raw: &[T]) -> Vec<u8>
where T: Div<Output=T>+Mul<u8,Output=T>+Sub<Output=T>+Into<u8>+Copy+Ord
{
    let max = raw.iter().max().unwrap();
    let min = raw.iter().min().unwrap();
    let range = *max - *min;
    raw.iter().map(|x| (((*x - *min) / range) * 255).into()).collect()
}

pub struct VoxelPosIterator {
    pos: Vec<i64>,
    dim: [i64; 8],
}
impl VoxelPosIterator {
    pub fn new(dim: [i64; 8]) -> VoxelPosIterator {
        VoxelPosIterator{pos: vec![0i64; dim[0] as usize], dim: dim}
    }
}
impl Iterator for VoxelPosIterator {
    type Item = Vec<i64>;
    fn next(&mut self) -> Option<Vec<i64>> {
        let result = Some(self.pos.clone());
        self.pos[0] += 1;
        for (k, v) in self.pos.clone().iter().enumerate() {
            if *v >= self.dim[k + 1] {
                self.pos[k] = 0;
                self.pos[k + 1] += 1;
            }
        }
        result
    }
}

#[derive(Debug, Default)]
pub struct Voxel {
    intensity: u8,
}
