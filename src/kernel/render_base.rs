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
    size: [usize; 2],
}
impl Canvas {
    /// 创建一个长为`width`，宽为`height`的画布
    pub fn new(width: usize, height: usize, clear: palette::LinSrgb<f32>) -> Self {
        let mut data = RwLock::new(Vec::new());
        data.write().unwrap().resize(
            width * height,
            CanvasData {
                color: palette::LinSrgb::new(1.0, 1.0, 1.0),
                deepin: -1.0,
            },
        );

        Self {
            clear_color: clear,
            data,
            size: [width, height],
        }
    }

    /// 检查数据有误的地方，并进行纠正
    pub fn check(&mut self) {
        if self.data.read().unwrap().len() != self.size[0] * self.size[1] {
            self.data.write().unwrap().resize(
                self.size[0] * self.size[1],
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
     `size`既可以传入`None`来获取当前大小，也可以`Some([usize; 2])`来修改大小

    示例:
    ```rust
    canvas.size(None); // -> [114, 514]
    canvas.size(Some([191, 9810])) // -> &self
    ```
    */
    pub fn size(&mut self, size: Option<[usize; 2]>) -> &[usize; 2] {
        match size {
            Some(value) => {
                debug!("value: {:?}", value);
                self.data.write().unwrap().resize(
                    value[0] * value[1],
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
                if self.size[0] * self.size[1] != value.capacity() {
                    error!("Different canvas sizes");
                    panic!("Different canvas sizes");
                }

                *self.data.write().unwrap() = value;

                &self.data
            }
            None => &self.data,
        }
    }
}
impl Clone for Canvas {
    fn clone(&self) -> Self {
        let data = RwLock::new(Vec::new());
        data.write().unwrap().resize(
            self.size[0] * self.size[1],
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
