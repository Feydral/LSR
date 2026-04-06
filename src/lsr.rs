pub mod shader;
pub mod render_target;

use crate::lsr::shader::{FragmentShader, VertexShader};
use crate::lsr::render_target::RenderTarget;
use crate::math::mathf;
use crate::math::numerics::{Float2, Float3, Float4};

pub enum PrimitiveMode {
    Triangles,
    Lines,
    Points,
}

pub fn draw_arrays(target: &mut RenderTarget, mode: PrimitiveMode, vertices: &[Float3], uvs: &[Float2], normals: &[Float3], vshader: &impl VertexShader, fshader: &impl FragmentShader) {

}

pub fn draw_indexed(target: &mut RenderTarget, mode: PrimitiveMode, vertices: &[Float3], indices: &[u32], uvs: &[Float2], normals: &[Float3], vshader: &impl VertexShader, fshader: &impl FragmentShader) {

}

fn rasterize_triangle_to_target(target: &mut RenderTarget, fshader: &mut impl FragmentShader, v0: (Float4, Float2, Float3), v1: (Float4, Float2, Float3), v2: (Float4, Float2, Float3)) {
    let w = target.width() as f32;
    let h = target.height() as f32;

    let s0 = clip_to_screen(v0.0, w, h);
    let s1 = clip_to_screen(v1.0, w, h);
    let s2 = clip_to_screen(v2.0, w, h);

    let min_x = (f32::min(s0.x, f32::min(s1.x, s2.x)).floor() as i32).max(0);
    let min_y = (f32::min(s0.y, f32::min(s1.y, s2.y)).floor() as i32).max(0);
    let max_x = (f32::max(s0.x, f32::max(s1.x, s2.x)).ceil() as i32).min(target.width() as i32 - 1);
    let max_y = (f32::max(s0.y, f32::max(s1.y, s2.y)).ceil() as i32).min(target.height() as i32 - 1);

    if min_x > max_x || min_y > max_y { return; }

    let inv_w0 = 1.0 / v0.0.w;
    let inv_w1 = 1.0 / v1.0.w;
    let inv_w2 = 1.0 / v2.0.w;

    let uv0 = v0.1 * inv_w0;
    let uv1 = v1.1 * inv_w1;
    let uv2 = v2.1 * inv_w2;

    let n0 = v0.2 * inv_w0;
    let n1 = v1.2 * inv_w1;
    let n2 = v2.2 * inv_w2;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Float2::new(x as f32 + 0.5, y as f32 + 0.5);

            let mut wa = 0.0;
            let mut wb = 0.0;
            let mut wc = 0.0;

            if !mathf::point_in_triangle(s0, s1, s2, p, &mut wa, &mut wb, &mut wc) { continue; }

            let inv_w_interp = wa * inv_w0 + wb * inv_w1 + wc * inv_w2;
            let depth = 1.0 / inv_w_interp;

            if depth >= target.get_pixel_depth(x as u32, y as u32) {
                continue;
            }

            let uv = (uv0 * wa + uv1 * wb + uv2 * wc) * depth;
            let normal = (n0 * wa + n1 * wb + n2 * wc) * depth;

            let color = fshader.fragment(p, depth, uv, normal);
            target.set_pixel(x as u32, y as u32, color, depth);
        }
    }
}

fn draw_line_to_target(target: &mut RenderTarget, s0: Float2, s1: Float2) {
    let x0 = s0.x.round() as i32;
    let y0 = s0.y.round() as i32;
    let x1 = s1.x.round() as i32;
    let y1 = s1.y.round() as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && y >= 0 && (x as u32) < target.width() && (y as u32) < target.height() {
            target.set_pixel(x as u32, y as u32, Float4::new(1.0, 1.0, 1.0, 1.0), 1.0);
        }

        if x == x1 && y == y1 { break; }

        let e2 = 2 * err;
        if e2 > -dy { err -= dy; x += sx; }
        if e2 < dx { err += dx; y += sy; }
    }
}

fn draw_point_to_target(target: &mut RenderTarget, screen_pos: Float2) {
    let cx = screen_pos.x.round() as i32;
    let cy = screen_pos.y.round() as i32;

    for oy in -1..=1 {
        for ox in -1..=1 {
            let x = cx + ox;
            let y = cy + oy;

            if x >= 0 && y >= 0 && (x as u32) < target.width() && (y as u32) < target.height() {
                target.set_pixel(x as u32, y as u32, Float4::new(1.0, 1.0, 1.0, 1.0), 1.0);
            }
        }
    }
}

fn interpolate_vertex(a: (Float4, Float2, Float3), b: (Float4, Float2, Float3), t: f32) -> (Float4, Float2, Float3) {
    (mathf::lerp_float4(a.0, b.0, t), mathf::lerp_float2(a.1, b.1, t), mathf::lerp_float3(a.2, b.2, t).normalize())
}