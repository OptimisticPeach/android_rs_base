# android_base
__- A base to create android applications with ease__  
`android_base` provides an api to develop applications in rust for android without worrying about android specific details like opengl implementations or the event loop, all while having its exposed functions be very abstract in terms of what you can do
## example
This is basically the same example as shown in the [piston getting started page](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started), except with inverted colours.
> Note that this example requires some basic knowledge of rust and working with android

Getting started:
Make sure you've followed the `getting set up` section of this readme beforehand to setup the environment/dependencies
#### _Setting up the project_
To set up the cargo project follow the standard procedure you'd go through with for any other binary rust application:
1. Run `cargo new android_example --bin` in the directory you want to create the project in
2. Inside the newly created `android_example` folder, edit the `Cargo.toml` file and add `android_base = "0.1.0"` as a dependency
> Take note that this is the only dependency you are required to add; android_base automatically imports the right versions of all the crates you'll need in the form of a prelude
#### _Writing the code_
3. Now we'll start writing the app. For simplicity's sake you can just copy the following example, but feel free to experiment and play around with it.
```rust
#![feature(uniform_paths)]
//automatically imports right versions of crates
use android_base::*;
use graphics::*;
use android_glue::*;

pub struct App { 
    rotation: f64
}

impl App {
    pub fn new() -> Self {
        Self {rotation: 0.}
    }
}

impl AppImpl for App {
    fn draw(&mut self, c: Context, gl: &mut GlGraphics, args: &RenderArgs){
        clear([1., 0., 0., 1.], gl);
        let transform = c.transform
                            .trans(args.width as f64 / 2., args.height as f64 / 2.)
                            .rot_rad(self.rotation)
                            .trans(-75., -75.);
        rectangle([0., 1., 0., 1.], [0., 0., 150., 150.], transform, gl);
    }
    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += args.dt;
    }
    fn cancel_poll(&self) -> bool {
        false
    }
}

fn main() {
    enable_backtrace();
    let mut container = AppContainer::init(App::new(), AppConfig::new());
    container.run();
}
```
4. Now let's tear this down piece by piece:
   1. Because prelude importing extern crates' members is unstable, we add a flag to enable this, hence the `#![feature(uniform_paths)]` at the start
   2. Then we import `android_base` and two important dependencies in making an android app with piston; [`graphics`](https://github.com/PistonDevelopers/graphics) and [`android_glue`](https://github.com/tomaka/android-rs-glue/tree/master/glue). For now I'll just assume you know what `graphics` does, but `android_glue` serves as the, well, glue between your app and android's events
   3. To basically copy what the piston example does, we create a similar struct containing our app, holding a rotation; but notice that we don't have to hold any kind of opengl stuff or even have to import it, there'll be more on this later when I'm talking about the `draw` method.
   4. We have added a `new` function for simplicity, but we could just use the constructor instead, either way works
   5. We then implement a pretty self-explanatory trait for the app, `AppImpl`, which holds all the important functions that can be implemented for your app:
      1. `draw`: This holds all the drawing functionality for your app, and takes the parameters supplied from `GlGraphics`' draw method, so that `android_base` takes care of getting ready to draw, and you just draw
      2. `update`: This holds information when your app is needed to draw; this is basically just a passthrough from `piston::event_loop::Events::next()` when it emits an update flag.
      3. `cancel_poll`: This polls your app if it should stop running, when it returns true, it will stop executing, even if it is only meant to render a numbered amount of frames (Look at the part on configuration below)
      4. There are many more functions you can take a look at that you can override in `src/app_implementor.rs`
   6. In our main function we do the following:
      1. Set the environment variable `RUST_BACKTRACE` to 1 to allow debugging backtraces; this just makes debugging easier
      2. We initialize an `AppContainer` with our app, and a default configuration. Configuring the `AppContainer` with an `AppConfig` can make it run only a certain number of frames, or call a reset function on `run()`
      3. Which leads us to the `run` function which will run our app until `cancel_poll` returns true (Which it won't because it's set to false)
#### _Building and running with `cargo-apk`_
5. We then need to prepare to build, so add the following to your `Cargo.toml`:
```
[package.metadata.android]

build_targets = [ "arm-linux-androideabi" ]
```
> Note that this example will follow the steps for compiling to a real device running arm linux android and will __not__ detail how to start an emulator or even how to install one

6. And then we'll install the necessary target for rustc:
```bash
$ rustup target add arm-linux-androideabi
```
7. And to get adb ready:
   1. Connect your device physically to the computer and run `adb tcpip 5555`
   2. Disconnect your device and find out the ip address (Usually found out by going to your internet settings and tapping on your currently connected network)
   3. Run `adb connect <your ip address>` to connect over wifi to make things easier for repeated runs
8. And now run `cargo-apk run`, and if it doesn't work:
   1. Open a separate terminal and run `adb logcat` and re-open your app
   2. The call to `enable_backtrace` in `main` will print out a backtrace when `panic!()` occurs
   3. Post an issue describing your problem, and me or someone else will help you debug it

## Getting set up
1. Follow the instructions for installing rust at rustup.rs
2. Install cargo directly from github by calling
```
cargo install --git https://github.com/tomaka/android-rs-glue.git --rev 1d095846a687f873b6aed7d62ee32bc5a062e534 --force cargo-apk
```
3. Follow the instructions for getting [manual usage](https://github.com/tomaka/android-rs-glue#manual-usage) ready under `Setting up your environment` but without re-installing cargo-apk because the published version is out of date
4. If when following the above instructions for compiling and running an example you get a missing dependency error of some sort, look it up online to figure out how to install it for your system, and if not, open an issue to get some help figuring it out
----
> Note: This readme/setup/example is meant for users either on ubuntu or on on WSL (Which I personally use), I've found that windows directly doesn't work

> Another note: due to a minor implementation detail in the latest version of `cargo-apk` is broken and you can't call `cargo apk` and instead have to call `cargo-apk`.

> Another note: This crate includes a git clone of [`opengles_graphics`](https://github.com/Drakulix/opengles_graphics) which is a fork of `opengl_graphics` meant to work with opengles, but it hasn't been updated for over 2 years. I therefore updated its dependencies to accomodate for newer versions of other crates. I __do not__ claim to have developed the entirety of it, and have only made minor changes, and instead give credit to [Drakulix](https://github.com/Drakulix) and the [Piston team](https://github.com/PistonDevelopers)