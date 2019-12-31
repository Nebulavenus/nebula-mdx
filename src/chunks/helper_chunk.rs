use scroll::{ctx, Endian, Pread, Pwrite};
use crate::chunks::{BytesTotalSize, Node};
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct HelperChunk {
    pub chunk_size: u32,

    pub data: Vec<Helper>,
}

calculate_chunk_size_impl!(HelperChunk);

impl ctx::TryFromCtx<'_, Endian> for HelperChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let helper = src.gread_with::<Helper>(offset, ctx)?;
            total_size += helper.node.inclusive_size;
            data.push(helper);
        }

        Ok((HelperChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for HelperChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for helper in self.data {
            src.gwrite_with::<Helper>(helper, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for HelperChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for helper in &self.data {
            result += helper.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Helper {
    pub node: Node,
}

impl ctx::TryFromCtx<'_, Endian> for Helper {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let node = src.gread_with::<Node>(offset, ctx)?;

        Ok((Helper {
            node,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Helper {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<Node>(self.node, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Helper {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += &self.node.total_bytes_size();

        result
    }
}