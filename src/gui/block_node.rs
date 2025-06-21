// use crate::gui::Node;
use taffy::NodeId;

pub struct BlockNode {
    pub children: Vec<NodeId>,
}

/* impl Node for BlockNode {
    fn children(&self) -> &[NodeId] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.children
    }
}
 */
