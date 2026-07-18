pub struct Polygon {
    pub vertices: Vec<(i32, i32)>,
    pub holes: Vec<Vec<(i32, i32)>>,
}

impl Polygon {
    pub fn new(vertices: Vec<(i32, i32)>) -> Self {
        Self {
            vertices,
            holes: Vec::new(),
        }
    }

    pub fn add_hole(&mut self, vertices: Vec<(i32, i32)>) {
        self.holes.push(vertices);
    }
}