use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::path::Path;
use std::process::exit;
use std::sync::mpsc::channel;
use std::thread::{sleep, spawn};
use std::time::Duration;

use super::image::*;
use super::parser::*;
use super::download::*;

pub fn change_img(image: String) {
    let host = "localhost:6969";
    let ip_lookup = host.to_socket_addrs().unwrap().next().unwrap();
    let mut socket;
    if let Ok(s) = TcpStream::connect_timeout(&ip_lookup, Duration::from_millis(5000)) {
        socket = s;
    } else {
        eprintln!("ERROR: Could not connect to `{}`", host);
        exit(1);
    }

    socket
        .write(format!("chimg \"{}\"", image).as_bytes())
        .unwrap();
    socket.flush().unwrap();
}

pub fn floatimg(image: String) {
    let (tx, rx) = channel();

    spawn(move || {
        let listener = TcpListener::bind("localhost:6969").unwrap();
        println!("[floatimg] started listening to port 6969");

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let bufreader = BufReader::new(&mut stream);
            let http_request: Vec<_> = bufreader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            for line in http_request {
                if let Err(_) = tx.send(line) {
                    eprintln!("ERROR: Error sending data to via `tx`");
                    exit(1);
                }
            }
        }
    });

    println!("[floatimg] waiting 500 milliseconds...");
    sleep(Duration::from_millis(500));

    let sdl_context;
    if let Ok(ctx) = sdl2::init() {
        sdl_context = ctx;
    } else {
        eprintln!("ERROR: Could not initialize SDL2");
        exit(1);
    }

    let video_subsystem;
    if let Ok(vs) = sdl_context.video() {
        video_subsystem = vs;
    } else {
        eprintln!("ERROR: Could not initialize video subsystem");
        exit(1);
    }

    if let Err(_) =
        sdl2::image::init(InitFlag::PNG | InitFlag::JPG | InitFlag::TIF | InitFlag::WEBP)
    {
        eprintln!("ERROR: Could not initialize image subsystem");
        exit(1);
    }

    static INVALID_PNG_PATH: &str = "/tmp/invalid.png";

    let mut image_path = image.clone();

    download_file("https://archive.org/download/png-transparency-demonstration-1/PNG_transparency_demonstration_1.png".to_string(), INVALID_PNG_PATH.to_string());

    let (mut width, mut height) = get_image_dimensions(image.clone());
    if (width == 0 || height == 0) || (width == 0 && height == 0) {
        image_path = INVALID_PNG_PATH.to_string();
        (width, height) = get_image_dimensions(image_path.clone());
    }

    let window;
    if let Ok(w) = video_subsystem
        .window(image.as_str(), width, height)
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

    let texture_creator = canvas.texture_creator();

    let mut event_pump;
    if let Ok(e) = sdl_context.event_pump() {
        event_pump = e;
    } else {
        eprintln!("ERROR: Could create SDL2 event pump");
        exit(1);
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        if let Ok(msg) = rx.recv_timeout(Duration::from_millis(10)) {
            let args = parse_request(msg);
            if args[0] == "chimg".to_string() {
                image_path = args[1].clone();
                println!("[floatimg] changing image to: `{}`", image_path);
            }
        }

        let texture;
        if let Ok(t) = texture_creator.load_texture(Path::new(image_path.as_str())) {
            let window = canvas.window_mut();

            let (w, h) = get_image_dimensions(image_path.clone());
            if window.size() != (w, h) {
                window.set_size(w, h).unwrap();
            }
            texture = t;
        } else {
            if let Ok(t) = texture_creator.load_texture(Path::new(INVALID_PNG_PATH)) {
                let window = canvas.window_mut();

                let (w, h) = get_image_dimensions(INVALID_PNG_PATH.to_string());
                if window.size() != (w, h) {
                    window.set_size(w, h).unwrap();
                }
                texture = t;
            } else {
                eprintln!("ERROR: Could not load texture `{}`", INVALID_PNG_PATH);
                exit(1);
            }
        }

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
