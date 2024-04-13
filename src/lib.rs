mod rssdl;

pub use self::rssdl::*;
pub use sdl2_sys as raw_sdl;





pub fn init() -> anyhow::Result<renderer::SDLContext> {
    renderer::SDLContext::init()
}