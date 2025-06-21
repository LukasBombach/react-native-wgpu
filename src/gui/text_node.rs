use taffy::prelude::*;

pub struct TextNode {
    pub layout: Layout,
    pub style: Style,
    pub children: Vec<NodeId>,
}
