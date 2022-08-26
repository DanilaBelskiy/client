pub struct CornerMemory {
    pub axis_corner_a: f32,
    pub axis_corner_b: f32,
    pub axis_corner_c: f32,
    pub axis_corner_d: f32,
}

impl CornerMemory {
    pub fn new() -> CornerMemory {
        CornerMemory{
            axis_corner_a: 0.0,
            axis_corner_b: 0.0,
            axis_corner_c: 0.0,
            axis_corner_d: 0.0,
        }
    }

    pub fn make_negative(mem: &CornerMemory) -> CornerMemory {
        CornerMemory{
            axis_corner_a: mem.axis_corner_a * (-1.0),
            axis_corner_b: mem.axis_corner_b * (-1.0),
            axis_corner_c: mem.axis_corner_c * (-1.0),
            axis_corner_d: mem.axis_corner_d * (-1.0),
        }
    }
}