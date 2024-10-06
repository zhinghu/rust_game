use log::{debug, error};
use palette::{FromColor, Mix, MixAssign};
use std::sync::RwLock;

/// 画布`Canvas`里的数据存储
#[derive(Clone, Debug)]
pub struct CanvasData {
    color: palette::LinSrgb<f32>,
    deepin: f32,
}
impl CanvasData {
    /// 设置颜色\/获取颜色
    pub fn color(&mut self, value: Option<palette::LinSrgb<f32>>) -> &palette::LinSrgb<f32> {
        match value {
            Some(value) => {
                self.color = value;
                &self.color
            }
            None => &self.color,
        }
    }
}

/**
`Canvas`自定义画布

`x`和`y`的起点都是从左上角开始
*/
#[derive(Debug)]
pub struct Canvas {
    clear_color: palette::LinSrgb<f32>,
    data: RwLock<Vec<CanvasData>>,
    size: glm::UVec2,
}
impl Canvas {
    /// 创建一个长为`width`，宽为`height`的画布
    pub fn new(width: u32, height: u32, clear: palette::LinSrgb<f32>) -> Self {
        let data = RwLock::new(Vec::new());
        data.write().unwrap().resize(
            (width * height) as usize,
            CanvasData {
                color: palette::LinSrgb::new(1.0, 1.0, 1.0),
                deepin: -1.0,
            },
        );

        Self {
            clear_color: clear,
            data,
            size: glm::uvec2(width, height),
        }
    }

    /// 检查数据有误的地方，并进行纠正
    pub fn check(&mut self) {
        if self.data.read().unwrap().len() as u32 != self.size.x * self.size.y {
            self.data.write().unwrap().resize(
                (self.size.x * self.size.y) as usize,
                CanvasData {
                    color: self.clear_color,
                    deepin: 0.0,
                },
            );
        }
    }

    /// 字面意思，清空画布
    pub fn clear(&mut self) {
        for d in self.data.write().unwrap().iter_mut() {
            d.color = self.clear_color;
        }
    }

    /**
    `fill_color`用`color`颜色填满整个画布
    */
    pub fn fill_color(&mut self, color: palette::LinSrgba<f32>) -> &Self {
        self.check();
        for d in self.data.write().unwrap().iter_mut() {
            d.color
                .mix_assign(palette::LinSrgb::from_color(color), color.alpha);
        }

        self
    }

    /**
     `size`既可以传入`None`来获取当前大小，也可以`Some(glm::UVec2)`来修改大小

    示例:
    ```rust
    canvas.size(None); // -> UVec2{114, 514}
    canvas.size(Some(glm::uvec2(191, 9810))) // -> &self
    ```
    */
    pub fn size(&mut self, size: Option<glm::UVec2>) -> &glm::UVec2 {
        match size {
            Some(value) => {
                self.data.write().unwrap().resize(
                    (value.x * value.y) as usize,
                    CanvasData {
                        color: self.clear_color,
                        deepin: 0.0,
                    },
                );
                self.size = value;

                &self.size
            }

            None => &self.size,
        }
    }

    /// 获取数据和存储(覆盖)数据
    /// 提供的数据`data`必须和原本的数据`self.data`长度相同
    /// 即: `data.capacity()`获取的数据要等于自身的长度`size`
    pub fn data(&mut self, data: Option<Vec<CanvasData>>) -> &RwLock<Vec<CanvasData>> {
        self.check();
        match data {
            Some(value) => {
                if self.size.x * self.size.y != value.capacity() as u32 {
                    error!("Different canvas sizes");
                    panic!("Different canvas sizes");
                }

                *self.data.write().unwrap() = value;

                &self.data
            }
            None => &self.data,
        }
    }

