#![allow(unused_variables)]
use opengles_graphics::*;
use graphics::*;
use piston::input::*;

/// A trait describing an implementation of a basic android rust app
pub trait AppImpl {
    /// Called when rotated, or when split-screen is enabled (Unsure about that last point)
    fn on_size_change(&mut self, new_size: &(usize, usize), old_size: &(usize, usize)) {}

    /// Called when need to draw
    /// Initialization and stuff is taken care of behind the scenes
    fn draw(&mut self, c: Context, gl: &mut GlGraphics, args: &RenderArgs);
    
    /// When focus is lost or gained, this function is called to let app save states or do anything it needs to do to save
    fn signal_pause_change(&mut self) {}
 
    /// Called when asked to update. Pretty standard piston/glutin_window update
    fn update(&mut self, args: &UpdateArgs);

    /// When android sends a motion, it's forwarded here
    fn motion(&mut self, motion: android_glue::Motion) {}

    /// Called at the start of `AppContainer::run` if `config` requires a reset on start
    fn reset_on_start(&mut self) {}

    /// Asks app if it wants to stop execution, considered even when running with a counted number of frames
    fn cancel_poll(&self) -> bool;

    /// Called just after `signal_pause_change` when focus is gained. Kind of meant to be the opposite to it, just more optional
    fn refresh(&mut self) {}

    /// Called with all other android events that `AppContainer` isn't ready to handle, usually not implemented
    fn handle(&mut self, event: android_glue::Event) {}
}