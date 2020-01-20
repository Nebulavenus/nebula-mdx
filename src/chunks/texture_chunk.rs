use crate::chunks::BytesTotalSize;
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct TextureChunk {
    pub chunk_size: u32,

    // chunk_size / 268
    pub data: Vec<Texture>,
}

calculate_chunk_size_impl!(TextureChunk);

impl ctx::TryFromCtx<'_, Endian> for TextureChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        if let Some(texture_count) = u32::checked_div(chunk_size.clone(), 268) {
            for _ in 0..texture_count {
                let texture = src.gread_with::<Texture>(offset, ctx)?;
                data.push(texture);
            }
        }

        Ok((TextureChunk { chunk_size, data }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for TextureChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for texture in self.data {
            src.gwrite_with::<Texture>(texture, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for TextureChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for texture in &self.data {
            result += texture.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Texture {
    pub replaceable_id: u32,
    pub file_name: String,
    pub unknown: u32,
    pub flags: u32,
}

impl ctx::TryFromCtx<'_, Endian> for Texture {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let replaceable_id = src.gread_with::<u32>(offset, ctx)?;

        // Name has fixed size
        let max_name_len = 256usize;
        let file_name = src.gread::<&str>(&mut offset.clone())?.to_string();
        *offset += max_name_len;

        let unknown = src.gread_with::<u32>(offset, ctx)?;
        let flags = src.gread_with::<u32>(offset, ctx)?;

        Ok((
            Texture {
                replaceable_id,
                file_name,
                unknown,
                flags,
            },
            *offset,
        ))
    }
}

impl ctx::TryIntoCtx<Endian> for Texture {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.replaceable_id, offset, ctx)?;

        // Name has fixed size
        let max_name_len = 256usize;
        let null_offset = &mut offset.clone();
        for _ in 0..max_name_len {
            src.gwrite_with::<u8>(0x0, null_offset, ctx)?;
        }
        src.gwrite_with::<&str>(self.file_name.as_ref(), &mut offset.clone(), ())?
            .to_string();
        *offset += max_name_len;

        src.gwrite_with::<u32>(self.unknown, offset, ctx)?;
        src.gwrite_with::<u32>(self.flags, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Texture {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.replaceable_id);
        let max_name_len = 256usize;
        result += max_name_len;
        result += size_of_val(&self.unknown);
        result += size_of_val(&self.flags);

        result
    }
}