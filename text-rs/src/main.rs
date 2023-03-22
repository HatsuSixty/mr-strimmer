use std::env::args;
use std::path::Path;
use std::process::exit;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

static SCREEN_WIDTH: u32 = 621;
static SCREEN_HEIGHT: u32 = 85;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

fn floatext(font_path: &Path, text: &str, image_background_path: &str) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsys
        .window(text, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .borderless()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Load a font
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
        .render(text)
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    if (|| {
        let image_texture;
        if let Ok(t) = texture_creator.load_texture(Path::new(image_background_path)) {
            image_texture = t;
        } else {
            return 1;
        }

        canvas.copy(&image_texture, None, None).unwrap();
        canvas.present();

        return 0;
    })() == 1 {
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();
    }

    let TextureQuery { width, height, .. } = texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 4;
    let target = get_centered_rect(
        width,
        height,
        SCREEN_WIDTH - padding,
        SCREEN_HEIGHT - padding,
    );

    canvas.copy(&texture, None, Some(target))?;
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        eprintln!("ERROR: No input file or text was provided");
        eprintln!(r#"USAGE: {} <INPUT> <TEXT>
  INPUT: The font that will be used for rendering the text
  TEXT: The text that is going to be rendered"#, args[0]);
        exit(1);
    }

    let font = Path::new(args[1].as_str());
    let text = args[2].as_str();
    let bimg;
    if let Some(x) = args.get(3) {
        bimg = x.as_str();
    } else {
        bimg = "/tmp/invalid.png";
    }

    floatext(font, text, bimg).unwrap();
}
