use crate::chunks::{BytesTotalSize, CameraRotation, CameraTargetTranslation, CameraTranslation};
use crate::consts::{KCRL_TAG, KCTR_TAG, KTTR_TAG};
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

// TODO(nv): NOT TESTED! WRITE TESTS.

#[derive(PartialEq, Debug)]
pub struct CameraChunk {
    pub chunk_size: u32,

    pub data: Vec<Camera>,
}

calculate_chunk_size_impl!(CameraChunk);

impl ctx::TryFromCtx<'_, Endian> for CameraChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let camera = src.gread_with::<Camera>(offset, ctx)?;
            total_size += camera.inclusive_size;
            assert_eq!(camera.inclusive_size, camera.total_bytes_size() as u32);
            data.push(camera);
        }

        Ok((CameraChunk { chunk_size, data }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for CameraChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for camera in self.data {
            src.gwrite_with::<Camera>(camera, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for CameraChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for camera in &self.data {
            result += camera.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Camera {
    pub inclusive_size: u32,

    // max length 80
    pub name: String,
    pub position: [f32; 3],
    pub field_of_view: f32,
    pub far_clipping_plane: f32,
    pub near_clipping_plane: f32,
    pub target_position: [f32; 3],

    pub translation: Option<CameraTranslation>,
    pub rotation: Option<CameraRotation>,
    pub target_translation: Option<CameraTargetTranslation>,
}

impl ctx::TryFromCtx<'_, Endian> for Camera {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx)?;

        let max_name_len = 80usize;
        let name = src.gread::<&str>(&mut offset.clone())?.to_string();
        *offset += max_name_len;

        let mut position = [0f32; 3];
        for id in 0..position.len() {
            position[id] = src.gread_with::<f32>(offset, ctx)?;
        }

        let field_of_view = src.gread_with::<f32>(offset, ctx)?;
        let far_clipping_plane = src.gread_with::<f32>(offset, ctx)?;
        let near_clipping_plane = src.gread_with::<f32>(offset, ctx)?;

        let mut target_position = [0f32; 3];
        for id in 0..target_position.len() {
            target_position[id] = src.gread_with::<f32>(offset, ctx)?;
        }

        let mut camera = Camera {
            inclusive_size,
            name,
            position,
            field_of_view,
            far_clipping_plane,
            near_clipping_plane,
            target_position,
            translation: None,
            rotation: None,
            target_translation: None,
        };

        while (*offset as u32) < inclusive_size {
            let tag = src.gread_with::<u32>(offset, ctx).unwrap();

            match tag {
                KCTR_TAG => {
                    let translation = src.gread_with::<CameraTranslation>(offset, ctx)?;
                    camera.translation = Some(translation);
                }
                KCRL_TAG => {
                    let rotation = src.gread_with::<CameraRotation>(offset, ctx)?;
                    camera.rotation = Some(rotation);
                }
                KTTR_TAG => {
                    let target_translation =
                        src.gread_with::<CameraTargetTranslation>(offset, ctx)?;
                    camera.target_translation = Some(target_translation);
                }
                _ => unreachable!(),
            }
        }

        Ok((camera, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Camera {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.inclusive_size, offset, ctx)?;

        // String has fixed size
        let max_path_len = 80usize;
        let null_offset = &mut offset.clone();
        for _ in 0..max_path_len {
            src.gwrite_with::<u8>(0x0, null_offset, ctx)?;
        }
        src.gwrite_with::<&str>(self.name.as_ref(), &mut offset.clone(), ())?;
        *offset += max_path_len;

        for v in &self.position {
            src.gwrite_with::<f32>(*v, offset, ctx)?;
        }

        src.gwrite_with::<f32>(self.field_of_view, offset, ctx)?;
        src.gwrite_with::<f32>(self.far_clipping_plane, offset, ctx)?;
        src.gwrite_with::<f32>(self.near_clipping_plane, offset, ctx)?;

        for v in &self.target_position {
            src.gwrite_with::<f32>(*v, offset, ctx)?;
        }

        if self.translation.is_some() {
            src.gwrite_with::<u32>(KCTR_TAG, offset, ctx)?;
            src.gwrite_with::<CameraTranslation>(self.translation.unwrap(), offset, ctx)?;
        }

        if self.rotation.is_some() {
            src.gwrite_with::<u32>(KCRL_TAG, offset, ctx)?;
            src.gwrite_with::<CameraRotation>(self.rotation.unwrap(), offset, ctx)?;
        }

        if self.target_translation.is_some() {
            src.gwrite_with::<u32>(KCRL_TAG, offset, ctx)?;
            src.gwrite_with::<CameraTargetTranslation>(
                self.target_translation.unwrap(),
                offset,
                ctx,
            )?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for Camera {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);

        let max_path_len = 80usize;
        result += max_path_len;

        result += size_of_val(&self.position);
        result += size_of_val(&self.field_of_view);
        result += size_of_val(&self.far_clipping_plane);
        result += size_of_val(&self.near_clipping_plane);
        result += size_of_val(&self.target_position);

        if self.translation.is_some() {
            result += 4;
            result += self.translation.as_ref().unwrap().total_bytes_size();
        }

        if self.rotation.is_some() {
            result += 4;
            result += self.rotation.as_ref().unwrap().total_bytes_size();
        }

        if self.target_translation.is_some() {
            result += 4;
            result += self.target_translation.as_ref().unwrap().total_bytes_size();
        }

        result
    }
}
