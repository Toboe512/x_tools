use crate::errors::Error;
use crate::utils::{read_u16, read_u8};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};

pub fn copy_header<R: Read, W: Write>(
    source: &mut R,
    destination: &mut W,
    marker: &[u8; 2],
) -> Result<(), Error> {
    destination.write_all(marker)?;
    let size = read_u16(source)?;
    destination.write_all(&size.to_be_bytes())?;
    if size > 2 {
        let size = size as u64 - 2;
        if io::copy(&mut source.take(size), destination)? < size {
            return Err(Error::Malformed);
        }
    }
    Ok(())
}

pub fn copy_data<R: Read + Seek, W: Write>(
    source: &mut R,
    destination: &mut W,
) -> Result<(), Error> {
    loop {
        let value = read_u8(source)?;
        if value == 0xFF {
            let next = read_u8(source)?;
            if next == 0 {
                destination.write_all(&[value, next])?;
            } else {
                source.seek(SeekFrom::Current(-2))?;
                break;
            }
        } else {
            destination.write_all(&[value])?;
        }
    }
    Ok(())
}
