use crate::chunks::BytesTotalSize;
use crate::chunks::Extent;
use scroll::{ctx, Endian, Pwrite};
use nebula_mdx_internal::{NMread, NMbts};

#[derive(NMread, NMbts, PartialEq, Debug)]
pub struct SequenceChunk {
    pub chunk_size: u32,

    // chunk_size / 132
    #[nebula(behaviour = "divided")]
    pub data: Vec<Sequence>,
}

calculate_chunk_size_impl!(SequenceChunk);

impl ctx::TryIntoCtx<Endian> for SequenceChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for sequence in self.data {
            src.gwrite_with::<Sequence>(sequence, offset, ctx)?;
        }

        Ok(*offset)
    }
}

#[derive(NMread, NMbts, Default, PartialEq, Debug)]
pub struct Sequence {
    #[nebula(length = "80")]
    pub name: String,
    pub interval_start: u32,
    pub interval_end: u32,
    pub move_speed: f32,
    pub non_looping: u32,
    pub rarity: f32,
    pub unknown: u32,
    pub extent: Extent,
}

impl ctx::TryIntoCtx<Endian> for Sequence {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        // Name has fixed size
        let max_name_len = 80usize;
        let null_offset = &mut 0usize;
        for _ in 0..max_name_len {
            src.gwrite_with::<u8>(0x0, null_offset, ctx)?;
        }
        src.gwrite_with::<&str>(self.name.as_ref(), &mut offset.clone(), ())?
            .to_string();
        *offset += max_name_len;

        src.gwrite_with::<u32>(self.interval_start, offset, ctx)?;
        src.gwrite_with::<u32>(self.interval_end, offset, ctx)?;
        src.gwrite_with::<f32>(self.move_speed, offset, ctx)?;
        src.gwrite_with::<u32>(self.non_looping, offset, ctx)?;
        src.gwrite_with::<f32>(self.rarity, offset, ctx)?;
        src.gwrite_with::<u32>(self.unknown, offset, ctx)?;
        src.gwrite_with::<Extent>(self.extent, offset, ctx)?;

        Ok(*offset)
    }
}