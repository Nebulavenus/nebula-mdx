use crate::chunks::{BytesTotalSize, Transform, Vec3, Vec4};
use crate::consts::{KGRT_TAG, KGSC_TAG, KGTR_TAG};
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct Node {
    pub inclusive_size: u32,

    // max length 80
    pub name: String,
    pub object_id: u32,
    pub parent_id: u32,
    pub flags: u32,

    pub translation: Option<Transform<Vec3>>,
    pub rotation: Option<Transform<Vec4>>,
    pub scaling: Option<Transform<Vec3>>,
}

impl ctx::TryFromCtx<'_, Endian> for Node {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx)?;

        // Name has fixed size
        let max_name_len = 80usize;
        let name = src.gread::<&str>(&mut offset.clone())?.to_string();
        *offset += max_name_len;

        let object_id = src.gread_with::<u32>(offset, ctx)?;
        let parent_id = src.gread_with::<u32>(offset, ctx)?;
        let flags = src.gread_with::<u32>(offset, ctx)?;

        let mut node = Node {
            inclusive_size,
            name,
            object_id,
            parent_id,
            flags,
            translation: None,
            rotation: None,
            scaling: None,
        };

        while (*offset as u32) < inclusive_size {
            let tag = src.gread_with::<u32>(offset, ctx).unwrap();

            match tag {
                KGTR_TAG => {
                    let translation = src.gread_with::<Transform<Vec3>>(offset, ctx)?;
                    node.translation = Some(translation);
                }
                KGRT_TAG => {
                    let rotation = src.gread_with::<Transform<Vec4>>(offset, ctx)?;
                    node.rotation = Some(rotation);
                }
                KGSC_TAG => {
                    let scaling = src.gread_with::<Transform<Vec3>>(offset, ctx)?;
                    node.scaling = Some(scaling);
                }
                _ => unreachable!(),
            }
        }

        Ok((node, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Node {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.inclusive_size, offset, ctx)?;

        // Name has fixed size
        let max_name_len = 80usize;
        let null_offset = &mut offset.clone();
        for _ in 0..max_name_len {
            src.gwrite_with::<u8>(0x0, null_offset, ctx)?;
        }
        src.gwrite_with::<&str>(self.name.as_ref(), &mut offset.clone(), ())?;
        *offset += max_name_len;

        src.gwrite_with::<u32>(self.object_id, offset, ctx)?;
        src.gwrite_with::<u32>(self.parent_id, offset, ctx)?;
        src.gwrite_with::<u32>(self.flags, offset, ctx)?;

        if self.translation.is_some() {
            src.gwrite_with::<u32>(KGTR_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<Vec3>>(self.translation.unwrap(), offset, ctx)?;
        }
        if self.rotation.is_some() {
            src.gwrite_with::<u32>(KGRT_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<Vec4>>(self.rotation.unwrap(), offset, ctx)?;
        }
        if self.scaling.is_some() {
            src.gwrite_with::<u32>(KGSC_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<Vec3>>(self.scaling.unwrap(), offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for Node {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);

        let max_name_len = 80usize;
        result += max_name_len;

        result += size_of_val(&self.object_id);
        result += size_of_val(&self.parent_id);
        result += size_of_val(&self.flags);

        if self.translation.is_some() {
            result += 4;
            result += self.translation.as_ref().unwrap().total_bytes_size();
        }
        if self.rotation.is_some() {
            result += 4;
            result += self.rotation.as_ref().unwrap().total_bytes_size();
        }
        if self.scaling.is_some() {
            result += 4;
            result += self.scaling.as_ref().unwrap().total_bytes_size();
        }

        result
    }
}
