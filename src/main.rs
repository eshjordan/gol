mod gol;
use core::time;
use std::thread;

use gol::controller::Controller;

use error_iter::ErrorIter as _;
use log::error;
use winit::{window::Window, event_loop::{EventLoop, ControlFlow}};
use winit_input_helper::WinitInputHelper;

fn main() {

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = Window::new(&event_loop).unwrap();

    let mut controller = Controller::new(&window);
    let mut grid = controller.initialise_game();

    const DISPLAY_RATE: f32 = 60.0;
    const GAME_RATE: f32 = 20.0;
    const TICK_RATE: f32 = 1000.0;
    let mut game_rate_counter = 0;
    let mut display_rate_counter = 0;

    event_loop.run(move |event, _, control_flow| {

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(winit::event::VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = controller.resize_window(size) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            if input.key_pressed(winit::event::VirtualKeyCode::Space) {
                if controller.paused() {
                    controller.unpause();
                } else {
                    controller.pause();
                }
            }

            if input.mouse_held(0) {
                match input.mouse() {
                    Some((x, y)) => controller.manual_set(&mut grid, x, y),
                    _ => (),
                }
            }

            if input.mouse_held(1) {
                match input.mouse() {
                    Some((x, y)) => controller.manual_unset(&mut grid, x, y),
                    _ => (),
                }
            }

            // Update internal state at GAME_RATE Hz
            if game_rate_counter == 0 {
                controller.generate(&mut grid);
            }
            game_rate_counter += 1;
            game_rate_counter %= (TICK_RATE/GAME_RATE).floor() as u32;

            // Render at DISPLAY_RATE Hz
            if display_rate_counter == 0 {
                if let Err(err) = controller.display(&grid) {
                    log_error("pixels.render", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            display_rate_counter += 1;
            display_rate_counter %= (TICK_RATE/DISPLAY_RATE).floor() as u32;

            // Run the main loop at TICK_RATE Hz
            thread::sleep(time::Duration::from_millis((1000.0/TICK_RATE).floor() as u64));
        }
    });

}


fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
