# Hungry Pixel (Rust edition)

I once used LÖVE to implement a simple game where a player control a pixel, eats other pixels and grow. This is my second attempt: in order to get familiar with Rust, SDL2 and to develop my game architecture (described later) I decided to implement a simple game.

## Technologies used

I've decided to use Rust, along with `rust-sdl2`. I also use `image` and `ttf` features of `rust-sdl2`.

## Building the game

### Prerequisites

First of all, ensure that you have dev versions of `sdl2`, `sdl2-image` and `sdl2-ttf`. You should be able to obtain them either using your OS package manager or by downloading them from [SDL2 site](https://www.libsdl.org/download-2.0.php) and then following instructions in `INSTALL` file. If you're on Windows you should add directory with .dll files to your `PATH` to run the built game.

### Building the game

Next, clone the repository and run `cargo build --release` in it. You will need to move `target/release/hungry-pixel-rs` executable along with `resources` dir somewhere and then you'll be able to run it. As an alternative, you can use `cargo run` command to just run the dev build.

Linux users can run `sh scripts/release.sh` from repository root to create a `.tar.gz` archive with everything needed to play (including a shell-script `run.sh`).

#### Building under Windows

If you want to build the game under Windows, you will have to do the following steps:

1. Install Rust and Cargo (preferrable way to do so is to use rustup.rs)
2. Install MinGW (you will likely want to install MSYS as well) and make sure its `bin` directory is added to PATH.
3. Use `rust-x86_64-pc-windows-stable` (or the corresponding 32 bit version) as your target in rustup
4. Download MinGW dev libs from SDL2 site. You will need SDL2, SDL2_image and SDL2_ttf.
5. Unpack the archives downloaded in previous step, and put all `.la` and `.a` files from `lib` directory in archive to your rustup toolchain `lib` folder, e.g. `C:\Users\<your username>\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\lib`
6. Put `.dll` files from the archives somewhere in your PATH (MinGW `bin` directory will do just fine) and also save them somewhere if your intention is to create a standalone executable archive.
7. Run MSYS prompt and put hungry-pixel-rs sources in your MSYS `home` directory (it is located within MinGW path)
8. Use `cargo build --release` to build the game, then copy resources and executable to target directory along with all the `dll` files.

You will likely need the latest MSVC redistributable which can be obtained via Windows Update or from Microsoft site.

## Architectural overview

Game architecture is based on a notion of "States". A `State` is an entity that knows how to respond to `Messages` and how to `render` itself on screen. States are gathered into a stack of states that resides in the `Engine` entity. `Engine` also has a queue of `Messages`.

Inside the `Engine`, a loop is running:

1. First, the events from SDL are collected. Some of them are translated into `Messages` that are added to MessageQueue.
2. Then the rendering happens. All the states are analyzed, searching the first that returns `true` when `is_fullscreen` is called on it. Then they are rendered in the reverse order, from that state to the top of the stack.
3. Then, messages are propagated down the `States` stack. Every `State` in stack can either consume, transform or pass the message. Consuming happens when `process_message` fn returns `None`, and makes the `Engine` process next message right away. `State` can transform message by returning a `Some(Message)` variant of `Option<Message>` type from `process_message` fn, or pass it by returning the original message in `Some` variant. The message is then propagated to the next `State` in queue and passed to the `process_message` method of it.
4. When the message is either consumed or propagated all the way through the stack, it is processed by the `Engine` itself and next message is consumed.

## Licensing information

The game is distributed under `MIT` license. Font used (`PressStart2P-Regular.ttf`) is distributed under [OFL License](http://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL_web).

```
Copyright (c) 2017 Crazy-Owl (https://github.com/Crazy-Owl)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```