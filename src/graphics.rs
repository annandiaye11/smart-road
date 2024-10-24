use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
// use std::f64::consts::PI;

// Constantes pour les dimensions
const WINDOW_WIDTH: u32 = 1050;
const WINDOW_HEIGHT: u32 = 850;
const ROAD_WIDTH: u32 = 120; // Largeur totale de la route (3 voies)
const LANE_WIDTH: u32 = 50; // Largeur d'une voie
// const INTERSECTION_SIZE: u32 = ROAD_WIDTH * 2; // Taille de l'intersection
// const LINE_WIDTH: u32 = 3; // Largeur des lignes de séparation

pub fn draw_roads(canvas: &mut Canvas<Window>) {
    // Fond de route en gris foncé
    canvas.set_draw_color(Color::RGB(50, 50, 50));
    
    // Routes horizontales
    let horizontal_road = Rect::new(
        0,
        (WINDOW_HEIGHT as i32 - ROAD_WIDTH as i32) / 2 - 95,
        WINDOW_WIDTH,
        ROAD_WIDTH + 190,
    );
    canvas.fill_rect(horizontal_road).unwrap();
    
    // Routes verticales
    let vertical_road = Rect::new(
        (WINDOW_WIDTH as i32 - ROAD_WIDTH as i32) / 2 - 95,
        0,
        ROAD_WIDTH + 190,
        WINDOW_HEIGHT,
    );
    canvas.fill_rect(vertical_road).unwrap();
    
    // Dessiner les lignes de séparation
    draw_lane_markings(canvas);
    
    // Dessiner les flèches directionnelles
    // draw_direction_arrows(canvas);
}

fn draw_lane_markings(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    
    // Position centrale
    let center_x = WINDOW_WIDTH as i32 / 2;
    let center_y = WINDOW_HEIGHT as i32 / 2;
    
    // Définir la zone d'intersection
    let intersection_margin = ROAD_WIDTH as i32 + 50;
    
    // Nombre de lignes
    let num_lines = 4;
    
    // Lignes horizontales
    for i in 0..num_lines {
        let offset = (i as i32) * LANE_WIDTH as i32;
        
        // Ligne au-dessus du centre
        let y_above = center_y - offset;
        draw_dashed_line(canvas, 
            0, y_above, 
            center_x - intersection_margin, y_above);
        draw_dashed_line(canvas,
            center_x + intersection_margin, y_above,
            WINDOW_WIDTH as i32, y_above);
            
        // Ligne en-dessous du centre
        let y_below = center_y + offset;
        draw_dashed_line(canvas, 
            0, y_below, 
            center_x - intersection_margin, y_below);
        draw_dashed_line(canvas,
            center_x + intersection_margin, y_below,
            WINDOW_WIDTH as i32, y_below);
    }
    
    // Lignes verticales
    for i in 0..num_lines {
        let offset = (i as i32) * LANE_WIDTH as i32;
        
        // Ligne à gauche du centre
        let x_left = center_x - offset;
        draw_dashed_line(canvas,
            x_left, 0,
            x_left, center_y - intersection_margin);
        draw_dashed_line(canvas,
            x_left, center_y + intersection_margin,
            x_left, WINDOW_HEIGHT as i32);
            
        // Ligne à droite du centre
        let x_right = center_x + offset;
        draw_dashed_line(canvas,
            x_right, 0,
            x_right, center_y - intersection_margin);
        draw_dashed_line(canvas,
            x_right, center_y + intersection_margin,
            x_right, WINDOW_HEIGHT as i32);
    }
    
    // Ajout des barres d'arrêt sur les 3 premières voies
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    
    // Barre d'arrêt pour les voies du haut (vers le bas)
    for i in 0..3 {
        let x_start = center_x - (i as i32 + 1) * LANE_WIDTH as i32;
        let x_end = center_x - (i as i32) * LANE_WIDTH as i32;
        canvas.draw_line(
            (x_start, center_y - intersection_margin),
            (x_end, center_y - intersection_margin)
        ).unwrap();
    }
    
    // Barre d'arrêt pour les voies de droite (vers la gauche)
    for i in 0..3 {
        let y_start = center_y - (i as i32 + 1) * LANE_WIDTH as i32;
        let y_end = center_y - (i as i32) * LANE_WIDTH as i32;
        canvas.draw_line(
            (center_x + intersection_margin, y_start),
            (center_x + intersection_margin, y_end)
        ).unwrap();
    }
    
    // Barre d'arrêt pour les voies du bas (vers le haut)
    for i in 0..3 {
        let x_start = center_x + (i as i32) * LANE_WIDTH as i32;
        let x_end = center_x + (i as i32 + 1) * LANE_WIDTH as i32;
        canvas.draw_line(
            (x_start, center_y + intersection_margin),
            (x_end, center_y + intersection_margin)
        ).unwrap();
    }
    
    // Barre d'arrêt pour les voies de gauche (vers la droite)
    for i in 0..3 {
        let y_start = center_y + (i as i32) * LANE_WIDTH as i32;
        let y_end = center_y + (i as i32 + 1) * LANE_WIDTH as i32;
        canvas.draw_line(
            (center_x - intersection_margin , y_start),
            (center_x - intersection_margin , y_end) 
        ).unwrap();
    }
}

