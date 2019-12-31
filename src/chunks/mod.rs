pub trait BytesTotalSize {
    fn total_bytes_size(&self) -> usize;
}

pub use attachment_chunk::AttachmentChunk;
pub use bone_chunk::{BoneChunk, Bone};
pub use camera_chunk::CameraChunk;
pub use collision_shape_chunk::CollisionShapeChunk;
pub use event_object_chunk::EventObjectChunk;
pub use geoset_animation_chunk::{GeosetAnimationChunk, GeosetAnimation};
pub use geoset_chunk::GeosetChunk;
pub use global_sequence_chunk::{GlobalSequenceChunk, GlobalSequence};
pub use helper_chunk::{HelperChunk, Helper};
pub use light_chunk::LightChunk;
pub use material_chunk::MaterialChunk;
pub use model_chunk::ModelChunk;
pub use node::Node;
pub use particle_emitter_chunk::ParticleEmitterChunk;
pub use particle_emitter2_chunk::ParticleEmitter2Chunk;
pub use pivot_point_chunk::{PivotPointChunk, PivotPoint};
pub use ribbon_emitter_chunk::RibbonEmitterChunk;
pub use sequence_chunk::{SequenceChunk, Sequence};
pub use texture_animation_chunk::{TextureAnimationChunk, TextureAnimation};
pub use texture_chunk::{TextureChunk, Texture};
pub use tracks::*;
pub use version_chunk::VersionChunk;

macro_rules! calculate_chunk_size_impl {
    ($name:ident) => {
        impl $name {
            // Chunk size is a struct size without chunk_size itself.
            pub fn calculate_chunk_size(&mut self) {
                //dbg!($name);
                dbg!(&self.chunk_size);
                self.chunk_size = self.total_bytes_size() as u32 - 4;
                dbg!(&self.chunk_size);
            }
        }
    };
}

mod attachment_chunk;
mod bone_chunk;
mod camera_chunk;
mod collision_shape_chunk;
mod event_object_chunk;
mod geoset_animation_chunk;
mod geoset_chunk;
mod global_sequence_chunk;
mod helper_chunk;
mod light_chunk;
mod material_chunk;
mod model_chunk;
mod node;
mod particle_emitter_chunk;
mod particle_emitter2_chunk;
mod pivot_point_chunk;
mod ribbon_emitter_chunk;
mod sequence_chunk;
mod texture_animation_chunk;
mod texture_chunk;
mod tracks;
mod version_chunk;
