# Mr Strimmer

A Discord stream bot.

## Discord stream bot?

Yes. When you start the bot, it spawns a bunch of what I like to call "widgets". Those widgets can be controlled via bot commands.

You can choose which widgets you want to use by specifiying them in the `config.toml` (See [configuring](#configuring))

In other words, it's a bot designed to make livestreams on discord.

### Configuring

Example config file:
```toml
[discord]
token = "<discord-token>"

[widgets]
#          show widget      |  color of the border around the camera
camera = { enabled = true,     border_color = "0xdeadbeef" }
#          disable widget   |  text font       | text                  | background image
text   = { enabled = false,    font = "default", text = "Hello, World!", background_image = "default" }
#          disable widget   |  image path
image  = { enabled = false,    image_path = "default" }
```

## Quick Start

### Dependencies

This project depends on `Python` (for building), `OpenCV` and `SDL2`, which can be installed via the command:

#### Arch Linux
```console
# pacman -S clang python sdl2 sdl2_image sdl2_ttf
```

### Building and Running

```console
$ ./build.py build run
```
