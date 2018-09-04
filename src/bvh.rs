
use std::option::*;
use hit::*;

pub struct BvhNode {
    object: &'Box<Hit>,
    left:   Option<Box<BvhNode>>,
    right:  Option<Box<BvhNode>>,
    z:      u32
}

impl BvhNode {
    pub fn new(object: &'Box<Hit>, z: u32) -> Self {
        BvhNode {
            object,
            left:  None(),
            right: None(),
            z
        }
    }
    pub fn set_left(&self, left: Box<BvhNode>) {
        self.left = Some(left);
    }
    pub fn set_right(&self, right: Box<BvhNode>) {
        self.right = Some(right);
    }
    pub fn get_z(&self) -> u32 {
        self.z
    }
    pub fn compare_z(&self, z: u32) -> bool {
        self.z > z
    }
}