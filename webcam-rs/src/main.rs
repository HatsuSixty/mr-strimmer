use sdl2::event::Event;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use opencv::{prelude::*, videoio};

static WINDOW_SCALE_FACTOR: f32 = 0.35;

fn mat_to_vec_u8(mat: &Mat) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    let bytes = mat.data_bytes().unwrap();

    for pixel in bytes {
        vec.push(*pixel);
    }

    vec
}

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
        PixelFormatEnum::BGR24,
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
    let mut cam;
    if let Ok(c) = videoio::VideoCapture::new(0, videoio::CAP_ANY) {
        cam = c;
    } else {
        eprintln!("ERROR: Could not create `cam: VideoCapture`");
        exit(1);
    }

    let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
    if !opened {
        eprintln!("ERROR: Could not open default camera");
        exit(1);
    }

    let (width, height);
    {
        let mut frame = Mat::default();
        if let Err(_) = cam.read(&mut frame) {
            eprintln!("ERROR: Could not get frame from camera");
            exit(1);
        }

        let dimensions = frame.size().unwrap();

        width = dimensions.width;
        height = dimensions.height;
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
            let mut frame = Mat::default();
            if let Err(_) = cam.read(&mut frame) {
                eprintln!("ERROR: Could not get frame from camera");
                exit(1);
            }

            pixels = mat_to_vec_u8(&frame);
        }

        let texture =
            create_texture_from_u8_vec(&texture_creator, pixels, width as u32, height as u32);

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
