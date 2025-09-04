//! Video system for emulator

use crate::{EmuliteResult, EmuliteError, config::VideoConfig as ConfigVideoConfig};
use wgpu::util::DeviceExt;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    event::{Event, WindowEvent},
};
use image::{ImageBuffer, RgbaImage, Rgba};
use std::sync::{Arc, Mutex};

/// Video configuration
#[derive(Debug, Clone)]
pub struct VideoConfig {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub scale: f32,
    pub filter: VideoFilter,
    pub enabled: bool,
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            width: 256,
            height: 240,
            fullscreen: false,
            vsync: true,
            scale: 3.0,
            filter: VideoFilter::Nearest,
            enabled: true,
        }
    }
}

/// Video filtering options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VideoFilter {
    Nearest,
    Linear,
    PixelPerfect,
}

/// Color palette for retro systems
#[derive(Debug, Clone)]
pub struct ColorPalette {
    colors: Vec<[u8; 4]>, // RGBA
}

impl ColorPalette {
    pub fn new() -> Self {
        Self {
            colors: vec![[0, 0, 0, 255]; 256], // Default to black
        }
    }
    
    pub fn set_color(&mut self, index: usize, r: u8, g: u8, b: u8, a: u8) {
        if index < self.colors.len() {
            self.colors[index] = [r, g, b, a];
        }
    }
    
    pub fn get_color(&self, index: usize) -> [u8; 4] {
        self.colors.get(index).copied().unwrap_or([0, 0, 0, 255])
    }
    
    pub fn load_nes_palette(&mut self) {
        // NES color palette
        let nes_colors = [
            [84, 84, 84, 255], [0, 30, 116, 255], [8, 16, 144, 255], [48, 0, 136, 255],
            [68, 0, 100, 255], [92, 0, 48, 255], [84, 4, 0, 255], [60, 24, 0, 255],
            [32, 42, 0, 255], [8, 58, 0, 255], [0, 64, 0, 255], [0, 60, 0, 255],
            [0, 50, 60, 255], [0, 0, 0, 255], [0, 0, 0, 255], [0, 0, 0, 255],
            [152, 150, 152, 255], [8, 76, 196, 255], [48, 50, 236, 255], [92, 30, 228, 255],
            [136, 20, 176, 255], [160, 20, 100, 255], [152, 34, 32, 255], [120, 60, 0, 255],
            [84, 90, 0, 255], [40, 114, 0, 255], [8, 124, 0, 255], [0, 118, 40, 255],
            [0, 102, 120, 255], [0, 0, 0, 255], [0, 0, 0, 255], [0, 0, 0, 255],
            [236, 238, 236, 255], [76, 154, 236, 255], [120, 124, 236, 255], [176, 98, 236, 255],
            [228, 84, 236, 255], [236, 88, 180, 255], [236, 106, 100, 255], [212, 136, 32, 255],
            [160, 170, 0, 255], [116, 196, 0, 255], [76, 208, 32, 255], [56, 204, 108, 255],
            [56, 180, 204, 255], [60, 60, 60, 255], [0, 0, 0, 255], [0, 0, 0, 255],
            [236, 238, 236, 255], [168, 204, 236, 255], [188, 188, 236, 255], [212, 178, 236, 255],
            [236, 174, 236, 255], [236, 174, 212, 255], [236, 180, 176, 255], [228, 196, 144, 255],
            [204, 210, 120, 255], [180, 222, 120, 255], [168, 226, 144, 255], [152, 226, 180, 255],
            [160, 214, 228, 255], [160, 162, 160, 255], [0, 0, 0, 255], [0, 0, 0, 255],
        ];
        
        for (i, color) in nes_colors.iter().enumerate() {
            self.set_color(i, color[0], color[1], color[2], color[3]);
        }
    }
}

/// Video system for handling graphics output
pub struct VideoSystem {
    config: VideoConfig,
    window: Option<Window>,
    event_loop: Option<EventLoop<()>>,
    surface: Option<wgpu::Surface>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    render_pipeline: Option<wgpu::RenderPipeline>,
    texture: Option<wgpu::Texture>,
    texture_view: Option<wgpu::TextureView>,
    framebuffer: Arc<Mutex<Vec<u8>>>,
    palette: ColorPalette,
    current_frame: u64,
}

impl VideoSystem {
    pub fn new(config: &ConfigVideoConfig) -> EmuliteResult<Self> {
        let video_config = VideoConfig {
            width: config.width,
            height: config.height,
            scale: config.scale,
            fullscreen: config.fullscreen,
            vsync: config.vsync,
            filter: VideoFilter::Nearest,
            enabled: true,
        };
        
        Self::new_with_video_config(&video_config)
    }
    
    pub fn new_with_video_config(config: &VideoConfig) -> EmuliteResult<Self> {
        let mut video_system = Self {
            config: config.clone(),
            window: None,
            event_loop: None,
            surface: None,
            device: None,
            queue: None,
            render_pipeline: None,
            texture: None,
            texture_view: None,
            framebuffer: Arc::new(Mutex::new(vec![0; (config.width * config.height) as usize])),
            palette: ColorPalette::new(),
            current_frame: 0,
        };
        
        // Initialize NES palette by default
        video_system.palette.load_nes_palette();
        
        // Initialize window and graphics
        video_system.initialize_graphics()?;
        
        Ok(video_system)
    }
    
