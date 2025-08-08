use crate::write_impl::write_gltf_tile::write_gltf_tile;

#[derive(Copy, Clone, Debug, bytemuck::NoUninit)]
#[repr(C)]
pub struct EncodablePosition {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub trait EncodableContent {
    fn encode(&self) -> Result<Vec<u8>, etiles_core::Error>;
}

impl EncodableContent for &Vec<etiles_core::Vertex> {
    fn encode(&self) -> Result<Vec<u8>, etiles_core::Error> {
        let mut point_data_buffer: Vec<u8> = Vec::new();
        write_gltf_tile(&mut point_data_buffer, self).expect("TODO: panic message");

        Ok(point_data_buffer)
    }
}
