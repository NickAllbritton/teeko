mod view;
mod model;
mod utils;


fn main() -> Result<(), String> {
  
    let scrn_width: u32 = 1600;
    let scrn_height: u32= 1200;

    let sdl_context = sdl2::init()?;
    let vid_subsystem = sdl_context.video()?;
    let wnd = vid_subsystem.window("Teeko", scrn_width, scrn_height)
        .build()
        .unwrap();
    
    let mut canvas = wnd.into_canvas()
        .build()
        .unwrap();

    let board_view: view::board_view::Renderer 
        = view::board_view::Renderer::new((scrn_width/2 - 2*scrn_height/5).try_into().unwrap(), (scrn_height/10).try_into().unwrap(), scrn_height/5 * 4, scrn_height/5 * 4);

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
                    let x_from_left: i32 = x - board_view.scrn_area.x;
                    let y_from_top: i32 = y - board_view.scrn_area.y;
                    let click_radius: i32 = board_view.scrn_area.w / 12; // The radius of the click
                    let cell_side: i32 = board_view.scrn_area.w / 4;
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
        
        board_view.render(&mut canvas, &game_state.board);

        canvas.present();
    }

    Ok(())
}
