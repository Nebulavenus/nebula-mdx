use crate::chunks::BytesTotalSize;
use scroll::{ctx, Endian, Pwrite};
use nebula_mdx_internal::{NMread, NMbts};

#[derive(NMread, NMbts, PartialEq, Debug)]
pub struct TextureChunk {
    pub chunk_size: u32,

    // chunk_size / 268
    #[nebula(behaviour = "divided")]
    pub data: Vec<Texture>,
}

calculate_chunk_size_impl!(TextureChunk);

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

#[derive(NMread, NMbts, Default, PartialEq, Debug)]
pub struct Texture {
    pub replaceable_id: u32,
    #[nebula(length = "256")]
    pub file_name: String,
    pub unknown: u32,
    pub flags: u32,
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