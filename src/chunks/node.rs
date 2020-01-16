use crate::chunks::{BytesTotalSize, GeosetRotation, GeosetScaling, GeosetTranslation};
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

    pub geoset_translation: Option<GeosetTranslation>,
    pub geoset_rotation: Option<GeosetRotation>,
    pub geoset_scaling: Option<GeosetScaling>,
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
            geoset_translation: None,
            geoset_rotation: None,
            geoset_scaling: None,
        };

        while (*offset as u32) < inclusive_size {
            let tag = src.gread_with::<u32>(offset, ctx).unwrap();

            match tag {
                KGTR_TAG => {
                    let geoset_translation = src.gread_with::<GeosetTranslation>(offset, ctx)?;
                    node.geoset_translation = Some(geoset_translation);
                }
                KGRT_TAG => {
                    let geoset_rotation = src.gread_with::<GeosetRotation>(offset, ctx)?;
                    node.geoset_rotation = Some(geoset_rotation);
                }
                KGSC_TAG => {
                    let geoset_scaling = src.gread_with::<GeosetScaling>(offset, ctx)?;
                    node.geoset_scaling = Some(geoset_scaling);
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

        if self.geoset_translation.is_some() {
            src.gwrite_with::<u32>(KGTR_TAG, offset, ctx)?;
            src.gwrite_with::<GeosetTranslation>(self.geoset_translation.unwrap(), offset, ctx)?;
        }
        if self.geoset_rotation.is_some() {
            src.gwrite_with::<u32>(KGRT_TAG, offset, ctx)?;
            src.gwrite_with::<GeosetRotation>(self.geoset_rotation.unwrap(), offset, ctx)?;
        }
        if self.geoset_scaling.is_some() {
            src.gwrite_with::<u32>(KGSC_TAG, offset, ctx)?;
            src.gwrite_with::<GeosetScaling>(self.geoset_scaling.unwrap(), offset, ctx)?;
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

        if self.geoset_translation.is_some() {
            result += 4;
            result += self.geoset_translation.as_ref().unwrap().total_bytes_size();
        }
        if self.geoset_rotation.is_some() {
            result += 4;
            result += self.geoset_rotation.as_ref().unwrap().total_bytes_size();
        }
        if self.geoset_scaling.is_some() {
            result += 4;
            result += self.geoset_scaling.as_ref().unwrap().total_bytes_size();
        }

        result
    }
}
