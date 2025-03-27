mod view;
mod model;
mod utils;


fn main() -> Result<(), String> {
  
    let scrn_width: u32 = 1600;
    let scrn_height: u32= 1200;

    let sdl_context = sdl2::init()?;
    let vid_subsystem = sdl_context.video()?;
    let wnd = vid_subsystem.window("Rust!", scrn_width, scrn_height)
        .build()
        .unwrap();
    
    let mut canvas = wnd.into_canvas()
        .build()
        .unwrap();

    let board_view: view::board_view::Renderer = view::board_view::Renderer {
        scrn_area: sdl2::rect::Rect::new(0, 0, scrn_width, scrn_height),
        clear_color: sdl2::pixels::Color::RGB(158, 103, 68)
    };

    let mut game_state: model::game::GameState = model::game::GameState{ board: model::game::make_blank_board() };
    let mut running: bool = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
   
    game_state.jumbl_board();

    while running {

        for event in event_queue.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    running = false;
                }
                _ => {}
            }
        }
        
        board_view.render(&mut canvas, &game_state.board);

        canvas.present();
    }

    Ok(())
}
