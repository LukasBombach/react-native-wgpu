use crate::gui::BlockNode;
use crate::gui::TextNode;
use taffy::NodeId;
/* ;

pub trait Node {
    fn children(&self) -> &[NodeId];
    fn children_mut(&mut self) -> &mut Vec<NodeId>;
} */

pub enum Node {
    BlockNode(BlockNode),
    TextNode(TextNode),
}

impl Node {
    pub fn children(&self) -> &Vec<NodeId> {
        match self {
            Node::BlockNode(block_node) => &block_node.children,
            Node::TextNode(text_node) => &text_node.children,
        }
    }
}
