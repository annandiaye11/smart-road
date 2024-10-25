// statistics.rs
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Instant;
use crate::text_renderer::TextRenderer;

#[derive(Clone, Copy)]
pub struct VehicleStats {
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub max_velocity: f64,
    pub min_velocity: f64,
}

impl VehicleStats {
    pub fn new(initial_velocity: f64) -> Self {
        Self {
            start_time: Instant::now(),
            end_time: None,
            max_velocity: initial_velocity,
            min_velocity: initial_velocity,
        }
    }

    pub fn update(&mut self, current_velocity: f64) {
        self.max_velocity = f64::max(self.max_velocity, current_velocity);  // Correction ici
        self.min_velocity = f64::min(self.min_velocity, current_velocity);  // Correction ici
    }

    pub fn complete(&mut self) {
        self.end_time = Some(Instant::now());
    }

    pub fn crossing_time(&self) -> Option<f64> {
        self.end_time.map(|end| {
            end.duration_since(self.start_time).as_secs_f64()
        })
    }
}

pub struct Statistics {
    pub vehicle_stats: Vec<VehicleStats>,
    pub vehicles_passed: usize,
    pub close_calls: usize,
    pub safe_distance: f64,
}

impl Statistics {
    pub fn new(safe_distance: f64) -> Self {
        Self {
            vehicle_stats: Vec::new(),
            vehicles_passed: 0,
            close_calls: 0,
            safe_distance,
        }
    }

    pub fn add_vehicle(&mut self, initial_velocity: f64) -> usize {
        let stats = VehicleStats::new(initial_velocity);
        self.vehicle_stats.push(stats);
        self.vehicle_stats.len() - 1
    }

    pub fn check_close_calls(&mut self, vehicles: &[(f64, f64)]) {
        for i in 0..vehicles.len() {
            for j in (i + 1)..vehicles.len() {
                let (x1, y1) = vehicles[i];
                let (x2, y2) = vehicles[j];
                let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                if distance < self.safe_distance {
                    self.close_calls += 1;
                }
            }
        }
    }

    pub fn get_summary(&self) -> StatsSummary {
        let mut max_velocity = 0.0f64;  // Spécifie explicitement le type
        let mut min_velocity = f64::INFINITY;
        let mut max_time = 0.0f64;  // Spécifie explicitement le type
        let mut min_time = f64::INFINITY;

        for stats in &self.vehicle_stats {
            max_velocity = f64::max(max_velocity, stats.max_velocity);  // Correction ici
            min_velocity = f64::min(min_velocity, stats.min_velocity);  // Correction ici
            
            if let Some(time) = stats.crossing_time() {
                max_time = f64::max(max_time, time);  // Correction ici
                min_time = f64::min(min_time, time);  // Correction ici
            }
        }

        StatsSummary {
            total_vehicles: self.vehicles_passed,
            max_velocity,
            min_velocity,
            max_crossing_time: max_time,
            min_crossing_time: min_time,
            close_calls: self.close_calls,
        }
    }

    pub fn display_stats(&self, canvas: &mut Canvas<Window>, text_renderer: &TextRenderer) -> Result<(), String> {
        let summary = self.get_summary();
        
        // Fond de la fenêtre de statistiques
        canvas.set_draw_color(Color::RGB(40, 44, 52));
        let stats_window = Rect::new(200, 150, 650, 450);
        canvas.fill_rect(stats_window)?;
        
        // Bordure de la fenêtre
        canvas.set_draw_color(Color::RGB(97, 175, 239));
        canvas.draw_rect(stats_window)?;
        
        // Prépare les lignes de texte avec ownership explicite
        let lines = vec![
            String::from("Traffic Simulation Statistics"),
            String::new(),
            format!("Total Vehicles: {}", summary.total_vehicles),
            format!("Maximum Velocity: {:.2} units/s", summary.max_velocity),
            format!("Minimum Velocity: {:.2} units/s", summary.min_velocity),
            format!("Maximum Crossing Time: {:.2} seconds", summary.max_crossing_time),
            format!("Minimum Crossing Time: {:.2} seconds", summary.min_crossing_time),
            format!("Close Calls: {}", summary.close_calls),
        ];
        
        // Convertit les String en &str pour render_text_lines
        let lines_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
        
        // Rend le texte
        text_renderer.render_text_lines(
            canvas,
            &lines_refs,
            220,
            170,
            Color::RGB(255, 255, 255),
        )?;
        
        Ok(())
    }
}

pub struct StatsSummary {
    pub total_vehicles: usize,
    pub max_velocity: f64,
    pub min_velocity: f64,
    pub max_crossing_time: f64,
    pub min_crossing_time: f64,
    pub close_calls: usize,
}