    /// 在一个点上放置一个颜色
    pub fn put_color(&mut self, pos: glm::Vec2, color: palette::LinSrgba<f32>) {
        self.put_color_origin(pos, color);
        if pos.x - pos.x.floor() > 0.5 {
            let mut cl = color.clone();
            cl.alpha = cl.alpha * (0.5 - 1.0 + pos.x - pos.x.floor()) * 2.0;
            self.put_color_origin(glm::vec2(pos.x.floor() + 1.0, pos.y), cl);
        }
        if pos.x - pos.x.floor() < 0.5 {
            let mut cl = color.clone();
            cl.alpha = cl.alpha * (0.5 - pos.x + pos.x.floor()) * 2.0;
            self.put_color_origin(glm::vec2(pos.x.floor() - 1.0, pos.y), cl);
        }
        if pos.y - pos.y.floor() > 0.5 {
            let mut cl = color.clone();
            cl.alpha = cl.alpha * (0.5 - 1.0 + pos.y - pos.y.floor()) * 2.0;
            self.put_color_origin(glm::vec2(pos.x, pos.y.floor() + 1.0), cl);
        }
        if pos.y - pos.y.floor() < 0.5 {
            let mut cl = color.clone();
            cl.alpha = cl.alpha * (0.5 - pos.y + pos.y.floor()) * 2.0;
            self.put_color_origin(glm::vec2(pos.x, pos.y.floor() - 1.0), cl);
        }

        if pos.x - pos.x.floor() > 0.5 && pos.y - pos.y.floor() > 0.5 {
            let mut cl = color.clone();
            cl.alpha = cl.alpha
                * ((0.5 - 1.0 + pos.x - pos.x.floor()) + (0.5 - 1.0 + pos.y - pos.y.floor()));
            self.put_color_origin(glm::vec2(pos.x.floor() + 1.0, pos.y.floor() + 1.0), cl);
        }
        if pos.x - pos.x.floor() < 0.5 && pos.y - pos.y.floor() < 0.5 {
            let mut cl = color.clone();
            cl.alpha = cl.alpha * ((0.5 - pos.x + pos.x.floor()) + (0.5 - pos.y + pos.y.floor()));
            self.put_color_origin(glm::vec2(pos.x.floor() - 1.0, pos.y.floor() - 1.0), cl);
        }
        if pos.y - pos.y.floor() > 0.5 && pos.x - pos.x.floor() < 0.5 {
            let mut cl = color.clone();
            cl.alpha =
                cl.alpha * ((0.5 - 1.0 + pos.y - pos.y.floor()) + (0.5 - pos.x + pos.x.floor()));
            self.put_color_origin(glm::vec2(pos.x.floor() - 1.0, pos.y.floor() + 1.0), cl);
        }
        if pos.y - pos.y.floor() < 0.5 && pos.x - pos.x.floor() > 0.5 {
            let mut cl = color.clone();
            cl.alpha =
                cl.alpha * ((0.5 - pos.y + pos.y.floor()) + (0.5 - 1.0 + pos.x - pos.x.floor()));
            self.put_color_origin(glm::vec2(pos.x.floor() + 1.0, pos.y.floor() - 1.0), cl);
        }
    }
    fn put_color_origin(&mut self, pos: glm::Vec2, color: palette::LinSrgba<f32>) {
        self.check();
        if pos.x > self.size.x as f32 || pos.y > self.size.y as f32 || pos.x < 0.0 || pos.y < 0.0 {
            return;
        }
        let mut d = self.data.write().unwrap();
        let data = d
            .get_mut((pos.y as u32 * self.size.x + pos.x as u32) as usize)
            .unwrap();

        data.color
            .mix_assign(palette::LinSrgb::from_color(color), color.alpha);
    }
}
impl Clone for Canvas {
    fn clone(&self) -> Self {
        let data = RwLock::new(Vec::new());
        data.write().unwrap().resize(
            (self.size.x * self.size.y) as usize,
            CanvasData {
                color: palette::LinSrgb::new(1.0, 1.0, 1.0),
                deepin: -1.0,
            },
        );
        Self {
            clear_color: self.clear_color.clone(),
            data,
            size: self.size.clone(),
        }
    }
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

pub trait Renderer {
    fn render(&mut self);
    fn get_canvas(&self) -> &Canvas;
    fn get_mut_canvas(&mut self) -> &mut Canvas;
}