    fn initialize_graphics(&mut self) -> EmuliteResult<()> {
        // Note: We don't create our own EventLoop here since eframe already provides one
        // The window will be created by eframe and passed to us when needed
        log::info!("Video system initialized (window will be created by eframe)");
        
        // Graphics initialization will be done when we have access to the eframe window
        // For now, just prepare the framebuffer
        
        Ok(())
    }
    
    fn create_render_pipeline(
        device: &wgpu::Device,
        texture_view: &wgpu::TextureView,
    ) -> wgpu::RenderPipeline {
        let vs_src = include_str!("shaders/vertex.wgsl");
        let fs_src = include_str!("shaders/fragment.wgsl");
        
        let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(vs_src.into()),
        });
        
        let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(fs_src.into()),
        });
        
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }
    
    /// Update video system (called each frame)
    pub fn update(&mut self) -> EmuliteResult<()> {
        self.current_frame += 1;
        
        // Update texture with current framebuffer
        if let (Some(device), Some(queue), Some(texture)) = 
            (self.device.as_ref(), self.queue.as_ref(), self.texture.as_ref()) {
            
            let framebuffer = self.framebuffer.lock().unwrap();
            let mut rgba_data = Vec::new();
            
            // Convert palette indices to RGBA
            for &pixel in framebuffer.iter() {
                let color = self.palette.get_color(pixel as usize);
                rgba_data.extend_from_slice(&color);
            }
            
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &rgba_data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.config.width * 4),
                    rows_per_image: Some(self.config.height),
                },
                wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1,
                },
            );
        }
        
        Ok(())
    }
    
    /// Set pixel in framebuffer
    pub fn set_pixel(&mut self, x: u32, y: u32, color_index: u8) -> EmuliteResult<()> {
        if x >= self.config.width || y >= self.config.height {
            return Err(EmuliteError::VideoError("Pixel coordinates out of bounds".to_string()));
        }
        
        let mut framebuffer = self.framebuffer.lock().unwrap();
        let index = (y * self.config.width + x) as usize;
        if index < framebuffer.len() {
            framebuffer[index] = color_index;
        }
        
        Ok(())
    }
    
    /// Get pixel from framebuffer
    pub fn get_pixel(&self, x: u32, y: u32) -> EmuliteResult<u8> {
        if x >= self.config.width || y >= self.config.height {
            return Err(EmuliteError::VideoError("Pixel coordinates out of bounds".to_string()));
        }
        
        let framebuffer = self.framebuffer.lock().unwrap();
        let index = (y * self.config.width + x) as usize;
        Ok(framebuffer.get(index).copied().unwrap_or(0))
    }
    
    /// Clear framebuffer
    pub fn clear(&mut self, color_index: u8) {
        let mut framebuffer = self.framebuffer.lock().unwrap();
        framebuffer.fill(color_index);
    }
    
    /// Set color in palette
    pub fn set_palette_color(&mut self, index: usize, r: u8, g: u8, b: u8, a: u8) {
        self.palette.set_color(index, r, g, b, a);
    }
    
    /// Load palette from file
    pub fn load_palette(&mut self, path: &str) -> EmuliteResult<()> {
        // This would load a palette file
        // For now, just use the default NES palette
        self.palette.load_nes_palette();
        Ok(())
    }
    
    /// Get current configuration
    pub fn config(&self) -> &VideoConfig {
        &self.config
    }
    
    /// Set resolution
    pub fn set_resolution(&mut self, width: u32, height: u32) -> EmuliteResult<()> {
        self.config.width = width;
        self.config.height = height;
        
        // Recreate framebuffer
        self.framebuffer = Arc::new(Mutex::new(vec![0; (width * height) as usize]));
        
        // Recreate texture
        if let (Some(device), Some(texture)) = (self.device.as_ref(), self.texture.as_ref()) {
            let new_texture = device.create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
                label: None,
            });
            
            self.texture = Some(new_texture);
            self.texture_view = Some(self.texture.as_ref().unwrap().create_view(&wgpu::TextureViewDescriptor::default()));
        }
        
        Ok(())
    }
    
    /// Take screenshot
    pub fn screenshot(&self, path: &str) -> EmuliteResult<()> {
        let framebuffer = self.framebuffer.lock().unwrap();
        let mut image = RgbaImage::new(self.config.width, self.config.height);
        
        for (i, &pixel) in framebuffer.iter().enumerate() {
            let x = i as u32 % self.config.width;
            let y = i as u32 / self.config.width;
            let color = self.palette.get_color(pixel as usize);
            image.put_pixel(x, y, Rgba(color));
        }
        
        image.save(path)?;
        Ok(())
    }
}

impl Drop for VideoSystem {
    fn drop(&mut self) {
        // Cleanup is handled automatically by dropping the fields
    }
}
