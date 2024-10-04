use std::io::{self, Write};

use super::render_base;

pub struct Terminal_renderer {
    canvas: render_base::Canvas,
}

impl Terminal_renderer {
    pub fn new() -> Self {
        let canvas = render_base::Canvas::new(
            termsize::get().unwrap().cols as usize,
            termsize::get().unwrap().rows as usize,
        );

        Terminal_renderer { canvas }
    }
}

impl render_base::Renderer for Terminal_renderer {
    /// 渲染至终端
    fn render(&mut self) {
        self.canvas.size(Some([
            termsize::get().unwrap().cols as usize,
            termsize::get().unwrap().rows as usize,
        ]));

        let mut buffer = io::stdout().lock();
        let mut canvas_data = self.canvas.data(None);
        buffer.write("\x1b[0;0H".as_bytes()).unwrap();
        canvas_data.write().unwrap().iter_mut().for_each(|d| {
            let color =
                (*d.color(None) + glm::vec3(1.0, 1.0, 1.0)) * glm::vec3(127.5, 127.5, 127.5);
            buffer
                .write(
                    format!(
                        "\x1b[48;2;{};{};{}m\u{0020}",
                        color.x as u8, color.y as u8, color.z as u8
                    )
                    .as_bytes(),
                )
                .unwrap();
        });

        buffer.write("\x1b[0m".as_bytes()).unwrap();
        buffer.flush().unwrap();
    }

    fn get_canvas(&self) -> &render_base::Canvas {
        &self.canvas
    }

    fn get_mut_canvas(&mut self) -> &mut render_base::Canvas {
        &mut self.canvas
    }
}
