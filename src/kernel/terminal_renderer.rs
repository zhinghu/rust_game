use std::io::{self, Write};

use super::render_base;

pub struct TerminalRenderer {
    canvas: render_base::Canvas,
    old_canvas: render_base::Canvas,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        let canvas = render_base::Canvas::new(
            termsize::get().unwrap().cols as usize,
            termsize::get().unwrap().rows as usize,
            palette::LinSrgb::new(0.0, 0.0, 0.0),
        );

        TerminalRenderer {
            old_canvas: canvas.clone(),
            canvas,
        }
    }
}

impl render_base::Renderer for TerminalRenderer {
    /// 渲染至终端
    fn render(&mut self) {
        self.canvas.size(Some([
            termsize::get().unwrap().cols as usize,
            termsize::get().unwrap().rows as usize,
        ]));

        let mut buffer = io::stdout().lock();
        let canvas_data = self.canvas.data(None);
        buffer.write("\x1b[0;0H".as_bytes()).unwrap();
        canvas_data.write().unwrap().iter_mut().for_each(|d| {
            let color: palette::Srgb<u8> = d.color(None).into_encoding();
            buffer
                .write(
                    format!(
                        "\x1b[48;2;{};{};{}m\u{0020}",
                        color.red, color.green, color.blue
                    )
                    .as_bytes(),
                )
                .unwrap();
        });

        buffer.write("\x1b[0m".as_bytes()).unwrap();
        buffer.flush().unwrap();
        self.old_canvas = self.canvas.clone();
        self.canvas.clear();
        self.canvas.check();
    }

    fn get_canvas(&self) -> &render_base::Canvas {
        &self.canvas
    }

    fn get_mut_canvas(&mut self) -> &mut render_base::Canvas {
        &mut self.canvas
    }
}
