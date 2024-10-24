use crate::graphics::draw_roads;
pub use crate::intersection::Intersection;
pub use crate::vehicle::Vehicle;
pub use piston_window::Key;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;

pub struct Simulation {
    pub intersection: Intersection,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            intersection: Intersection::new(),
        }
    }

    pub fn handle_keypress(&mut self, key: Keycode) {
        match key {
            Keycode::Up => self.intersection.add_vehicle(Vehicle::new([100.0, 500.0], 2.0)),
            Keycode::Down => self.intersection.add_vehicle(Vehicle::new([700.0, 100.0], -2.0)),
            _ => {}
        }
    }

    pub fn update(&mut self) {
        // Appelle la mise à jour de l'intersection
        self.intersection.update(0.016); // Par exemple, delta_time = 16ms
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        // Dessiner les routes
        draw_roads(canvas);

        // Dessiner les véhicules
        for vehicle in &self.intersection.vehicles {
            crate::graphics::draw_vehicle(canvas, vehicle);
        }
    }
    
}
