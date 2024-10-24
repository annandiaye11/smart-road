use std::time::{Duration, Instant};

pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::pixels::Color;
pub use sdl2::rect::Rect;
pub use sdl2::render::Canvas;
pub use sdl2::video::Window;
pub use sdl2::Sdl;

pub mod vehicle;
pub mod intersection;
pub mod simulation;
pub mod graphics;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem
        .window("Intersection Simulation", 1050, 850)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut sim = simulation::Simulation::new();
    let mut last_frame = Instant::now();

    loop {
        let delta_time = last_frame.elapsed();
        last_frame = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    sim.handle_keypress(Keycode::Up);
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    sim.handle_keypress(Keycode::Down);
                }
                _ => {}
            }
        }

        sim.update();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(201, 5, 123)); // Couleur de fond
        canvas.clear();
        sim.draw(&mut canvas); // Dessiner la simulation
        canvas.present();

        let frame_duration = Duration::from_millis(16);
        if delta_time < frame_duration {
            std::thread::sleep(frame_duration - delta_time);
        }
    }
}
