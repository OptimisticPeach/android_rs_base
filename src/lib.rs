pub extern crate android_glue;
pub extern crate opengles_graphics;
pub extern crate graphics;
pub extern crate piston;
pub extern crate glutin_window;
pub extern crate glutin;

mod app_container;
mod app_implementor;
mod app_config;

pub use self::app_config::*;
pub use self::app_container::*;
pub use self::app_implementor::*;

// Useful to have pre-imported

pub use android_glue::*;
pub use opengles_graphics::GlGraphics;
pub use piston::input::{RenderArgs, UpdateArgs};
pub use graphics::Context;

/// Sets RUST_BACKTRACE=1 to enable backtraces in android, useful to get backtraces
pub fn enable_backtrace() {
    use std::env;
    let key = "RUST_BACKTRACE";
    env::set_var(key, "1");
}
