use crate::math::mathi;
use crate::math::numerics::float4::Float4;
use crate::math::numerics::float3::Float3;

pub struct RenderTarget {
    color_buffer: Vec<Float4>,
    depth_buffer: Vec<f32>,

    width: u32,
    height: u32,
}

impl RenderTarget {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            color_buffer: vec![Float4::ZERO; (width * height) as usize],
            depth_buffer: vec![f32::INFINITY; (width * height) as usize],

            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Float4, depth: f32) {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds: x: {} y: {}", x, y);
        }

        if color.w <= 0.0 {
            return;
        }

        let index = mathi::xy_to_index(x, y, self.width, self.height) as usize;

        let src = color;
        let alpha = src.w.clamp(0.0, 1.0);

        if alpha >= 1.0 {
            self.color_buffer[index] = src;
            self.depth_buffer[index] = depth;
            return;
        }

        let dst = self.color_buffer[index];
        let inv_alpha = 1.0 - alpha;

        let out = Float4::new(
            src.x * alpha + dst.x * inv_alpha,
            src.y * alpha + dst.y * inv_alpha,
            src.z * alpha + dst.z * inv_alpha,
            alpha + dst.w * inv_alpha,
        );

        self.color_buffer[index] = out;
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> Float3 {

    }
    
    pub fn get_pixel_depth(&self, x: u32, y: u32) -> f32 {

    }

    pub fn clear(&mut self, color: Float4) {
        
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}