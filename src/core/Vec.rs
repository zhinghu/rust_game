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
        let mut c: &mut [f32] = &mut a;
        for n in 0..a.len() {
            c[n] = a[n] + b[n];
        }
        c
    }
}
