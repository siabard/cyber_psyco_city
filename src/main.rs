use hangul_jaso::*;
use jaso_sdl2::*;
use sdl2::image::*;
use sdl2::render::Texture;
use std::collections::HashMap;

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

    let mut bitmap_fonts: HashMap<Languages, &Texture> = HashMap::new();
    let kor_bitmap_fonts = texture_creator
        .load_texture("assets/bitmap_fonts/hangul-dkby-dinaru-2.png")
        .unwrap();
    let eng_bitmap_fonts = texture_creator
        .load_texture("assets/bitmap_fonts/ascii-plain.png")
        .unwrap();

    bitmap_fonts.insert(Languages::Hangul, &kor_bitmap_fonts);
    bitmap_fonts.insert(Languages::HangulJamo, &kor_bitmap_fonts);
    bitmap_fonts.insert(Languages::Ascii, &eng_bitmap_fonts);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        print_string(
            &mut canvas,
            &bitmap_fonts,
            0,
            0,
            220,
            300,
            &"다람쥐쳇바퀴돌았따 가나다 01234 ABCD()-+_!@#$%^&*()".to_string(),
        );

        print_string(
            &mut canvas,
            &bitmap_fonts,
            0,
            120,
            150,
            90,
            &"동해물과 백두산이".to_string(),
        );
        canvas.present();

        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
