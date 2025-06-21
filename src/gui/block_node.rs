use taffy::prelude::*;

pub struct BlockNode {
    pub layout: Layout,
    pub style: Style,
    pub children: Vec<NodeId>,
}
