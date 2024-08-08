pub mod vec {
    pub type Vec2 = [f32; 2];
    pub type Vec3 = [f32; 3];
    pub type Vec4 = [f32; 4];

    pub fn dot(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());
        a.iter().zip(b.iter()).map(|(&v1, &v2)| v1 * v2).sum()
    }

    pub fn add<'a>(a: &'a[f32], b: &'a[f32]) -> &'a[f32] {
        assert_eq!(a.len(), b.len());
        //
    }

    // 向量的模长
    pub fn magnitude(a: &[f32]) -> f32 {
        a.iter().map(|&v| v * v).sum::<f32>().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::vec;

    #[test]
    fn test_dot() {
        let a = &[1.0, -1.0];
        let b = &[1.0, 1.0];
        assert_eq!(vec::dot(a, b), 0.0);
    }

    #[test]
    fn test_add() {
        let a = &[1.0, -1.0];
        let b = &[1.0, 1.0];
        assert_eq!(vec::add(a, b), [2.0, 0.0]);
    }
}