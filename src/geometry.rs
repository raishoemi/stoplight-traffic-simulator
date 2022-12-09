#[rustfmt::skip]
pub fn cube(length: f32) -> Vec<f32> {
    let half_len: f32 = length / 2.0;
    let mut vertices = vec![];
    // front-back
    vertices.extend(vec![
        -half_len, half_len, half_len,
        half_len, half_len, half_len,
        half_len, -half_len, half_len,
    ]);
    vertices.extend(vec![
        half_len, -half_len, half_len,
        -half_len, -half_len, half_len,
        -half_len, half_len, half_len,
    ]);
    vertices.extend(vec![
        -half_len, half_len, -half_len,
        half_len, half_len, -half_len,
        half_len, -half_len, -half_len,
    ]);
    vertices.extend(vec![
        half_len, -half_len, -half_len,
        -half_len, -half_len, -half_len,
        -half_len, half_len, -half_len,
    ]);

    // sides
    vertices.extend(vec![
        half_len, half_len, half_len,
        half_len, half_len, -half_len,
        half_len, -half_len, half_len,
    ]);
    vertices.extend(vec![
        half_len, -half_len, -half_len,
        half_len, -half_len, half_len,
        half_len, half_len, -half_len,
    ]);
    vertices.extend(vec![
        -half_len, half_len, half_len,
        -half_len, half_len, -half_len,
        -half_len, -half_len, half_len,
    ]);
    vertices.extend(vec![
        -half_len, -half_len, -half_len,
        -half_len, -half_len, half_len,
        -half_len, half_len, -half_len,
    ]);

    // bottom-top
    vertices.extend(vec![
        half_len, half_len, half_len,
        half_len, half_len, -half_len,
        -half_len, half_len, half_len,
    ]);
    vertices.extend(vec![
        -half_len, half_len, -half_len,
        half_len, half_len, -half_len,
        -half_len, half_len, half_len,
    ]);
    vertices.extend(vec![
        half_len, -half_len, half_len,
        half_len, -half_len, -half_len,
        -half_len, -half_len, half_len,
    ]);
    vertices.extend(vec![
        -half_len, -half_len, -half_len,
        half_len, -half_len, -half_len,
        -half_len, -half_len, half_len,
    ]);
    vertices
}
