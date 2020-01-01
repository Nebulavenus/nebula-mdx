use crate::chunks::*;
use crate::consts::*;
use scroll::{Pread, Pwrite, LE};

#[derive(PartialEq, Debug, Default)]
pub struct MDLXModel {
    pub version_chunk: Option<VersionChunk>,
    pub model_chunk: Option<ModelChunk>,
    pub sequence_chunk: Option<SequenceChunk>,
    pub global_sequence_chunk: Option<GlobalSequenceChunk>,
    pub texture_chunk: Option<TextureChunk>,
    pub texture_animation_chunk: Option<TextureAnimationChunk>,
    pub geoset_chunk: Option<GeosetChunk>,
    pub geoset_animation_chunk: Option<GeosetAnimationChunk>,
    pub bone_chunk: Option<BoneChunk>,
    pub light_chunk: Option<LightChunk>,
    pub helper_chunk: Option<HelperChunk>,
    pub attachment_chunk: Option<AttachmentChunk>,
    pub pivot_point_chunk: Option<PivotPointChunk>,
    pub particle_emitter_chunk: Option<ParticleEmitterChunk>,
    pub particle_emitter2_chunk: Option<ParticleEmitter2Chunk>,
    pub ribbon_emitter_chunk: Option<RibbonEmitterChunk>,
    pub event_object_chunk: Option<EventObjectChunk>,
    pub camera_chunk: Option<CameraChunk>,
    pub collision_shape_chunk: Option<CollisionShapeChunk>,
    pub material_chunk: Option<MaterialChunk>,
}

impl MDLXModel {
    pub fn read_mdx_file(data: Vec<u8>) -> Result<MDLXModel, scroll::Error> {
        let offset = &mut 0usize;
        let mdlx_tag = data.gread_with::<u32>(offset, LE)?;
        if mdlx_tag == MDLX_TAG {
            let mut result = MDLXModel::default();

            // Iterate over chunks
            while *offset < data.len() {
                info!("Offset: {}", &offset);

                // For debug
                let mut tag_offset = offset.clone();
                let tag_buffer = (0..4)
                    .map(|_| data.gread::<u8>(&mut tag_offset).unwrap())
                    .collect::<Vec<u8>>();
                let tag_name = String::from_utf8(tag_buffer).unwrap_or("NOTAG".to_string());

                let tag = data.gread_with::<u32>(offset, LE)?;
                info!("TagHex: {}", format!("{:X}", &tag));
                info!("TagDec: {}", &tag);
                info!("TagName: {}", &tag_name);

                result.handle_tag(tag, &data, offset)?;
            }

            Ok(result)
        } else {
            Err(scroll::Error::Custom("Not correct MDLX file".to_string()))
        }
    }

