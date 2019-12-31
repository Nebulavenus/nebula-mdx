use scroll::{ctx, Endian, Pread, Pwrite};
use crate::chunks::BytesTotalSize;
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct VersionChunk {
    pub chunk_size: u32,

    pub version: u32,
}

calculate_chunk_size_impl!(VersionChunk);

impl ctx::TryFromCtx<'_, Endian> for VersionChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;
        let version = src.gread_with::<u32>(offset, ctx)?;
        Ok((VersionChunk { chunk_size, version }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for VersionChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;
        src.gwrite_with::<u32>(self.version, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for VersionChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);
        result += size_of_val(&self.version);

        result
    }
}
