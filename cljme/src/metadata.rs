use crate::errors::Error;
use crate::file::{copy_data, copy_header};
use crate::utils::{read, read_u16, skip};
use std::io::{Read, Seek, Write};

pub fn delete_metadata<R: Read + Seek, W: Write>(
    source: &mut R,
    destination: &mut W,
) -> Result<(), Error> {
    loop {
        let mut marker: [u8; 2] = [0; 2];
        if !read(source, &mut marker)? {
            break;
        }

        match marker[1] {
            0xC0..=0xCF => {
                copy_header(source, destination, &marker)?;
            }
            0xD0..=0xD7 => {
                destination.write_all(&marker)?;
                copy_data(source, destination)?;
            }
            0xD8..=0xD9 => {
                destination.write_all(&marker)?;
            }
            0xDA => {
                copy_header(source, destination, &marker)?;
                copy_data(source, destination)?;
            }
            0xDB..=0xDF => {
                copy_header(source, destination, &marker)?;
            }
            _ => {
                let size = read_u16(source)?;
                skip(source, size as u64 - 2)?;
            }
        }
    }
    Ok(())
}