    pub fn write_mdx_file(mut model: MDLXModel) -> Result<Vec<u8>, scroll::Error> {
        // Get total size of mdx file
        let total_size = model.model_total_size();
        model.correct_chunk_size();

        // Create vec with capacity and set it len to total size
        let mut data = Vec::<u8>::with_capacity(total_size);
        unsafe {
            data.set_len(total_size);
        }

        // Begin to write fields
        let offset = &mut 0usize;

        data.gwrite_with::<u32>(MDLX_TAG, offset, LE)?;

        if model.version_chunk.is_some() {
            data.gwrite_with::<u32>(VERS_TAG, offset, LE)?;
            data.gwrite_with::<VersionChunk>(model.version_chunk.unwrap(), offset, LE)?;
        }
        if model.model_chunk.is_some() {
            data.gwrite_with::<u32>(MODL_TAG, offset, LE)?;
            data.gwrite_with::<ModelChunk>(model.model_chunk.unwrap(), offset, LE)?;
        }
        if model.sequence_chunk.is_some() {
            data.gwrite_with::<u32>(SEQS_TAG, offset, LE)?;
            data.gwrite_with::<SequenceChunk>(model.sequence_chunk.unwrap(), offset, LE)?;
        }
        if model.global_sequence_chunk.is_some() {
            data.gwrite_with::<u32>(GLBS_TAG, offset, LE)?;
            data.gwrite_with::<GlobalSequenceChunk>(
                model.global_sequence_chunk.unwrap(),
                offset,
                LE,
            )?;
        }
        if model.texture_chunk.is_some() {
            data.gwrite_with::<u32>(TEXS_TAG, offset, LE)?;
            data.gwrite_with::<TextureChunk>(model.texture_chunk.unwrap(), offset, LE)?;
        }
        if model.texture_animation_chunk.is_some() {
            data.gwrite_with::<u32>(TXAN_TAG, offset, LE)?;
            data.gwrite_with::<TextureAnimationChunk>(
                model.texture_animation_chunk.unwrap(),
                offset,
                LE,
            )?;
        }
        if model.geoset_chunk.is_some() {
            data.gwrite_with::<u32>(GEOS_TAG, offset, LE)?;
            data.gwrite_with::<GeosetChunk>(model.geoset_chunk.unwrap(), offset, LE)?;
        }
        if model.geoset_animation_chunk.is_some() {
            data.gwrite_with::<u32>(GEOA_TAG, offset, LE)?;
            data.gwrite_with::<GeosetAnimationChunk>(
                model.geoset_animation_chunk.unwrap(),
                offset,
                LE,
            )?;
        }
        if model.bone_chunk.is_some() {
            data.gwrite_with::<u32>(BONE_TAG, offset, LE)?;
            data.gwrite_with::<BoneChunk>(model.bone_chunk.unwrap(), offset, LE)?;
        }
        if model.light_chunk.is_some() {
            data.gwrite_with::<u32>(LITE_TAG, offset, LE)?;
            data.gwrite_with::<LightChunk>(model.light_chunk.unwrap(), offset, LE)?;
        }
        if model.helper_chunk.is_some() {
            data.gwrite_with::<u32>(HELP_TAG, offset, LE)?;
            data.gwrite_with::<HelperChunk>(model.helper_chunk.unwrap(), offset, LE)?;
        }
        if model.attachment_chunk.is_some() {
            data.gwrite_with::<u32>(ATCH_TAG, offset, LE)?;
            data.gwrite_with::<AttachmentChunk>(model.attachment_chunk.unwrap(), offset, LE)?;
        }
        if model.pivot_point_chunk.is_some() {
            data.gwrite_with::<u32>(PIVT_TAG, offset, LE)?;
            data.gwrite_with::<PivotPointChunk>(model.pivot_point_chunk.unwrap(), offset, LE)?;
        }
        if model.particle_emitter_chunk.is_some() {
            data.gwrite_with::<u32>(PREM_TAG, offset, LE)?;
            data.gwrite_with::<ParticleEmitterChunk>(
                model.particle_emitter_chunk.unwrap(),
                offset,
                LE,
            )?;
        }
        if model.particle_emitter2_chunk.is_some() {
            data.gwrite_with::<u32>(PRE2_TAG, offset, LE)?;
            data.gwrite_with::<ParticleEmitter2Chunk>(
                model.particle_emitter2_chunk.unwrap(),
                offset,
                LE,
            )?;
        }
        if model.ribbon_emitter_chunk.is_some() {
            data.gwrite_with::<u32>(RIBB_TAG, offset, LE)?;
            data.gwrite_with::<RibbonEmitterChunk>(
                model.ribbon_emitter_chunk.unwrap(),
                offset,
                LE,
            )?;
        }
        if model.event_object_chunk.is_some() {
            data.gwrite_with::<u32>(EVTS_TAG, offset, LE)?;
            data.gwrite_with::<EventObjectChunk>(model.event_object_chunk.unwrap(), offset, LE)?;
        }
        if model.camera_chunk.is_some() {
            data.gwrite_with::<u32>(CAMS_TAG, offset, LE)?;
            data.gwrite_with::<CameraChunk>(model.camera_chunk.unwrap(), offset, LE)?;
        }
        if model.collision_shape_chunk.is_some() {
            data.gwrite_with::<u32>(CLID_TAG, offset, LE)?;
            data.gwrite_with::<CollisionShapeChunk>(
                model.collision_shape_chunk.unwrap(),
                offset,
                LE,
            )?;
        }
        if model.material_chunk.is_some() {
            data.gwrite_with::<u32>(MTLS_TAG, offset, LE)?;
            data.gwrite_with::<MaterialChunk>(model.material_chunk.unwrap(), offset, LE)?;
        }

        // Return result
        Ok(data)
    }

