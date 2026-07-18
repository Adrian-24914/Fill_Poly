pub struct Polygon {
    pub vertices: Vec<(i32, i32)>,
}

impl Polygon {
    pub fn new(vertices: Vec<(i32, i32)>) -> Self {
        Self { vertices }
    }
}
