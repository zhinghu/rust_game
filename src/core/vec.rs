mod vec {
  pub fn dot(a: Vec<f32>, b: Vec<f32>) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x*y).sum()
  }
  pub fn to_rgb(mut vec2: Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];
    result.resize(4, -1.0);
    vec2.resize(4, -1.0);
    for n in 0..4 {
      result[n] = (vec2[n] + 1.0) * 127.5;
    }

    result
  }
}