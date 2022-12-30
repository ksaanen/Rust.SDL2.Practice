extern crate sdl2;

use std::arch::x86_64::_CMP_FALSE_OS;

use sdl2::sys::{SDL_CreateWindow, SDL_CreateRenderer, SDL_Init, SDL_INIT_EVERYTHING};

fn main() {
    let _window;
    let _renderer;
    unsafe {
        // Initialize SDL
        if (SDL_Init(SDL_INIT_EVERYTHING) >= 0) {
            _window = SDL_CreateWindow(b"Chapter 1: Setting up SDL".as_ptr().cast(), 0, 0, 800, 600, 0);

            if _window {
                _renderer = SDL_CreateRenderer(_window, 0, 0);
            }
        }
    }
     
}
