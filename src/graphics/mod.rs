mod canvas;
mod color;
mod draw_parameters;
mod gpu;
mod image;
mod point;
mod rectangle;
pub mod texture_array;
mod transformation;
mod vector;
pub mod window;

pub use self::image::Image;
pub use canvas::Canvas;
pub use color::Color;
pub use draw_parameters::DrawParameters;
pub use gpu::{Gpu, Target};
pub use point::Point;
pub use rectangle::Rectangle;
pub use texture_array::TextureArray;
pub use transformation::Transformation;
pub use vector::Vector;
pub use window::{Frame, Window};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}
