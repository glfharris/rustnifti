extern crate libc;

use self::libc::{c_int, c_char, c_double, c_short, c_float};
use std::mem;
use std::path::Path;
use std::fs::File;
// use std::error::Error;
use std::io::Read;

// pub fn generic_read_header(path: &Path) -> Result<NiftiHeader, String> {
//
//     let hsize = read_size(path);
//     let mut buf = vec![0u8; hsize as usize];
//     let mut file = File::open(path).unwrap();
//
//     file.read(&mut buf);
//
//     let header = match hsize {
//         540 => Ok(read_nifti2h(buf)),
//         348 => Ok(read_nifti1h(buf)),
//         val => Err("Shit went down".to_string()),//format!("Header type not recognised: {} bytes", val)),
//     };
//     header
// }
// pub fn read_nifti2h(buf: Vec<u8>) -> NiftiHeader {
//     let bhead: ByteNifti2Header = unsafe { mem::transmute_copy(&buf) };
//     NiftiHeader::from_nifti2_byte_header(bhead)
// }
// pub fn read_nifti1h(buf: Vec<u8>) -> NiftiHeader {
//     let bhead: ByteNifti1Header = unsafe { mem::transmute_copy(&buf) };
//     NiftiHeader::from_nifti1_byte_header(bhead)
// }

pub fn read_size(path: &Path) -> u64 {
    let mut file = File::open(path).unwrap();
    let mut buf = [0u8; 4];
    file.read(&mut buf).unwrap();
    let hsize: i32 = unsafe { mem::transmute(buf) };
    hsize as u64
}

pub fn read_header(path: &Path) -> Result<NiftiHeader, String> {
    let header = match read_size(path) {
        540 => read_nifti2_header(path),
        348 => read_nifti1_header(path),
        val => return Err(format!("Header size reported {} bytes", val)),
    };
    Ok(header)
}

pub fn read_nifti2_header(path: &Path) -> NiftiHeader {
    let mut file = File::open(path).unwrap();
    let mut buf = [0u8; 540];
    let bytes_read = file.read(&mut buf).unwrap();
    assert_eq!(bytes_read, buf.len());
    let byteh: ByteNifti2Header = unsafe { mem::transmute(buf) };
    NiftiHeader::from_nifti2_byte_header(byteh)
}


pub fn read_nifti1_header(path: &Path) -> NiftiHeader {
    let mut file = File::open(path).unwrap();
    let mut buf = [0u8; 348];
    let bytes_read = file.read(&mut buf).unwrap();
    assert_eq!(bytes_read, buf.len());
    let byteh: ByteNifti1Header = unsafe { mem::transmute(buf) };
    NiftiHeader::from_nifti1_byte_header(byteh)
}

#[repr(C)]
#[repr(packed)]
#[allow(dead_code)]
struct ByteNifti2Header {
    sizeof_hdr: c_int,
    magic: [c_char; 8],
    datatype: i16,
    bitpix: i16,
    dim: [i64; 8],
    intent_p1: c_double,
    intent_p2: c_double,
    intent_p3: c_double,
    pixdim: [c_double; 8],
    vox_offset: i64,
    scl_slope: c_double,
    scl_inter: c_double,
    cal_max: c_double,
    cal_min: c_double,
    slice_duration: c_double,
    toffset: c_double,
    slice_start: i64,
    slice_end: i64,
    descrip: [c_char; 80],
    aux_file: [c_char; 24],
    qform_code: c_int,
    sform_code: c_int,
    quatern_b: c_double,
    quatern_c: c_double,
    quatern_d: c_double,
    qoffset_x: c_double,
    qoffset_y: c_double,
    qoffset_z: c_double,
    srow_x: [c_double; 4],
    srow_y: [c_double; 4],
    srow_z: [c_double; 4],
    slice_code: c_int,
    xyzt_units: c_int,
    intent_code: c_int,
    intent_name: [c_char; 16],
    dim_info: c_char,
    unused_str: [c_char; 15],
    //should be 540 bytes and now is! make sure to use [repr(packed)]
}

