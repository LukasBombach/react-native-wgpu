use taffy::NodeId;

pub struct TextNode {
    pub children: Vec<NodeId>,
}

/* impl Node for TextNode {
    fn children(&self) -> &[NodeId] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.children
    }
} */
