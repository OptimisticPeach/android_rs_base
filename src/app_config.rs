/// Configuration for running an app in an `AppContainer<T>`
pub struct AppConfig {
    pub num_frames: Option<usize>,
    pub reset_on_start: bool,
}

impl AppConfig {
    /// Standard config: 
    /// `frames` = `None` to make it run until told not to
    /// `reset_on_start` = `true`
    pub fn new() -> Self {
        Self {
            num_frames: None,
            reset_on_start: true
        }
    }
    /// Sets or resets the number of frames to be run
    /// `x`: Number of frames to be run, leave as None to only depend on `AppImpl::cancel_poll`
    pub fn frames(mut self, x: Option<usize>) -> Self {
        self.num_frames = x;
        self
    }
    /// When set to true, will call `app.reset_on_start`
    pub fn reset_when_run(mut self, doit: bool) -> Self {
        self.reset_on_start = doit;
        self
    }
}
