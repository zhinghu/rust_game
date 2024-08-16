use ndarray::*;

pub fn to_rgba(
    mut vec4: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>,
) -> ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>> {
    // 防止传入一个长度为4的向量
    assert_eq!(vec4.len(), 4);
    // 将范围设置到-1.0 ~ 1.0
    vec4 = vec4.map(|&x| x.clamp(-1.0, 1.0));

    let ones = arr1(&[1., 1., 1., 1.]);
    let rgba = arr1(&[127.5, 127.5, 127.5, 127.5]);
    &rgba * (&vec4 + &ones)
}
