pub mod vec {
    pub type Vec2 = [f32; 2];
    pub type Vec3 = [f32; 3];
    pub type Vec4 = [f32; 4];

    pub fn dot(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());
        a.iter().zip(b.iter()).map(|(&v1, &v2)| v1 * v2).sum()
    }

    // 向量加法
    pub fn add(a: &[f32], b: &[f32]) -> Vec4 {
        assert_eq!(a.len(), b.len());
        let mut result = [0.0; 4];
        for i in 0..a.len() {
            result[i] = a[i] + b[i];
        }
        result
    }

    // 向量减法
    pub fn sub(a: &[f32], b: &[f32]) -> Vec4 {
        assert_eq!(a.len(), b.len());
        let mut result = [0.0; 4];
        for i in 0..a.len() {
            result[i] = a[i] - b[i];
        }
        result
    }

    // 标量乘法
    pub fn scalar_mul(a: &[f32], scalar: f32) -> Vec4 {
        let mut result = [0.0; 4];
        for i in 0..a.len() {
            result[i] = a[i] * scalar;
        }
        result
    }

    // 向量的模长
    pub fn magnitude(a: &[f32]) -> f32 {
        a.iter().map(|&v| v * v).sum::<f32>().sqrt()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert_eq!(dot(&a, &b), 32.0);
    }

    #[test]
    fn test_addition() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert_eq!(add(&a, &b), [5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_subtraction() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert_eq!(sub(&a, &b), [-3.0, -3.0, -3.0]);
    }

    #[test]
    fn test_scalar_multiplication() {
        let a = [1.0, 2.0, 3.0];
        let scalar = 2.0;
        assert_eq!(scalar_mul(&a, scalar), [2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_magnitude() {
        let a = [1.0, 2.0, 3.0];
        assert_eq!(magnitude(&a), (14.0 as f32).sqrt());
    }
}