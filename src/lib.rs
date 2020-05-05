extern crate iced;

pub mod core;
pub mod native;
pub mod style;
pub mod wgpu;

pub use crate::core::*;

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    pub use crate::wgpu::{
        h_slider,
    };

    #[doc(no_inline)]
    pub use {
        h_slider::HSlider,
    };
}

pub use platform::*;