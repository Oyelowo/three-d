
use crate::core::*;
use std::rc::Rc;

#[derive(Default)]
pub struct CPUMaterial {
    pub name: String,
    pub color: Option<(f32, f32, f32, f32)>,
    pub texture_image: Option<image::DynamicImage>,
    pub diffuse_intensity: Option<f32>,
    pub specular_intensity: Option<f32>,
    pub specular_power: Option<f32>
}

impl CPUMaterial {
    pub fn new(name: &str, texture_image: Option<image::DynamicImage>, color: Option<(f32, f32, f32, f32)>,
               diffuse_intensity: Option<f32>, specular_intensity: Option<f32>, specular_power: Option<f32>) -> Result<Self, Error> {
        Ok(Self { name: name.to_string(), texture_image, color, diffuse_intensity, specular_intensity, specular_power })
    }
}

#[derive(Clone)]
pub enum ColorSource {
    Color(Vec4),
    Texture(Rc<Texture2D>)
}

#[derive(Clone)]
pub struct Material {
    pub name: String,
    pub color_source: ColorSource,
    pub diffuse_intensity: Option<f32>,
    pub specular_intensity: Option<f32>,
    pub specular_power: Option<f32>
}

impl Material {
    pub fn new(gl: &Gl, cpu_material: &CPUMaterial) -> Result<Self, Error> {
        let color_source = if let Some(ref img) = cpu_material.texture_image {
            use image::GenericImageView;
            ColorSource::Texture(Rc::new(texture::Texture2D::new_with_u8(&gl, Interpolation::Linear, Interpolation::Linear,
                                                                  Some(Interpolation::Linear), Wrapping::Repeat, Wrapping::Repeat,
                                                                  img.width(), img.height(), &img.to_bytes())?))
        }
        else {
            ColorSource::Color(cpu_material.color.map(|(r, g, b, a)| vec4(r, g, b, a)).unwrap_or(vec4(1.0, 1.0, 1.0, 1.0)))
        };
        Ok(Material {name: cpu_material.name.clone(), color_source, diffuse_intensity: cpu_material.diffuse_intensity,
            specular_intensity: cpu_material.specular_intensity, specular_power: cpu_material.specular_power})
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            name: "default".to_string(),
            color_source: ColorSource::Color(vec4(1.0, 1.0, 1.0, 1.0)),
            diffuse_intensity: None,
            specular_intensity: None,
            specular_power: None
        }
     }
}