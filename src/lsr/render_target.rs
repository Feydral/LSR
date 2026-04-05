use crate::math::mathi;
use crate::math::numerics::float4::Float4;

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