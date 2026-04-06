pub mod shader;
pub mod render_target;

use crate::lsr::shader::{FragmentShader, VertexShader};
use crate::lsr::render_target::RenderTarget;
use crate::math::numerics::{Float2, Float3};

pub enum PrimitiveMode {
    Triangles,
    Lines,
    Points,
}

pub fn draw_arrays(target: &mut RenderTarget, mode: PrimitiveMode, vertices: &[Float3], uvs: &[Float2], normals: &[Float3], vshader: &impl VertexShader, fshader: &impl FragmentShader) {

}

pub fn draw_indexed(target: &mut RenderTarget, mode: PrimitiveMode, vertices: &[Float3], indices: &[u32], uvs: &[Float2], normals: &[Float3], vshader: &impl VertexShader, fshader: &impl FragmentShader) {

}