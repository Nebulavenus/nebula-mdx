use crate::chunks::BytesTotalSize;
use scroll::{ctx, Endian, Pwrite};
use nebula_mdx_internal::{NMread, NMbts};

#[derive(NMread, NMbts, PartialEq, Debug)]
pub struct GlobalSequenceChunk {
    pub chunk_size: u32,

    // chunk_size / 4
    #[nebula(behaviour = "divided")]
    pub data: Vec<GlobalSequence>,
}

calculate_chunk_size_impl!(GlobalSequenceChunk);

impl ctx::TryIntoCtx<Endian> for GlobalSequenceChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for global_sequence in self.data {
            src.gwrite_with::<GlobalSequence>(global_sequence, offset, ctx)?;
        }

        Ok(*offset)
    }
}

#[derive(NMread, NMbts, Default, PartialEq, Debug)]
pub struct GlobalSequence {
    pub duration: u32,
}

impl ctx::TryIntoCtx<Endian> for GlobalSequence {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.duration, offset, ctx)?;

        Ok(*offset)
    }
}