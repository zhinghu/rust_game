use crate::kernel::console;
use ndarray::*;

pub fn color_to_term(vec4: ArrayBase<OwnedRepr<f64>, Dim<[usize; 1]>>) -> String {
    assert_eq!(vec4.len(), 4);
    let result = format!(
        "\x1b[48;2;{:?};{:?};{:?}m\u{0020}\x1b[0m",
        vec4[0] as u8, vec4[1] as u8, vec4[2] as u8
    );

    result
}
