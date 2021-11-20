#[derive(Debug, Clone)]
pub struct Node {
    pub x: usize,
    pub y: usize,
    pub elevation: f32,
    pub depth: f32,
}

impl Node {
    pub fn new() -> Node {
        Node {
            x: 0,
            y: 0,
            elevation: 0.0,
            depth: 0.0,
        }
    }

    pub fn new_with_xy(x: usize, y: usize) -> Node {
        Node {
            x,
            y,
            elevation: 0.0,
            depth: 0.0,
        }
    }

    pub fn new_with_xyz(x: usize, y: usize, z: f32) -> Node {
        Node {
            x,
            y,
            elevation: z,
            depth: 0.0,
        }
    }

    pub fn new_with_xyzh(x: usize, y: usize, z: f32) -> Node {
        Node {
            x,
            y,
            elevation: z,
            depth: 0.0,
        }
    }

    pub fn water_level(&self) -> f32 {
        self.elevation + self.depth
    }
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn test_water_level() {
        let mut node = Node::new_with_xyz(0, 0, 10.5);
        node.depth = 0.5;

        assert_eq!(node.water_level(), 11.0);
    }
}
