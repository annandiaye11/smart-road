pub use crate::vehicle::Vehicle;
pub use sdl2::pixels::Color;
pub use sdl2::video::Window;
pub use sdl2::{rect::Rect, render::Canvas};

pub fn draw_vehicle(canvas: &mut Canvas<sdl2::video::Window>, vehicle: &Vehicle) {
    let rect = Rect::new(
        vehicle.position[0] as i32,
        vehicle.position[1] as i32,
        50,
        100,
    );
    canvas.set_draw_color(sdl2::pixels::Color::RGB(123, 5, 25)); // Couleur du véhicule
    canvas.fill_rect(rect).unwrap();
}

// Définition des constantes
const ROAD_WIDTH: u32 = 100; // Largeur des routes
const INTERSECTION_SIZE: u32 = 200; // Taille de l'intersection
const WINDOW_WIDTH: u32 = 800; // Largeur de la fenêtre
const WINDOW_HEIGHT: u32 = 600; // Hauteur de la fenêtre

pub fn draw_roads(canvas: &mut Canvas<Window>) {
    // Couleur des traits des routes
    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Noir pour les traits

    // Dessiner les routes horizontales
    for y in (0..WINDOW_HEIGHT).step_by(ROAD_WIDTH as usize) {
        let road = Rect::new(
            0,
            y as i32,
            WINDOW_WIDTH.try_into().unwrap(),
            ROAD_WIDTH.try_into().unwrap(),
        );
        canvas.draw_rect(road).unwrap(); // Dessiner le contour du rectangle
    }

    // Dessiner les routes verticales
    for x in (0..WINDOW_WIDTH).step_by(ROAD_WIDTH as usize) {
        let road = Rect::new(
            x as i32,
            0,
            ROAD_WIDTH.try_into().unwrap(),
            WINDOW_HEIGHT.try_into().unwrap(),
        );
        canvas.draw_rect(road).unwrap(); // Dessiner le contour du rectangle
    }

    // Dessiner les intersections
    let intersection = Rect::new(
        (WINDOW_WIDTH as i32 - INTERSECTION_SIZE as i32) / 2,
        (WINDOW_HEIGHT as i32 - INTERSECTION_SIZE as i32) / 2,
        INTERSECTION_SIZE,
        INTERSECTION_SIZE,
    );
    canvas.set_draw_color(Color::RGB(100, 100, 100)); // Couleur de l'intersection
    canvas.fill_rect(intersection).unwrap();
}
