pub mod shader;
pub mod render_target;

use crate::lsr::shader::{FragmentShader, VertexShader};

pub fn draw_arrays<V: VertexShader, F: FragmentShader>(target: &mut RenderTarget, mode: PrimitiveMode, vertices: &[Float3], uvs: &[Float2], normals: &[Float3], vshader: &V, fshader: &F) {

}

pub fn draw_indexed<V: VertexShader, F: FragmentShader>(target: &mut RenderTarget, mode: PrimitiveMode, vertices: &[Float3], indices: &[u32], uvs: &[Float2], normals: &[Float3], vshader: &V, fshader: &F) {

}