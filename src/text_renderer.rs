use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;

pub struct TextRenderer {
    font: sdl2::ttf::Font<'static, 'static>,
}

impl TextRenderer {
    pub fn new() -> Result<Self, String> {
         // On crée un Box pour ttf_context et on le leak
         let ttf_context = Box::leak(
            Box::new(sdl2::ttf::init().map_err(|e| e.to_string())?)
        );
        
        let font_path: &'static str = Box::leak(
            String::from("assets/fonts/Roboto-Regular.ttf").into_boxed_str()
        );
        
        let font = ttf_context
            .load_font(font_path, 24)
            .map_err(|e| e.to_string())?;

        Ok(Self {
            font,
        })
    }

    pub fn render_text_lines(
        &self,
        canvas: &mut Canvas<Window>,
        lines: &[&str],
        x: i32,
        y: i32,
        color: Color
    ) -> Result<(), String> {
        let texture_creator = canvas.texture_creator();
        let mut y_offset = y;
        let line_height = 30; // Espacement entre les lignes

        for line in lines {
            let surface = self.font
                .render(line)
                .blended(color)
                .map_err(|e| e.to_string())?;

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let TextureQuery { width, height, .. } = texture.query();
            let target = Rect::new(x, y_offset, width, height);

            canvas.copy(&texture, None, Some(target))?;
            y_offset += line_height;
        }

        Ok(())
    }
}
