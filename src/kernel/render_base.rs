use super::tools;
use log::{debug, error};
use std::sync::RwLock;

/// 画布`Canvas`里的数据存储
#[derive(Clone, Debug)]
pub struct Data {
    color: glm::Vec3,
    deepin: f32,
}
impl Data {
    /// 设置颜色\/获取颜色
    pub fn color(&mut self, value: Option<glm::Vec3>) -> &glm::Vec3 {
        match value {
            Some(value) => {
                self.color = glm::normalize(value);
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
    data: RwLock<Vec<Data>>,
    size: [usize; 2],
}
impl Canvas {
    /// 创建一个长为`width`，宽为`height`的画布
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = RwLock::new(Vec::new());
        data.write().unwrap().resize(
            width * height,
            Data {
                color: glm::vec3(-1.0, -1.0, -1.0),
                deepin: 0.0,
            },
        );

        Self {
            data,
            size: [width, height],
        }
    }

    /**
    `fill_color`用`color`颜色填满整个画布
    */
    pub fn fill_color(&mut self, color: glm::Vec4) -> &Self {
        self.data.write().unwrap().iter_mut().for_each(|d| {
            //
            debug!("{:?}", d.color);
        });

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
                    Data {
                        color: glm::vec3(-1.0, -1.0, -1.0),
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
    pub fn data(&mut self, data: Option<Vec<Data>>) -> &RwLock<Vec<Data>> {
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

pub trait Renderer {
    fn render(&mut self);
    fn get_canvas(&self) -> &Canvas;
    fn get_mut_canvas(&mut self) -> &mut Canvas;
}