    fn correct_chunk_size(&mut self) {
        if self.version_chunk.is_some() {
            let version = self.version_chunk.as_mut().unwrap();
            version.calculate_chunk_size();
        }
        if self.model_chunk.is_some() {
            let model = self.model_chunk.as_mut().unwrap();
            model.calculate_chunk_size();
        }
        if self.sequence_chunk.is_some() {
            let sequence = self.sequence_chunk.as_mut().unwrap();
            sequence.calculate_chunk_size();
        }
        if self.global_sequence_chunk.is_some() {
            let global_sequence = self.global_sequence_chunk.as_mut().unwrap();
            global_sequence.calculate_chunk_size();
        }
        if self.texture_chunk.is_some() {
            let texture = self.texture_chunk.as_mut().unwrap();
            texture.calculate_chunk_size();
        }
        if self.texture_animation_chunk.is_some() {
            let texture_animation = self.texture_animation_chunk.as_mut().unwrap();
            texture_animation.calculate_chunk_size();
        }
        if self.geoset_chunk.is_some() {
            let geoset = self.geoset_chunk.as_mut().unwrap();
            geoset.calculate_chunk_size();
        }
        if self.geoset_animation_chunk.is_some() {
            let geoset_animation = self.geoset_animation_chunk.as_mut().unwrap();
            geoset_animation.calculate_chunk_size();
        }
        if self.bone_chunk.is_some() {
            let bone = self.bone_chunk.as_mut().unwrap();
            bone.calculate_chunk_size();
        }
        if self.light_chunk.is_some() {
            let light = self.light_chunk.as_mut().unwrap();
            light.calculate_chunk_size();
        }
        if self.helper_chunk.is_some() {
            let helper = self.helper_chunk.as_mut().unwrap();
            helper.calculate_chunk_size();
        }
        if self.attachment_chunk.is_some() {
            let attachment = self.attachment_chunk.as_mut().unwrap();
            attachment.calculate_chunk_size();
        }
        if self.pivot_point_chunk.is_some() {
            let pivot = self.pivot_point_chunk.as_mut().unwrap();
            pivot.calculate_chunk_size();
        }
        if self.particle_emitter_chunk.is_some() {
            let particle_emitter = self.particle_emitter_chunk.as_mut().unwrap();
            particle_emitter.calculate_chunk_size();
        }
        if self.particle_emitter2_chunk.is_some() {
            let particle_emitter2 = self.particle_emitter2_chunk.as_mut().unwrap();
            particle_emitter2.calculate_chunk_size();
        }
        if self.ribbon_emitter_chunk.is_some() {
            let ribbon_emitter = self.ribbon_emitter_chunk.as_mut().unwrap();
            ribbon_emitter.calculate_chunk_size();
        }
        if self.event_object_chunk.is_some() {
            let event_object = self.event_object_chunk.as_mut().unwrap();
            event_object.calculate_chunk_size();
        }
        if self.camera_chunk.is_some() {
            let camera = self.camera_chunk.as_mut().unwrap();
            camera.calculate_chunk_size();
        }
        if self.collision_shape_chunk.is_some() {
            let collision_shape = self.collision_shape_chunk.as_mut().unwrap();
            collision_shape.calculate_chunk_size();
        }
        if self.material_chunk.is_some() {
            let material = self.material_chunk.as_mut().unwrap();
            material.calculate_chunk_size();
        }
    }

