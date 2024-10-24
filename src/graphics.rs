use sdl2::{render::Canvas, rect::Rect};
use crate::vehicle::Vehicle;

pub fn draw_vehicle(canvas: &mut Canvas<sdl2::video::Window>, vehicle: &Vehicle) {
    let rect = Rect::new(vehicle.position[0] as i32, vehicle.position[1] as i32, 50, 100);
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0)); // Couleur du véhicule
    canvas.fill_rect(rect).unwrap();
}
