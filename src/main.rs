extern crate generic_game as gg;
extern crate nalgebra as na;
extern crate time;
extern crate num;
extern crate rand;
#[macro_use]
extern crate glium;
extern crate rusttype;

use gg::debug::*;
use gg::{debug, input, window, handler_basic, Handler};
use gg::rendering::DisplaySettings;
use std::env;
mod polar_game;
mod rendering;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    debug::set_flags(DebugFlags::NOLOGGING);
    debug(&format!("Starting Up - Date: {}", time::now_utc().ctime()));
    let error_writer = Box::new(ErrorWriter::new());

    let display_settings = DisplaySettings {
        res: (1920, 1080),
        fullscreen: true,
        text_glyph_detail: 128.0,
            ..Default::default()
    };

    let renderer = Box::new(::rendering::glium_renderer::GliumRenderer::new(display_settings));
    let input_handler: Box<input::InputHandler> = Box::new(input::multihandler::MultiInput::new());
    let window_handler: Box<window::WindowHandler> = Box::new(window::GlutinInput::new());

    let game = Box::new(polar_game::PolarGameBuilder::default().build_game());
    let mut handler: Box<Handler> = Box::new(handler_basic::HandlerBasic::new(renderer, input_handler, window_handler, game));

    handler.init();
    while !handler.should_exit() {
        debug_clock_start_main();
        handler.update_input();
        handler.update_rendering();
        handler.update_logic();
        debug_clock_stop_main();
    }
    handler.on_exit();
}
