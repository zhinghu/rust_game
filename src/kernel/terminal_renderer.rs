use super::render_base::Renderer;

pub struct Terminal_renderer;

impl Terminal_renderer {
    pub fn new() -> Self {
        Terminal_renderer {}
    }
}

impl Renderer for Terminal_renderer {
    fn render(&self) {
        //
    }
}