    fn model_total_size(&self) -> usize {
        let mut result = 0usize;
        // MDLX_TAG
        result += 4;
        // Tag + size of bytes inside structs
        if self.version_chunk.is_some() {
            let version = self.version_chunk.as_ref().unwrap();
            result += 4;
            result += version.total_bytes_size();
        }
        if self.model_chunk.is_some() {
            let model = self.model_chunk.as_ref().unwrap();
            result += 4;
            result += model.total_bytes_size();
        }
        if self.sequence_chunk.is_some() {
            let sequence = self.sequence_chunk.as_ref().unwrap();
            result += 4;
            result += sequence.total_bytes_size();
        }
        if self.global_sequence_chunk.is_some() {
            let global_sequence = self.global_sequence_chunk.as_ref().unwrap();
            result += 4;
            result += global_sequence.total_bytes_size();
        }
        if self.texture_chunk.is_some() {
            let texture = self.texture_chunk.as_ref().unwrap();
            result += 4;
            result += texture.total_bytes_size();
        }
        if self.texture_animation_chunk.is_some() {
            let texture_animation = self.texture_animation_chunk.as_ref().unwrap();
            result += 4;
            result += texture_animation.total_bytes_size();
        }
        if self.geoset_chunk.is_some() {
            let geoset = self.geoset_chunk.as_ref().unwrap();
            result += 4;
            result += geoset.total_bytes_size();
        }
        if self.geoset_animation_chunk.is_some() {
            let geoset_animation = self.geoset_animation_chunk.as_ref().unwrap();
            result += 4;
            result += geoset_animation.total_bytes_size();
        }
        if self.bone_chunk.is_some() {
            let bone = self.bone_chunk.as_ref().unwrap();
            result += 4;
            result += bone.total_bytes_size();
        }
        if self.light_chunk.is_some() {
            let light = self.light_chunk.as_ref().unwrap();
            result += 4;
            result += light.total_bytes_size();
        }
        if self.helper_chunk.is_some() {
            let helper = self.helper_chunk.as_ref().unwrap();
            result += 4;
            result += helper.total_bytes_size();
        }
        if self.attachment_chunk.is_some() {
            let attachment = self.attachment_chunk.as_ref().unwrap();
            result += 4;
            result += attachment.total_bytes_size();
        }
        if self.pivot_point_chunk.is_some() {
            let pivot = self.pivot_point_chunk.as_ref().unwrap();
            result += 4;
            result += pivot.total_bytes_size();
        }
        if self.particle_emitter_chunk.is_some() {
            let particle_emitter = self.particle_emitter_chunk.as_ref().unwrap();
            result += 4;
            result += particle_emitter.total_bytes_size();
        }
        if self.particle_emitter2_chunk.is_some() {
            let particle_emitter2 = self.particle_emitter2_chunk.as_ref().unwrap();
            result += 4;
            result += particle_emitter2.total_bytes_size();
        }
        if self.ribbon_emitter_chunk.is_some() {
            let ribbon_emitter = self.ribbon_emitter_chunk.as_ref().unwrap();
            result += 4;
            result += ribbon_emitter.total_bytes_size();
        }
        if self.event_object_chunk.is_some() {
            let event_object = self.event_object_chunk.as_ref().unwrap();
            result += 4;
            result += event_object.total_bytes_size();
        }
        if self.camera_chunk.is_some() {
            let camera = self.camera_chunk.as_ref().unwrap();
            result += 4;
            result += camera.total_bytes_size();
        }
        if self.collision_shape_chunk.is_some() {
            let collision_shape = self.collision_shape_chunk.as_ref().unwrap();
            result += 4;
            result += collision_shape.total_bytes_size();
        }
        if self.material_chunk.is_some() {
            let material = self.material_chunk.as_ref().unwrap();
            result += 4;
            result += material.total_bytes_size();
        }

        result
    }

