pub mod color;
pub mod console;

mod render;
pub use render::Render;
mod thread_pool;
pub use thread_pool::ThreadPool;
mod camera;
pub use camera::Camera;
mod ray;
pub use ray::Ray;
