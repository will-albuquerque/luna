use imageproc::point::Point;

#[derive(Debug)]
pub struct Triangle {
    vertices: [Point<i32>; 3],
}

impl Triangle {
    pub fn new(vertices: [Point<i32>; 3]) -> Self {
        Triangle { vertices }
    }

    pub fn into_vertices(self) -> [Point<i32>; 3] {
        self.vertices
    }
}
