use scroll::{ctx, Endian, Pread, Pwrite};
use crate::chunks::BytesTotalSize;
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct CameraChunk {
    pub chunk_size: u32,

    pub bytes: Vec<u8>,
}

calculate_chunk_size_impl!(CameraChunk);

impl ctx::TryFromCtx<'_, Endian> for CameraChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut bytes = Vec::with_capacity(chunk_size as usize);
        unsafe {
            bytes.set_len(chunk_size as usize);
        }
        src.gread_inout_with(offset, &mut bytes, ctx)?;

        Ok((CameraChunk {
            chunk_size,
            bytes,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for CameraChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for CameraChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += &self.bytes.capacity();

        result
    }
}