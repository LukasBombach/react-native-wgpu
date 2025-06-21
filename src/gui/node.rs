use crate::gui::BlockNode;
use crate::gui::TextNode;
use taffy::prelude::*;

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

    pub fn layout(&self) -> &Layout {
        match self {
            Node::BlockNode(block_node) => &block_node.layout,
            Node::TextNode(text_node) => &text_node.layout,
        }
    }

    pub fn style(&self) -> &Style {
        match self {
            Node::BlockNode(block_node) => &block_node.style,
            Node::TextNode(text_node) => &text_node.style,
        }
    }
}
