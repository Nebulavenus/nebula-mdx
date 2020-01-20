use crate::chunks::BytesTotalSize;
use crate::chunks::Extent;
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct ModelChunk {
    pub chunk_size: u32,

    pub name: String,
    pub unknown: u32,
    pub extent: Extent,
    pub blend_time: u32,
}

calculate_chunk_size_impl!(ModelChunk);

impl ctx::TryFromCtx<'_, Endian> for ModelChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        // Name has fixed size -- blizzard why?
        let max_name_len = 336usize;
        let name = src.gread::<&str>(&mut offset.clone())?.to_string();
        *offset += max_name_len;

        let unknown = src.gread_with::<u32>(offset, ctx)?;
        let extent = src.gread_with::<Extent>(offset, ctx)?;
        let blend_time = src.gread_with::<u32>(offset, ctx)?;
        Ok((
            ModelChunk {
                chunk_size,
                name,
                unknown,
                extent,
                blend_time,
            },
            *offset,
        ))
    }
}

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

impl BytesTotalSize for ModelChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        let max_name_len = 336usize;
        result += max_name_len;

        result += size_of_val(&self.unknown);
        result += &self.extent.total_bytes_size();
        result += size_of_val(&self.blend_time);

        result
    }
}