use crate::graphics::draw_roads;
pub use crate::intersection::Intersection;
use crate::text_renderer::TextRenderer;
pub use crate::vehicle::Vehicle;
pub use piston_window::Key;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use crate::statistics::Statistics;
pub struct Simulation {
    pub intersection: Intersection,
    pub statistics: Statistics,
    pub show_stats: bool,
    text_renderer: TextRenderer,
}

impl Simulation {
    pub fn new() -> Result<Self, String>  {
        Ok(Self {
            intersection: Intersection::new(),
            statistics: Statistics::new(50.0),  // 50.0 est la distance de sécurité
            show_stats: false,
            text_renderer: TextRenderer::new()?,
        })
    }

    pub fn handle_keypress(&mut self, key: Keycode) {
        match key {
            Keycode::Up => {
                let stats_id = self.statistics.add_vehicle(2.0);
                self.intersection.add_vehicle(Vehicle::new([100.0, 500.0], 2.0, stats_id));
            },
            Keycode::Down => {
                let stats_id = self.statistics.add_vehicle(2.0);
                self.intersection.add_vehicle(Vehicle::new([700.0, 100.0], -2.0, stats_id));
            },
            Keycode::Escape => {
                self.show_stats = true;
            },
            _ => {}
        }
    }

    pub fn update(&mut self) {
        // Appelle la mise à jour de l'intersection
        self.intersection.update(0.016); // Par exemple, delta_time = 16ms

        // Vérifier les distances de sécurité
        let vehicle_positions: Vec<(f64, f64)> = self.intersection.vehicles
            .iter()
            .map(|v| (v.position[0], v.position[1]))
            .collect();
        
        self.statistics.check_close_calls(&vehicle_positions);
        
        // Mettre à jour les statistiques de vitesse
        for vehicle in &self.intersection.vehicles {
            if let Some(stats) = self.statistics.vehicle_stats.get_mut(vehicle.stats_id) {
                stats.update(vehicle.velocity);
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        // Dessiner les routes
        draw_roads(canvas);

        // // Dessiner les véhicules
        // for vehicle in &self.intersection.vehicles {
        //     crate::graphics::draw_vehicle(canvas, vehicle);
        // }

        if self.show_stats {
            let _ = self.statistics.display_stats(canvas, &self.text_renderer);
        }
    }
    
}
