use taffy::prelude::{Layout, NodeId, Style};

pub enum Node {
    GridNode(GridNode),
    FlexNode(FlexNode),
    BlockNode(BlockNode),
    TextNode(TextNode),
}

pub struct GridNode {
    pub layout: Layout,
    pub style: Style,
    pub children: Vec<NodeId>,
}

pub struct FlexNode {
    pub layout: Layout,
    pub style: Style,
    pub children: Vec<NodeId>,
}

pub struct BlockNode {
    pub layout: Layout,
    pub style: Style,
    pub children: Vec<NodeId>,
}

pub struct TextNode {
    pub layout: Layout,
    pub style: Style,
    pub children: Vec<NodeId>,
}

impl Node {
    pub fn children(&self) -> &Vec<NodeId> {
        match self {
            Node::GridNode(block_node) => &block_node.children,
            Node::FlexNode(block_node) => &block_node.children,
            Node::BlockNode(block_node) => &block_node.children,
            Node::TextNode(text_node) => &text_node.children,
        }
    }

    pub fn layout(&self) -> &Layout {
        match self {
            Node::GridNode(block_node) => &block_node.layout,
            Node::FlexNode(block_node) => &block_node.layout,
            Node::BlockNode(block_node) => &block_node.layout,
            Node::TextNode(text_node) => &text_node.layout,
        }
    }

    pub fn set_layout(&mut self, layout: Layout) {
        match self {
            Node::GridNode(block_node) => block_node.layout = layout,
            Node::FlexNode(block_node) => block_node.layout = layout,
            Node::BlockNode(block_node) => block_node.layout = layout,
            Node::TextNode(text_node) => text_node.layout = layout,
        }
    }

    pub fn style(&self) -> &Style {
        match self {
            Node::GridNode(block_node) => &block_node.style,
            Node::FlexNode(block_node) => &block_node.style,
            Node::BlockNode(block_node) => &block_node.style,
            Node::TextNode(text_node) => &text_node.style,
        }
    }
}
