use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;



pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub color: Color
}


impl Circle {

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let left: i32 = self.center.x() - self.radius;
        let top: i32 = self.center.y() - self.radius;
        let right: i32 = self.center.x() + self.radius;
        let bottom: i32 = self.center.y() + self.radius;
        
        canvas.set_draw_color(self.color);

        for y in top..=bottom {
            for x in left..=right {
                if self.inCircle(x, y) {
                   canvas.draw_point(Point::new(x, y)); 
                }
            }
        }
    }

    pub fn inCircle(&self, x: i32, y: i32) -> bool {
       return (x - self.center.x()).pow(2) + (y - self.center.y()).pow(2) <= self.radius.pow(2);
    }
}