fn draw_dashed_line(canvas: &mut Canvas<Window>, x1: i32, y1: i32, x2: i32, y2: i32) {
    let dash_length = 20.0;
    let gap_length = 20.0;
    let total_length = (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt();
    let num_dashes = (total_length / (dash_length + gap_length)).ceil() as i32;
    
    for i in 0..num_dashes {
        let start_percent = (i as f64 * (dash_length + gap_length)) / total_length;
        let end_percent = (i as f64 * (dash_length + gap_length) + dash_length) / total_length;
        
        let start_x = x1 as f64 + (x2 - x1) as f64 * start_percent;
        let start_y = y1 as f64 + (y2 - y1) as f64 * start_percent;
        let end_x = x1 as f64 + (x2 - x1) as f64 * end_percent.min(1.0);
        let end_y = y1 as f64 + (y2 - y1) as f64 * end_percent.min(1.0);
        
        canvas.draw_line(
            (start_x as i32, start_y as i32),
            (end_x as i32, end_y as i32)
        ).unwrap();
    }
}

// fn draw_direction_arrows(canvas: &mut Canvas<Window>) {
//     canvas.set_draw_color(Color::RGB(255, 255, 255));
    
//     let center_x = WINDOW_WIDTH as i32 / 2;
//     let center_y = WINDOW_HEIGHT as i32 / 2;
    
//     // Positions des voies
//     let lanes = [
//         (center_x - ROAD_WIDTH as i32 / 3, "right"),
//         (center_x, "straight"),
//         (center_x + ROAD_WIDTH as i32 / 3, "left"),
//     ];
    
//     // Dessiner les flèches pour chaque voie
//     for (x, direction) in lanes.iter() {
//         // Nord
//         draw_arrow(canvas, *x, center_y - ROAD_WIDTH as i32, *direction, 270.0);
//         // Sud
//         draw_arrow(canvas, *x, center_y + ROAD_WIDTH as i32, *direction, 90.0);
//         // Est
//         draw_arrow(canvas, center_x + ROAD_WIDTH as i32, *x, *direction, 0.0);
//         // Ouest
//         draw_arrow(canvas, center_x - ROAD_WIDTH as i32, *x, *direction, 180.0);
//     }
// }

// fn draw_arrow(canvas: &mut Canvas<Window>, x: i32, y: i32, direction: &str, base_angle: f64) {
//     let arrow_length = 20.0;
//     let arrow_width = 10.0;
    
//     let angle = match direction {
//         "right" => base_angle - 90.0,
//         "straight" => base_angle,
//         "left" => base_angle + 90.0,
//         _ => base_angle,
//     };
    
//     let rad = angle * PI / 180.0;
//     let end_x = x as f64 + arrow_length * rad.cos();
//     let end_y = y as f64 + arrow_length * rad.sin();
    
//     // Ligne principale
//     canvas.draw_line(
//         (x, y),
//         (end_x as i32, end_y as i32)
//     ).unwrap();
    
//     // Pointes de flèche
//     let head_angle1 = rad + PI * 0.8;
//     let head_angle2 = rad - PI * 0.8;
    
//     let head_x1 = end_x + arrow_width * head_angle1.cos();
//     let head_y1 = end_y + arrow_width * head_angle1.sin();
//     let head_x2 = end_x + arrow_width * head_angle2.cos();
//     let head_y2 = end_y + arrow_width * head_angle2.sin();
    
//     canvas.draw_line(
//         (end_x as i32, end_y as i32),
//         (head_x1 as i32, head_y1 as i32)
//     ).unwrap();
    
//     canvas.draw_line(
//         (end_x as i32, end_y as i32),
//         (head_x2 as i32, head_y2 as i32)
//     ).unwrap();
// }