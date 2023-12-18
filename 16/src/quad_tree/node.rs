use crate::geometry::Bounds;

use super::Positioned2D;

#[derive(Clone, Debug)]
pub struct Node<'t, T: Positioned2D> {
    pub parent_id: Option<u32>,
    pub content: NodeContent<'t, T>,
    pub bounds: Bounds
}

impl <'t, T: Positioned2D> Node<'t, T> {
    pub fn get_items(self) -> Vec<&'t T> {
        match self.content {
            NodeContent::Children { .. } => panic!("Trying to split a non-leaf node"),
            NodeContent::Items(items) => items
        }
     }
}

#[derive(Clone, Debug)]
pub enum NodeContent<'t, T: Positioned2D> {
    Items(Vec<&'t T>),
    Children {
        top_left_id: u32,
        top_right_id: u32,
        bottom_right_id: u32,
        bottom_left_id: u32
    }
}