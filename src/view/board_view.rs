use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;


pub struct Renderer {
    pub scrn_area: Rect,
    pub clear_color: Color,
}

impl Renderer {

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.scrn_area).ok().unwrap_or_default();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let cell_width: i32 = self.scrn_area.h / 5;
        let cell_height: i32 = self.scrn_area.h / 5;
        // Board width: (4 spaces between 5 lines)x(Cell width)
        // Centering of board involves halving the board width which is a factor of 4.
        // So simplify to 2xCell width (and similarly for height)
        let left: i32 = self.scrn_area.w/2 - 2*cell_width; // x-position of left side of centered board
        let top: i32 = self.scrn_area.h/2 - 2*cell_height; // y-position of top side of centered board
        // Then subtracting left from screen width is algebraically equivalent to the right side. 

        for i in 0..5 {
            // Draw the horizontal lines of the board 
            canvas.draw_line(
                Point::new(left, top + i*cell_height),
                Point::new(self.scrn_area.w - left, top + i*cell_height)
            ).ok().unwrap_or_default();
            // Draw the horizontal lines of the board
            canvas.draw_line(
                Point::new(left + i*cell_width, top),
                Point::new(left + i*cell_width, self.scrn_area.h - top)
            ).ok().unwrap_or_default();
            

        }
        // To some extent this is stylistic. I am drawing only 6 diagonal lines out of 14
        // Draw the down and rightwards diagonal lines of the board
        canvas.draw_line(
            Point::new(left, top + cell_height*2),
            Point::new(left + cell_width*2, self.scrn_area.h - top)
        );
        canvas.draw_line(
            Point::new(left, top),
            Point::new(self.scrn_area.w - left, self.scrn_area.h - top)
        );
        canvas.draw_line(
            Point::new(left + cell_width*2, top),
            Point::new(self.scrn_area.w - left, top + cell_height*2)
        );
        // Draw the up and rightwards diagonal lines of the board
        canvas.draw_line(
            Point::new(left, top + cell_height*2),
            Point::new(left + cell_width*2, top)
        );
        canvas.draw_line(
            Point::new(left, self.scrn_area.h - top),
            Point::new(self.scrn_area.w - left, top)
        );
        canvas.draw_line(
            Point::new(left + cell_width*2, self.scrn_area.h - top),
            Point::new(self.scrn_area.w - left, top + cell_height*2)
        );
    }
}
