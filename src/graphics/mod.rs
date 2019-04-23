//! Draw your game with an explicit 2D graphics API.
//!
//! Graphics in Coffee focus on simplicity while __reducing global state__.
//! Operations like matrix transformations, off-screen rendering and draw calls
//! have always an explicit scope. In Coffee, you do not have to remember to pop
//! a transformation from the matrix stack, reset the render target, reset the
//! screen coordinates, etc. There are no global functions!
//!
//! # Basic concepts
//! The graphics module revolves around three concepts: [graphics processors],
//! [targets], and [resources].
//!
//! ## Graphics processors
//! A [`Gpu`] represents a link between your game and a graphics processor. It
//! is necessary to perform any kind of graphical operation, like loading
//! resources and drawing.
//!
//! As of now, you will only have one [`Gpu`] available at a given time.
//! However, in the future, the graphics module may allow recording graphical
//! operations concurrently.
//!
//! ## Targets
//! A [`Target`] represents a drawable target on a specific [`Gpu`]. A
//! [`Transformation`] can be applied to them.
//!
//! Any kind of draw operation needs an explicit [`Target`]. For example,
//! [`Image::draw`] needs a reference to a [`Target`] as the last argument.
//!
//! Currently, there are two ways to obtain a [`Target`]: either from a
//! [`Frame`] or a [`Canvas`], using the `as_target` method.
//!
//! ## Resources
//! A resource is the source of some drawable object. In Coffee, there is no
//! `Resource` or `Drawable` type/trait. Resources are represented by different
//! types like [`Image`], [`Font`], [`TextureArray`], etc.
//!
//! # Getting started
//! You should probably start your [`Game::draw`] implementation by getting a
//! [`Frame`] and clearing it:
//!
//! ```
//! use coffee::{Game, Timer};
//! use coffee::graphics::{Window, Color};
//! # use coffee::Result;
//! # use coffee::graphics::Gpu;
//! #
//! # struct MyGame;
//!
//! impl Game for MyGame {
//! #   type View = ();
//! #   type Input = ();
//! #
//! #   const TICKS_PER_SECOND: u16 = 60;
//! #
//! #   fn new(window: &mut Window) -> Result<(MyGame, Self::View, Self::Input)> {
//! #       Ok((MyGame, (), ()))
//! #   }
//! #
//!     // ...
//!
//! #   fn interact(&mut self, _input: &mut Self::Input,
//! #              _view: &mut Self::View, _gpu: &mut Gpu) {}
//! #
//! #   fn update(&mut self, _view: &Self::View, window: &Window) {}
//! #
//!     fn draw(
//!         &self,
//!         _view: &mut Self::View,
//!         window: &mut Window,
//!         _timer: &Timer,
//!     ) {
//!         let mut frame = window.frame();
//!         frame.clear(Color::BLACK);
//!
//!         // Use your resources here...
//!         // view.image.draw(Sprite { ... }, &mut frame.as_target());
//!     }
//! }
//! ```
//!
//! You can load your resources during [`Game::new`]. Check out the different
//! types in this module to get a basic understanding of which kind of resources
//! are supported.
//!
//! [graphics processors]: #graphics-processors
//! [targets]: #targets
//! [resources]: #resources
//! [`Gpu`]: struct.Gpu.html
//! [`Target`]: struct.Target.html
//! [`Transformation`]: struct.Transformation.html
//! [`Frame`]: struct.Frame.html
//! [`Canvas`]: struct.Canvas.html
//! [`Image`]: struct.Image.html
//! [`Image::draw`]: struct.Image.html#method.draw
//! [`TextureArray`]: texture_array/struct.TextureArray.html
//! [`Font`]: struct.Font.html
//! [`Game::draw`]: ../trait.Game.html#tymethod.draw
//! [`Game::new`]: ../trait.Game.html#tymethod.new
#[cfg(feature = "opengl")]
mod backend_gfx;
#[cfg(feature = "opengl")]
use backend_gfx as gpu;

#[cfg(feature = "vulkan")]
mod backend_wgpu;
#[cfg(feature = "vulkan")]
use backend_wgpu as gpu;

mod batch;
mod canvas;
mod color;
mod font;
mod image;
mod point;
mod quad;
mod rectangle;
mod sprite;
mod target;
mod text;
mod transformation;
mod vector;

pub mod texture_array;
pub(crate) mod window;

pub use self::image::Image;
pub use batch::Batch;
pub use canvas::Canvas;
pub use color::Color;
pub use font::Font;
pub use gpu::Gpu;
pub use point::Point;
pub use quad::{IntoQuad, Quad};
pub use rectangle::Rectangle;
pub use sprite::Sprite;
pub use target::Target;
pub use text::Text;
pub use texture_array::TextureArray;
pub use transformation::Transformation;
pub use vector::Vector;
pub use window::{Frame, Settings as WindowSettings, Window};
