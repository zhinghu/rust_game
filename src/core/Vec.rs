pub mod vec {
    pub type Vec2 = [f32; 2];
    pub type Vec3 = [f32; 3];
    pub type Vec4 = [f32; 4];

    pub fn dot(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());
        a.iter().zip(b.iter()).map(|(&v1, &v2)| v1 * v2).sum()
    }

    pub fn add(a: &[f32], b: &[f32]) -> Vec4 {
        assert_eq!(a.len(), b.len());
        a.iter().zip(b.iter()).map(|(&v1, &v2)| v1 + v2).collect::<Vec4>()
    }

    pub fn sub(a: &[f32], b: &[f32]) -> Vec4 {
        assert_eq!(a.len(), b.len());
        a.iter().zip(b.iter()).map(|(&v1, &v2)| v1 - v2).collect::<Vec4>()
    }

    pub fn scalar_mul(a: &[f32], scalar: f32) -> Vec4 {
        a.iter().map(|&v| v * scalar).collect::<Vec4>()
    }

    pub fn magnitude(a: &[f32]) -> f32 {
        a.iter().map(|&v| v * v).sum::<f32>().sqrt()
    }
}