use sdl2::event::Event;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use nokhwa::Camera;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};

use std::env::args;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

static WINDOW_SCALE_FACTOR: f32 = 0.35;

fn create_texture_from_u8_vec(
    texture_creator: &TextureCreator<WindowContext>,
    mut pixels: Vec<u8>,
    width: u32,
    height: u32,
) -> Texture {
    let surface;
    if let Ok(s) = Surface::from_data(
        &mut pixels,
        width,
        height,
        width * 3,
        PixelFormatEnum::RGB24,
    ) {
        surface = s;
    } else {
        eprintln!("ERROR: Could not create surface from pixels");
        exit(1);
    }

    if let Ok(t) = texture_creator.create_texture_from_surface(surface) {
        return t;
    } else {
        eprintln!("ERROR: Could not create texture from surface");
        exit(1);
    }
}

fn main() {
    nokhwa::nokhwa_initialize(|x| {
        println!("[INFO] Nokhwa initialized: {}", x);
    });
    sleep(Duration::from_millis(2000));
    webcam_rs_main();
}

fn webcam_rs_main() {
    let args: Vec<String> = args().collect();

    let mut border_color_hex = "0x01a1d1".to_string();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" => {
                println!(
                    r#"USAGE: {} [OPTIONS] [BORDERCOLOR]
  OPTIONS:
    --help       Prints this help
  BORDERCOLOR:   The color of the border around the webcam"#,
                    args[0]
                );
                exit(0);
            }
            _ => {
                border_color_hex = args[1].clone();
            }
        }
    }

    if border_color_hex.starts_with("0x") {
        // i know, absoletely beautiful
        border_color_hex = border_color_hex.chars().skip(1).collect::<String>();
        border_color_hex = border_color_hex.chars().skip(1).collect::<String>();
    }

    let border_color;
    match u32::from_str_radix(border_color_hex.as_str(), 16) {
        Ok(value) => {
            border_color = value;
        }
        Err(_) => {
            eprintln!("ERROR: Invalid hex string: {}", border_color_hex);
            exit(1);
        }
    }

    let index = CameraIndex::Index(0);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    let mut camera;
    match Camera::new(index, requested) {
        Ok(c) => camera = c,
        Err(e) => {
            eprintln!("ERROR: Could create new `Camera`: {}", e);
            exit(1);
        }
    }

    if let Err(e) = camera.open_stream() {
        eprintln!("ERROR: Could not open stream for `Camera`: {}", e);
        exit(1);
    }

    let (width, height);
    {
        let frame;
        match camera.frame() {
            Ok(f) => frame = f,
            Err(e) => {
                eprintln!("ERROR: Could not get camera frame: {}", e);
                exit(1);
            }
        }

        let decoded = frame.decode_image::<RgbFormat>().unwrap();

        width = decoded.width() as i32;
        height = decoded.height() as i32;
    }

    let sdl_context;
    if let Ok(c) = sdl2::init() {
        sdl_context = c;
    } else {
        eprintln!("ERROR: Could not initialize SDL2");
        exit(1);
    }

    let video_subsystem;
    if let Ok(v) = sdl_context.video() {
        video_subsystem = v;
    } else {
        eprintln!("ERROR: Could not initialize video subsystem");
        exit(1);
    }

    let window;
    if let Ok(w) = video_subsystem
        .window(
            "webcam",
            (width as f32 * WINDOW_SCALE_FACTOR) as u32,
            (height as f32 * WINDOW_SCALE_FACTOR) as u32,
        )
        .position_centered()
        .borderless()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
    {
        window = w;
    } else {
        eprintln!("ERROR: Could not create window");
        exit(1);
    }

    let mut canvas;
    if let Ok(c) = window.into_canvas().build().map_err(|e| e.to_string()) {
        canvas = c;
    } else {
        eprintln!("ERROR: Could not create canvas");
        exit(1);
    }
    canvas
        .set_logical_size(width as u32, height as u32)
        .unwrap();

    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump;
    if let Ok(e) = sdl_context.event_pump() {
        event_pump = e;
    } else {
        eprintln!("ERROR: Could not create SDL event pump");
        exit(1);
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        let pixels: Vec<u8>;
        {
            let frame = camera.frame().unwrap();
            let decoded = frame.decode_image::<RgbFormat>().unwrap();

            pixels = decoded.into_raw();
        }

        let texture =
            create_texture_from_u8_vec(&texture_creator, pixels, width as u32, height as u32);

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();

        let color;
        {
            let red = ((border_color >> 16) & 0xFF) as u8;
            let green = ((border_color >> 8) & 0xFF) as u8;
            let blue = (border_color & 0xFF) as u8;
            color = Color::RGB(red, green, blue);
        }
        canvas.set_draw_color(color);

        for x in 0..width {
            for y in 0..height {
                if (y > height - 50) || (y < 50) || (x > width - 50) || (x < 50) {
                    canvas.draw_point(Point::new(x, y)).unwrap();
                }
            }
        }
        canvas.set_draw_color(Color::RGB(255, 0, 0));

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
