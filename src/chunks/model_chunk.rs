use crate::chunks::BytesTotalSize;
use crate::chunks::Extent;
use scroll::{ctx, Endian, Pwrite};
use nebula_mdx_internal::{NMread, NMbts};

#[derive(NMread, NMbts, PartialEq, Debug)]
pub struct ModelChunk {
    pub chunk_size: u32,

    #[nebula(length = "336")]
    pub name: String,
    pub unknown: u32,
    pub extent: Extent,
    pub blend_time: u32,
}

calculate_chunk_size_impl!(ModelChunk);

impl ctx::TryIntoCtx<Endian> for ModelChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        // Name has fixed size
        let max_name_len = 336usize;
        let null_offset = &mut offset.clone();
        for _ in 0..max_name_len {
            src.gwrite_with::<u8>(0x0, null_offset, ctx)?;
        }
        // FIX THIS IN SCROLL LIB
        src.gwrite_with::<&str>(self.name.as_ref(), &mut offset.clone(), ())?;
        *offset += max_name_len;

        src.gwrite_with::<u32>(self.unknown, offset, ctx)?;
        src.gwrite_with::<Extent>(self.extent, offset, ctx)?;
        src.gwrite_with::<u32>(self.blend_time, offset, ctx)?;

        Ok(*offset)
    }
}