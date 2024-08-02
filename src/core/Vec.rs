pub mod vec {
    pub type vec2 = [f32; 2]
    pub type vec3 = [f32; 3]
    pub type vec4 = [f32; 4]

    pub fn dot(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());
        a.iter().zip(b.iter()).map(|(&v1, &v2)| v1 * v2).sum()
    }
}