#[repr(C)]
#[repr(packed)]
#[allow(dead_code)]
struct ByteNifti1Header {
    sizeof_hdr: c_int,
    data_type: [c_char; 10],
    db_name: [c_char; 18],
    extents: c_int,
    session_error: c_short,
    regular: c_char,
    dim_info: c_char,
    dim: [c_short; 8],
    intent_p1: c_float,
    intent_p2: c_float,
    intent_p3: c_float,
    intent_code: c_short,
    datatype: c_short,
    bitpix: c_short,
    slice_start: c_short,
    pixdim: [c_float; 8],
    vox_offset: c_float,
    scl_slope: c_float,
    scl_inter: c_float,
    slice_end: c_short,
    slice_code: c_char,
    xyzt_units: c_char,
    cal_max: c_float,
    cal_min: c_float,
    slice_duration: c_float,
    toffset: c_float,
    glmax: c_int,
    glmin: c_int,
    descrip: [c_char; 80],
    aux_file: [c_char; 24],
    qform_code: c_short,
    sform_code: c_short,
    quatern_b: c_float,
    quatern_c: c_float,
    quatern_d: c_float,
    qoffset_x: c_float,
    qoffset_y: c_float,
    qoffset_z: c_float,
    srow_x: [c_float; 4],
    srow_y: [c_float; 4],
    srow_z: [c_float; 4],
    intent_name: [c_char; 16],
    magic: [c_char; 4],
}

fn c_chars_to_string(chars: &[c_char]) -> String {
    let mut mediate = Vec::new();
    for x in chars.into_iter() {
        let y = *x as u8;
        mediate.push(y);
    }
    match String::from_utf8(mediate) {
        Ok(mystr) => mystr,
        Err(_) => panic!("Error in c_chars_to_string"),
    }
}

#[derive(Default)]
#[derive(Debug)]
pub struct NiftiHeader {
    pub sizeof_hdr: i32,
    pub magic: String,
    pub datatype: i16,
    pub bitpix: i16,
    pub dim: [i64; 8],
    pub intent_p1: f64,
    pub intent_p2: f64,
    pub intent_p3: f64,
    pub pixdim: [f64; 8],
    pub vox_offset: i64,
    pub scl_slope: f64,
    pub scl_inter: f64,
    pub cal_max: f64,
    pub cal_min: f64,
    pub slice_duration: f64,
    pub toffset: f64,
    pub slice_start: i64,
    pub slice_end: i64,
    pub descrip: String,
    pub aux_file: String,
    pub qform_code: i32,
    pub sform_code: i32,
    pub quatern_b: f64,
    pub quatern_c: f64,
    pub quatern_d: f64,
    pub qoffset_x: f64,
    pub qoffset_y: f64,
    pub qoffset_z: f64,
    pub srow_x: [f64; 4],
    pub srow_y: [f64; 4],
    pub srow_z: [f64; 4],
    pub slice_code: i32,
    pub xyzt_units: i32,
    pub intent_code: i32,
    pub intent_name: String,
    pub dim_info: char,
    pub unused_str: String,
}

