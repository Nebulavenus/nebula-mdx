use crate::chunks::{BytesTotalSize, Node};
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct BoneChunk {
    pub chunk_size: u32,

    pub data: Vec<Bone>,
}

calculate_chunk_size_impl!(BoneChunk);

impl ctx::TryFromCtx<'_, Endian> for BoneChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let bone = src.gread_with::<Bone>(offset, ctx)?;
            total_size += bone.node.inclusive_size + 4 + 4;
            data.push(bone);
        }

        Ok((BoneChunk { chunk_size, data }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for BoneChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for bone in self.data {
            src.gwrite_with::<Bone>(bone, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for BoneChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for bone in &self.data {
            result += bone.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Bone {
    pub node: Node,
    pub geoset_id: u32,
    pub geoset_animation_id: u32,
}

impl ctx::TryFromCtx<'_, Endian> for Bone {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let node = src.gread_with::<Node>(offset, ctx)?;
        let geoset_id = src.gread_with::<u32>(offset, ctx)?;
        let geoset_animation_id = src.gread_with::<u32>(offset, ctx)?;

        Ok((
            Bone {
                node,
                geoset_id,
                geoset_animation_id,
            },
            *offset,
        ))
    }
}

impl ctx::TryIntoCtx<Endian> for Bone {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<Node>(self.node, offset, ctx)?;
        src.gwrite_with::<u32>(self.geoset_id, offset, ctx)?;
        src.gwrite_with::<u32>(self.geoset_animation_id, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Bone {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += &self.node.total_bytes_size();
        result += size_of_val(&self.geoset_id);
        result += size_of_val(&self.geoset_animation_id);

        result
    }
}
