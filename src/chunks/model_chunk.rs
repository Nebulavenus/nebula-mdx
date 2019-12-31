use scroll::{ctx, Endian, Pread, Pwrite};
use crate::chunks::BytesTotalSize;
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct ModelChunk {
    pub chunk_size: u32,

    pub name: String,
    pub unknown: u32,
    pub bounds_radius: f32,
    pub minimum_extent: [f32; 3],
    pub maximum_extent: [f32; 3],
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
        let bounds_radius = src.gread_with::<f32>(offset, ctx)?;
        let minimum_extent = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];
        let maximum_extent = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];
        let blend_time = src.gread_with::<u32>(offset, ctx)?;
        Ok((ModelChunk {
            chunk_size,
            name,
            unknown,
            bounds_radius,
            minimum_extent,
            maximum_extent,
            blend_time,
        }, *offset))
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
        src.gwrite_with::<&str>(self.name.as_ref(), &mut offset.clone(), ())?.to_string();
        *offset += max_name_len;

        src.gwrite_with::<u32>(self.unknown, offset, ctx)?;
        src.gwrite_with::<f32>(self.bounds_radius, offset, ctx)?;
        for id in 0..3 {
            src.gwrite_with::<f32>(self.minimum_extent[id], offset, ctx)?;
        }
        for id in 0..3 {
            src.gwrite_with::<f32>(self.maximum_extent[id], offset, ctx)?;
        }
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
        result += size_of_val(&self.bounds_radius);
        result += size_of_val(&self.minimum_extent);
        result += size_of_val(&self.maximum_extent);
        result += size_of_val(&self.blend_time);

        result
    }
}