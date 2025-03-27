use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::model::game::BoardPiece;
use crate::utils::shapes::Circle;

pub struct Renderer {
    pub scrn_area: Rect,
    pub clear_color: Color,
}

impl Renderer {

    pub fn render(&self, canvas: &mut Canvas<Window>, board: &[[BoardPiece; 5]; 5]) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.scrn_area).ok().unwrap_or_default();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        
        // Draw the horizontal and vertical lines of the board
        self.draw_lines(canvas);

        // Draw the pieces 
        self.draw_pieces(canvas, board);
    }

    fn draw_lines(&self, canvas: &mut Canvas<Window>) {

        let cell_width: i32 = self.scrn_area.h / 5;
        let cell_height: i32 = self.scrn_area.h / 5;
        // Board width: (4 spaces between 5 lines)x(Cell width)
        // Centering of board involves halving the board width which is a factor of 4.
        // So simplify to 2xCell width (and similarly for height)
        let left: i32 = self.scrn_area.w/2 - 2*cell_width; // x-position of left side of centered board
        let top: i32 = self.scrn_area.h/2 - 2*cell_height; // y-position of top side of centered board
        let right: i32 = self.scrn_area.w/2 + 2*cell_width;
        let bottom: i32 = self.scrn_area.h/2 + 2*cell_height;

        for i in 0..=4 {
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
        // If you choose to draw all the lines in the future: Do not draw the lines directly
        // Instead loop through 3 iterations and draw the rectangles traced by all the diagonal
        // lines
        // Draw the down and rightwards diagonal lines of the board
        canvas.draw_line(
            Point::new(left, top + cell_height*2),
            Point::new(left + cell_width*2, bottom)
        );
        canvas.draw_line(
            Point::new(left, top),
            Point::new(right, bottom)
        );
        canvas.draw_line(
            Point::new(left + cell_width*2, top),
            Point::new(right, top + cell_height*2)
        );
        // Draw the up and rightwards diagonal lines of the board
        canvas.draw_line(
            Point::new(left, top + cell_height*2),
            Point::new(left + cell_width*2, top)
        );
        canvas.draw_line(
            Point::new(left, bottom),
            Point::new(right, top)
        );
        canvas.draw_line(
            Point::new(left + cell_width*2, bottom),
            Point::new(right, top + cell_height*2)
        );

    }

    fn draw_pieces(&self, canvas: &mut Canvas<Window>, board: &[[BoardPiece; 5]; 5]) {

        let width: i32 = self.scrn_area.h / 5;
        let height: i32 = self.scrn_area.h / 5;
        // Board width: (4 spaces between 5 lines)x(Cell width)
        // Centering of board involves halving the board width which is a factor of 4.
        // So simplify to 2xCell width (and similarly for height)
        let left: i32 = self.scrn_area.w/2 - 2*width; // x-position of left side of centered board
        let top: i32 = self.scrn_area.h/2 - 2*height; // y-position of top side of centered board


        for i in 0i32..=4 {
            let row: usize = i.try_into().unwrap();
            for j in 0i32..=4 {
                let col: usize = j.try_into().unwrap();
                if board[row][col] != BoardPiece::None {
                    let mut c = Color::RGB(25, 30, 30);
                    if board[row][col] == BoardPiece::Red {
                        c = Color::RGB(140, 35, 25);
                    }
                    //let rect = Rect::new(left - width/4 + width*j, top - height/4 + height*i, 
                    //        (width/2).try_into().unwrap(), (height/2).try_into().unwrap());
                    let circle: Circle = Circle {center: Point::new(left + width*j, top + height*i), radius: width/3, color: c};
                    circle.draw(canvas);
                }
            }
        }
    }
}




































