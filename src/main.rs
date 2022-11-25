use sdl2::TimerSubsystem;

fn main() -> Result<(), String> {
    let context = sdl2::init().unwrap();

    let video_system = context.video().unwrap();
    let _image_context =
        sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG).unwrap();

    let window = video_system
        .window("Cyber Psycho City", 800, 600)
        .build()
        .unwrap();

    // canvas & renderer
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
