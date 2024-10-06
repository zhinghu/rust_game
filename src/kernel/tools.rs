pub trait GlamVec {}
impl GlamVec for glm::Vec2 {}
impl GlamVec for glm::Vec3 {}
impl GlamVec for glm::Vec3A {}
impl GlamVec for glm::Vec4 {}

pub trait GlamMat {}
impl GlamMat for glm::Mat2 {}
impl GlamMat for glm::Mat3 {}
impl GlamMat for glm::Mat3A {}
impl GlamMat for glm::Mat4 {}

pub trait Transform<Output: GlamMat> {
    #[must_use]
    fn process(&self) -> Output;
}
