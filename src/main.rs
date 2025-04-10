mod view;
mod model;
mod utils;

use sdl2::image::LoadTexture;

fn main() -> Result<(), String> {
  
    let scrn_width: u32 = 1800;
    let scrn_height: u32= 1350;

    let sdl_context = sdl2::init()?;
    let vid_subsystem = sdl_context.video()?;
    let wnd = vid_subsystem.window("Teeko", scrn_width, scrn_height)
        .build()
        .unwrap();
    
    let mut canvas = wnd.into_canvas()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();

    let board_view: view::board_view::Renderer 
        = view::board_view::Renderer::new((scrn_width/2 - 2*scrn_height/5).try_into().unwrap(), 
            (scrn_height/10).try_into().unwrap(), 
            scrn_height/5 * 4, scrn_height/5 * 4,
            &texture_creator).expect("Failed to create the renderer");

    // TODO: Use relative paths
    let background_texture = texture_creator.load_texture("/home/nick/software/teeko/resources/timber_planks.png").expect("Loading background texture");
    let image_attr = background_texture.query();
    // Cut a source rect rom the texture
    let src_rect = sdl2::rect::Rect::new((image_attr.width - (scrn_width/scrn_height*image_attr.height)).try_into().unwrap(), 0, 
        (scrn_width/scrn_height*image_attr.height).try_into().unwrap(), (image_attr.height).try_into().unwrap());
    let dest_rect = sdl2::rect::Rect::new(0, 0, scrn_width, scrn_height);

    let mut game_state: model::game::GameState = model::game::GameState::new();
    let mut running: bool = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
   
    //game_state.jumbl_board();

    while running {

        for event in event_queue.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    running = false;
                }
                sdl2::event::Event::MouseButtonDown {x, y, ..} => {
                    let x_from_left: i32 = x - board_view.board_area.x;
                    let y_from_top: i32 = y - board_view.board_area.y;
                    let click_radius: i32 = board_view.board_area.w / 12; // The radius of the click
                    let cell_side: i32 = board_view.board_area.w / 4;
                    let x_shifted: i32 = x_from_left + click_radius;
                    let y_shifted: i32 = y_from_top + click_radius;
                    let col: usize = (x_shifted/cell_side).try_into().unwrap();
                    let row: usize = (y_shifted/cell_side).try_into().unwrap();

                    game_state.handle_click(row, col);
                }
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    if keycode.unwrap() == sdl2::keyboard::Keycode::U {
                        game_state.undo_action();
                    }
                    else if keycode.unwrap() == sdl2::keyboard::Keycode::R {
                        game_state.redo_action();
                    }
                }

                _ => {}
            }
        }
        

        // Cool blue background color. Clear screen with this color set to draw
        canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 120, 120));
        canvas.clear(); // Paint the background color

        // THE FOLLOWING LINE WILL CRASH THE PROGRAM IF WINDOW IS RESIZED TOO MUCH
        // USERS BEWARE
        // Copy background planks texture into the destination rect: the screen area
        canvas.copy(&background_texture, src_rect, dest_rect)?;       
        board_view.render(&mut canvas, &game_state.board);

        canvas.present();
    }

    Ok(())
}
