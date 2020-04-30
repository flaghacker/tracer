use imgref::ImgRefMut;

use crate::common::scene::{Color, Scene};

pub mod scene;
pub mod util;
pub mod math;

pub trait Renderer {
    fn render(&self, scene: &Scene, target: ImgRefMut<Color>);
}
