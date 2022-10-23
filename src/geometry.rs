pub fn cube(length: f32, color: (f32, f32, f32)) -> Vec<f32> {
    let half_len: f32 = length / 2.0;
    let mut vertices = vec![];
    // front-back
    vertices.extend(vec![
        -half_len, half_len, half_len, half_len, half_len, half_len, half_len, -half_len, half_len,
    ]);
    vertices.extend(vec![
        half_len, -half_len, half_len, -half_len, -half_len, half_len, -half_len, half_len,
        half_len,
    ]);
    vertices.extend(vec![
        -half_len, half_len, -half_len, half_len, half_len, -half_len, half_len, -half_len,
        -half_len,
    ]);
    vertices.extend(vec![
        half_len, -half_len, -half_len, -half_len, -half_len, -half_len, -half_len, half_len,
        -half_len,
    ]);

    // sides
    vertices.extend(vec![
        half_len, half_len, half_len, half_len, half_len, -half_len, half_len, -half_len, half_len,
    ]);
    vertices.extend(vec![
        half_len, -half_len, -half_len, half_len, -half_len, half_len, half_len, half_len,
        -half_len,
    ]);
    vertices.extend(vec![
        -half_len, half_len, half_len, -half_len, half_len, -half_len, -half_len, -half_len,
        half_len,
    ]);
    vertices.extend(vec![
        -half_len, -half_len, -half_len, -half_len, -half_len, half_len, -half_len, half_len,
        -half_len,
    ]);

    // bottom-top
    vertices.extend(vec![
        half_len, half_len, half_len, half_len, half_len, -half_len, -half_len, half_len, half_len,
    ]);
    vertices.extend(vec![
        -half_len, half_len, -half_len, half_len, half_len, -half_len, -half_len, half_len,
        half_len,
    ]);
    vertices.extend(vec![
        half_len, -half_len, half_len, half_len, -half_len, -half_len, -half_len, -half_len,
        half_len,
    ]);
    vertices.extend(vec![
        -half_len, -half_len, -half_len, half_len, -half_len, -half_len, -half_len, -half_len,
        half_len,
    ]);
    append_color(&mut vertices, color);
    vertices
}

fn append_color(positions: &mut Vec<f32>, color: (f32, f32, f32)) {
    let positions_len = positions.len() / 3;
    for n in (1..=positions_len).rev() {
        println!("{:?}", n);
        positions.insert(n * 3, color.0);
        positions.insert((n * 3) + 1, color.1);
        positions.insert((n * 3) + 2, color.2);
    }
}
