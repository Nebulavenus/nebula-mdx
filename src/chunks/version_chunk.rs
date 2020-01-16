use crate::chunks::BytesTotalSize;
use scroll::{ctx, Endian, Pwrite};
use nebula_mdx_internal::{NMread, NMbts};

#[derive(NMread, NMbts, PartialEq, Debug)]
pub struct VersionChunk {
    pub chunk_size: u32,

    pub version: u32,
}

calculate_chunk_size_impl!(VersionChunk);

impl ctx::TryIntoCtx<Endian> for VersionChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;
        src.gwrite_with::<u32>(self.version, offset, ctx)?;

        Ok(*offset)
    }
}