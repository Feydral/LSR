use crate::math::numerics::{Float2, Float3, Float4};

pub trait VertexShader {
    fn vertex(&mut self, position: Float3, uv: Float2, normal: Float3) -> (Float4, Float2, Float3);
}

pub trait FragmentShader {
    fn fragment(&mut self, pixel_pos: Float2, depth: f32, uv: Float2, normal: Float3) -> Float4;
}

#[allow(dead_code)]
pub struct VDefault { }

#[allow(dead_code)]
impl VertexShader for VDefault {
    fn vertex(&mut self, position: Float3, uv: Float2, normal: Float3, ) -> (Float4, Float2, Float3) {
        let clip_pos = Float4::new(position.x, position.y, position.z, 1.0);
        (clip_pos, uv, normal)
    }
}

#[allow(dead_code)]
pub struct FDefault { }

#[allow(dead_code)]
impl FragmentShader for FDefault {
    fn fragment(&mut self, _pixel_pos: Float2, _depth: f32, _uv: Float2, _normal: Float3) -> Float4 {
        Float4::new(1.0, 0.0, 0.0, 1.0)
    }
}