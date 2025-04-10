use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::image::LoadTexture;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use crate::model::game::BoardPiece;
use crate::utils::shapes::Circle;

pub struct Renderer<'a> {
    pub board_area: Rect,
    pub board_color: Color,
    board_wood_grain: Texture<'a>,
    pieces_texture: Texture<'a>
}

impl<'a> Renderer<'a> {

    pub fn new(posx: i32, posy: i32, width: u32, height: u32, tex_creator: &'a TextureCreator<WindowContext>) -> Result<Self, sdl2::Error> {

        //let grain_texture = tex_creator.load_texture("resources/wood_texture1.png");
        //let grain_texture = tex_creator.load_texture("resources/wood_texture2.png");
        // TODO: Find a way to use relative paths so that other people can install this game onto
        // their system
        let grain_texture = tex_creator.load_texture("/home/nick/software/teeko/resources/wood_panel_texture.png");

        let piece_texture = tex_creator.load_texture("/home/nick/software/teeko/resources/checkers_piece_texture.png");
        Ok(Self {
            board_area: Rect::new(posx, posy, width, height),
            board_color: Color::RGB(158, 103, 68),
            board_wood_grain: grain_texture.expect("Something went wrong with loading the texture!"),
            pieces_texture: piece_texture.expect("Something went wrong with loading the piece texture!")
        })
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, board: &[[BoardPiece; 5]; 5]) {
        canvas.set_draw_color(self.board_color); // Color of the board
        canvas.fill_rect(self.board_area).ok().unwrap_or_default();

        self.draw_woodgrain_texture(canvas);

        // Color of lines to draw
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        // Draw the horizontal and vertical lines of the board
        self.draw_lines(canvas);

        // Draw the pieces 
        self.draw_pieces(canvas, board);
    }

    fn draw_woodgrain_texture(&self, canvas: &mut Canvas<Window>) {
        let image = &self.board_wood_grain;
        let image_attr = image.query();

//        let src_rect = Rect::new(0, 0, image_attr.width, image_attr.height);
        // Take a square of the texture
        let src_rect = Rect::new(0, 0, image_attr.width, image_attr.width);
        // Copy it to the position and size of the board
        let dest_rect = Rect::new(self.board_area.x(), self.board_area.y(), 
                        self.board_area.width(), self.board_area.height());
        canvas.copy(image, src_rect, dest_rect).ok().unwrap();
    }

    fn draw_lines(&self, canvas: &mut Canvas<Window>) {

        let cell_width: i32 = self.board_area.w / 4;
        let cell_height: i32 = self.board_area.h / 4;
        let left: i32 = self.board_area.x; // x-position of left side of centered board
        let top: i32 = self.board_area.y; // y-position of top side of centered board
        let right: i32 = left + 4*cell_width;
        let bottom: i32 = top + 4*cell_height;

        for i in 0..=4 {
            // Draw the horizontal lines of the board 
            let _ = canvas.draw_line(
                Point::new(left, top + i*cell_height),
                Point::new(right, top + i*cell_height)
            ).ok().unwrap_or_default();
            // Draw the horizontal lines of the board
            let _ = canvas.draw_line(
                Point::new(left + i*cell_width, top),
                Point::new(left + i*cell_width, bottom)
            ).ok().unwrap_or_default();
            

        }
        // To some extent this is stylistic. I am drawing only 6 diagonal lines out of 14
        // If you choose to draw all the lines in the future: Do not draw the lines directly
        // Instead loop through 3 iterations and draw the rectangles traced by all the diagonal
        // lines
        // Draw the down and rightwards diagonal lines of the board
        let _ = canvas.draw_line(
            Point::new(left, top + cell_height*2),
            Point::new(left + cell_width*2, bottom)
        );
        let _ = canvas.draw_line(
            Point::new(left, top),
            Point::new(right, bottom)
        );
        let _ = canvas.draw_line(
            Point::new(left + cell_width*2, top),
            Point::new(right, top + cell_height*2)
        );
        // Draw the up and rightwards diagonal lines of the board
        let _ = canvas.draw_line(
            Point::new(left, top + cell_height*2),
            Point::new(left + cell_width*2, top)
        );
        let _ = canvas.draw_line(
            Point::new(left, bottom),
            Point::new(right, top)
        );
        let _ = canvas.draw_line(
            Point::new(left + cell_width*2, bottom),
            Point::new(right, top + cell_height*2)
        );

    }

    fn draw_pieces(&self, canvas: &mut Canvas<Window>, board: &[[BoardPiece; 5]; 5]) {
        // Dimensions of a cell
        let width: i32 = self.board_area.h / 4;
        let height: i32 = self.board_area.h / 4;
        // Board width: (4 spaces between 5 lines)x(Cell width)
        // Centering of board involves halving the board width which is a factor of 4.
        // So simplify to 2xCell width (and similarly for height)
        let left: i32 = self.board_area.x; // x-position of left side of centered board
        let top: i32 = self.board_area.y; // y-position of top side of centered board

        let image = &self.pieces_texture;
        let image_attr = image.query();



        let src_rect = Rect::new(0, 0, image_attr.width, image_attr.height);
        let mut dest_rect = Rect::new(0, 0, 
            (2*width/3).try_into().unwrap(), 
            (2*width/3).try_into().unwrap()); // Create a destination rect at position 0 with same width as the pieces' diameter

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
                    dest_rect.set_x(left + width*j - dest_rect.w/2);
                    dest_rect.set_y(top + height*i - dest_rect.h/2);
                    canvas.copy(image, src_rect, dest_rect).ok().unwrap();
                }
            }
        }
    }
}