    fn handle_tag(
        &mut self,
        tag: u32,
        data: &[u8],
        offset: &mut usize,
    ) -> Result<(), scroll::Error> {
        match tag {
            VERS_TAG => {
                let version_chunk = data.gread_with::<VersionChunk>(offset, LE)?;
                self.version_chunk = Some(version_chunk);
            }
            MODL_TAG => {
                let model_chunk = data.gread_with::<ModelChunk>(offset, LE)?;
                self.model_chunk = Some(model_chunk);
            }
            SEQS_TAG => {
                let sequence_chunk = data.gread_with::<SequenceChunk>(offset, LE)?;
                self.sequence_chunk = Some(sequence_chunk);
            }
            GLBS_TAG => {
                let global_sequence_chunk = data.gread_with::<GlobalSequenceChunk>(offset, LE)?;
                self.global_sequence_chunk = Some(global_sequence_chunk);
            }
            TEXS_TAG => {
                let texture_chunk = data.gread_with::<TextureChunk>(offset, LE)?;
                self.texture_chunk = Some(texture_chunk);
            }
            TXAN_TAG => {
                let texture_animation_chunk =
                    data.gread_with::<TextureAnimationChunk>(offset, LE)?;
                self.texture_animation_chunk = Some(texture_animation_chunk);
            }
            GEOS_TAG => {
                let geoset_chunk = data.gread_with::<GeosetChunk>(offset, LE)?;
                self.geoset_chunk = Some(geoset_chunk);
            }
            GEOA_TAG => {
                let geoset_animation_chunk = data.gread_with::<GeosetAnimationChunk>(offset, LE)?;
                self.geoset_animation_chunk = Some(geoset_animation_chunk);
            }
            BONE_TAG => {
                let bone_chunk = data.gread_with::<BoneChunk>(offset, LE)?;
                self.bone_chunk = Some(bone_chunk);
            }
            LITE_TAG => {
                let light_chunk = data.gread_with::<LightChunk>(offset, LE)?;
                self.light_chunk = Some(light_chunk);
            }
            HELP_TAG => {
                let helper_chunk = data.gread_with::<HelperChunk>(offset, LE)?;
                self.helper_chunk = Some(helper_chunk);
            }
            ATCH_TAG => {
                let attachment_chunk = data.gread_with::<AttachmentChunk>(offset, LE)?;
                self.attachment_chunk = Some(attachment_chunk);
            }
            PIVT_TAG => {
                let pivot_chunk = data.gread_with::<PivotPointChunk>(offset, LE)?;
                self.pivot_point_chunk = Some(pivot_chunk);
            }
            PREM_TAG => {
                let particle_emitter_chunk = data.gread_with::<ParticleEmitterChunk>(offset, LE)?;
                self.particle_emitter_chunk = Some(particle_emitter_chunk);
            }
            PRE2_TAG => {
                let particle_emitter2_chunk =
                    data.gread_with::<ParticleEmitter2Chunk>(offset, LE)?;
                self.particle_emitter2_chunk = Some(particle_emitter2_chunk);
            }
            RIBB_TAG => {
                let ribbon_emitter_chunk = data.gread_with::<RibbonEmitterChunk>(offset, LE)?;
                self.ribbon_emitter_chunk = Some(ribbon_emitter_chunk);
            }
            EVTS_TAG => {
                let event_object_chunk = data.gread_with::<EventObjectChunk>(offset, LE)?;
                self.event_object_chunk = Some(event_object_chunk);
            }
            CAMS_TAG => {
                let camera_chunk = data.gread_with::<CameraChunk>(offset, LE)?;
                self.camera_chunk = Some(camera_chunk);
            }
            CLID_TAG => {
                let collision_shape_chunk = data.gread_with::<CollisionShapeChunk>(offset, LE)?;
                self.collision_shape_chunk = Some(collision_shape_chunk);
            }
            MTLS_TAG => {
                let material_chunk = data.gread_with::<MaterialChunk>(offset, LE)?;
                self.material_chunk = Some(material_chunk);
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}