impl NiftiHeader {
    fn from_nifti2_byte_header(b: ByteNifti2Header) -> NiftiHeader {
        let mut t: NiftiHeader = NiftiHeader::default();
        t.sizeof_hdr = b.sizeof_hdr as i32;
        t.magic = c_chars_to_string(&b.magic);
        t.datatype = b.datatype;
        t.bitpix = b.bitpix;
        t.dim = b.dim;
        t.intent_p1 = b.intent_p1 as f64;
        t.intent_p2 = b.intent_p2 as f64;
        t.intent_p3 = b.intent_p3 as f64;
        t.pixdim = b.pixdim as [f64; 8];
        t.vox_offset = b.vox_offset;
        t.scl_slope = b.scl_slope as f64;
        t.scl_inter = b.scl_inter as f64;
        t.cal_max = b.cal_max as f64;
        t.cal_min = b.cal_min as f64;
        t.slice_duration = b.slice_duration as f64;
        t.toffset = b.toffset as f64;
        t.slice_start = b.slice_start;
        t.slice_end = b.slice_end;
        t.descrip = c_chars_to_string(&b.descrip);
        t.aux_file = c_chars_to_string(&b.aux_file);
        t.qform_code = b.qform_code as i32;
        t.sform_code = b.sform_code as i32;
        t.quatern_b = b.quatern_b as f64;
        t.quatern_c = b.quatern_c as f64;
        t.quatern_d = b.quatern_d as f64;
        t.qoffset_x = b.qoffset_x as f64;
        t.qoffset_y = b.qoffset_y as f64;
        t.qoffset_z = b.qoffset_z as f64;
        t.srow_x = b.srow_x as [f64; 4];
        t.srow_y = b.srow_y as [f64; 4];
        t.srow_z = b.srow_z as [f64; 4];
        t.slice_code = b.slice_code as i32;
        t.xyzt_units = b.xyzt_units as i32;
        t.intent_code = b.intent_code as i32;
        t.intent_name = c_chars_to_string(&b.intent_name);
        t.dim_info = b.dim_info as u8 as char;
        t.unused_str = c_chars_to_string(&b.unused_str);
        t
    }

    fn from_nifti1_byte_header(b: ByteNifti1Header) -> NiftiHeader {
        let mut t = NiftiHeader::default();
        t.sizeof_hdr = b.sizeof_hdr as i32;
        t.dim_info = b.dim_info as u8 as char;
        t.dim = dim_32_to_64(b.dim);
        t.intent_p1 = b.intent_p1 as f64;
        t.intent_p2 = b.intent_p2 as f64;
        t.intent_p3 = b.intent_p3 as f64;
        t.intent_code = b.intent_code as i32;
        t.datatype = b.datatype as i16;
        t.bitpix = b.datatype as i16;
        t.slice_start = b.slice_start as i64;
        t.pixdim = pixdim_32_to_64(b.pixdim);
        t.vox_offset = b.vox_offset as i64;
        t.scl_slope = b.scl_slope as f64;
        t.scl_inter = b.scl_inter as f64;
        t.slice_end = b.slice_end as i64;
        t.slice_code = b.slice_start as i32;
        t.xyzt_units = b.xyzt_units as i32;
        t.cal_max = b.cal_max as f64;
        t.cal_min = b.cal_min as f64;
        t.slice_duration = b.slice_duration as f64;
        t.toffset = b.toffset as f64;
        t.descrip = c_chars_to_string(&b.descrip);
        t.aux_file = c_chars_to_string(&b.aux_file);
        t.qform_code = b.qform_code as i32;
        t.sform_code = b.sform_code as i32;
        t.quatern_b = b.quatern_b as f64;
        t.quatern_c = b.quatern_c as f64;
        t.quatern_d = b.quatern_d as f64;
        t.qoffset_x = b.qoffset_x as f64;
        t.qoffset_y = b.qoffset_y as f64;
        t.qoffset_z = b.qoffset_z as f64;
        t.srow_x = srow_32_to_64(b.srow_x);
        t.srow_y = srow_32_to_64(b.srow_y);
        t.srow_z = srow_32_to_64(b.srow_z);
        t.magic = c_chars_to_string(&b.magic);
        t.intent_name = c_chars_to_string(&b.intent_name);
        t
    }
}

fn dim_32_to_64(dim: [c_short; 8]) -> [i64; 8] {
    let mut double = [0i64; 8];
    for (i, x) in dim.iter().enumerate() {
        double[i] = *x as i64;
    }
    double
}

fn pixdim_32_to_64(pixdim: [c_float; 8]) -> [f64; 8] {
    let mut double = [0f64; 8];
    for (i, x) in pixdim.iter().enumerate() {
        double[i] = *x as f64;
    }
    double
}

fn srow_32_to_64(srow: [c_float; 4]) -> [f64; 4] {
    let mut double = [0f64; 4];
    for (i, x) in srow.iter().enumerate() {
        double[i] = *x as f64;
    }
    double
}
