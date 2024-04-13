use commonsdl as sdl;
use sdl::{core::{IVec2, Point, Rect}, events};


fn main() -> anyhow::Result<()>{
    let sdlcontext = sdl::init()?;

    let mut window = sdlcontext.window_builder("SDL2\0", 640, 480).position_centered().accelerated().build()?;


    
    let timer = sdl::timer::TimerSubsystem::new()?;
    
    let surface = sdl::surface::Surface::load_bmp(std::path::Path::new("assets/characters.bmp\0"))?;
    let texture = window.renderer.create_texture_from_surface(&surface)?;



    let frames_per_anim = 4;
    let sprite_tile_size = (32, 32);

    // Baby - walk animation
    let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_0.center_on(Point::new(-64, 120));

    // King - walk animation
    let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_1.center_on(Point::new(0, 240));

    // Soldier - walk animation
    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_2.center_on(Point::new(440, 360));


    let mut is_running = true;
    while is_running {
        while let Some(e) = events::poll_event() {
            match events::type_(e) {
                events::EventType::SDL_QUIT => {
                    is_running = false;
                }

                _ => {}
            }
        }


        let ticks = timer.ticks() as i32;
        // set the current frame for time
        source_rect_0.raw.x = 32 * ((ticks / 100) % frames_per_anim);
        dest_rect_0.raw.x = 1 * ((ticks / 14) % 768) - 128;

        source_rect_1.raw.x = 32 * ((ticks / 100) % frames_per_anim);
        dest_rect_1.raw.x = (1 * ((ticks / 12) % 768) - 672) * -1;

        source_rect_2.raw.x = 32 * ((ticks / 100) % frames_per_anim);
        dest_rect_2.raw.x = 1 * ((ticks / 10) % 768) - 128;

        window.renderer.clear();
        // copy the frame to the window.renderer
        window.renderer.copy_ex_miss_center(
            &texture,
            source_rect_0,
            dest_rect_0,
            0.0,
            IVec2::default(),
            false,
            false,
        );
        window.renderer.copy_ex_miss_center(
            &texture,
            source_rect_1,
            dest_rect_1,
            0.0,
            IVec2::default(),
            true,
            false,
        );
        window.renderer.copy_ex_miss_center(
            &texture,
            source_rect_2,
            dest_rect_2,
            0.0,
            IVec2::default(),
            false,
            false,
        );
        window.renderer.present();

        std::thread::sleep(std::time::Duration::from_millis(100));

    }



    Ok(())
}