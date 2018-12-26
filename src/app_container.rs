use android_glue;
use crate::app_implementor::*;
use opengles_graphics::*;
use glutin::GlContext;
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use crate::app_config::*;

/// A utility struct for running an android application, to not have to worry about the minor
/// android-specific details when running and rendering an app with piston
pub struct AppContainer<T: AppImpl> {
    event_reciever: std::sync::mpsc::Receiver<android_glue::Event>,
    gl: GlGraphics,
    window: GlutinWindow,
    app: T,
    events: Events,
    window_size: (usize, usize),
    config: AppConfig
}

impl<T: AppImpl> AppContainer<T> {
    /// Creates an `AppContainer` with appropriate settings.
    /// Unsure what happens if you create two of them at the same time
    /// `app: T`: an instance of your struct which implements `AppImpl`
    /// `config: AppConfig`: a configuration setting with which to run your app like number of frames or reset options
    /// In more detail:
    /// 1. Creates a `GlutinWindow`
    /// 2. Loads Opengl pointers using the window's address
    /// 3. Prepares channels for use with `android_glue`
    /// 4. Creates an instance of `AppContainer` and fills in some other members
    pub fn init(app: T, config: AppConfig) -> Self {
        let android_window: GlutinWindow = WindowSettings::new("Glutin Window", (640, 480))
        .fullscreen(false)
        .opengl(shader_version::OpenGL::V2_0)
        .build()
        .unwrap();
        opengles_graphics::gl::load_with(|s| {
            android_window.window.get_proc_address(s) as *const std::ffi::c_void
        });
        let (sender, receiver) = std::sync::mpsc::channel();
        android_glue::add_sender(sender);
        Self {
            event_reciever: receiver,
            window: android_window,
            gl: GlGraphics::new(opengles_graphics::OpenGL::V3_1),
            app,
            events: Events::new(EventSettings::new()),
            window_size: (0, 0),
            config
        }
    }

    /// Returns if it ran render or not; useful for counting frames
    fn poll_event_loop(&mut self) -> bool {
        let mut rendered = false;
        while let Some(e) = self.events.next(&mut self.window) {
            match e {
                Event::Loop(loopargs) => match loopargs {
                    Loop::Render(rargs) => {
                        self.draw(&rargs);
                        rendered = true;
                    }
                    Loop::Update(uargs) => {
                        self.poll_android_events();
                        self.app.update(&uargs);
                    }
                    _ => {}
                },
                _ => {}
            }
        } 
        rendered
    }

    /// Prepares for draw, and then calls `self.app.draw` with the parameters it prepared
    fn draw(&mut self, rargs: &RenderArgs) {
        let app_ref = &mut self.app;
        let ws_ref = &mut self.window_size;
        self.gl.draw(rargs.viewport(), |c, gl| {
            if *ws_ref != (rargs.width as usize, rargs.height as usize) {
                let size_new = (rargs.width as usize, rargs.height as usize);
                app_ref.on_size_change(&size_new, ws_ref);
                *ws_ref = size_new;
            } else {
                app_ref.draw(c, gl, rargs);
            }
        });
    }

    /// Suspends thread until we get a GainedFocus
    /// A bit of a hack, but not using this leads to:
    /// calling `self.events.next()` which at some point tries to swap buffers crashing egl -- it's ugly
    fn wait_until_gain_focus(&mut self) {
        use android_glue::Event;
        use std::sync::mpsc::TryRecvError;
        loop{
            let recieved = self.event_reciever.recv();
            match recieved.unwrap() {
                Event::GainedFocus => {break;},
                _ => {}
            }
        }
        if let Err(error) = self.event_reciever.try_recv() {
            match error {
                TryRecvError::Disconnected => panic!(),
                _ => {}
            }
        }
    }

    /// Tries to recieve android events, and manages focus changes
    fn poll_android_events(&mut self) {
        use android_glue::Event;
        let mut flag = false;
        for event in self.event_reciever.try_iter(){
            match event {
                Event::EventMotion(motion) => self.app.motion(motion),
                Event::LostFocus => {
                    flag = true;
                    break;
                }
                misc => self.app.handle(misc)
            }
        }
        if flag {
            self.app.signal_pause_change();
            self.wait_until_gain_focus();
            self.app.signal_pause_change();
            self.app.refresh();
        }
    }

    /// Runs the application as per the configuration provided when `init` was called
    pub fn run(&mut self) {
        if self.config.reset_on_start {
            self.app.reset_on_start();
        }
        if let Some(x) = self.config.num_frames {
            'a: for _ in 0..x {
                while !self.app.cancel_poll() {
                    if self.poll_event_loop() {
                        continue 'a;
                    }
                }
            }
        }
        else {
            while !self.app.cancel_poll() {
                self.poll_event_loop();
            }
        }
    }
}
