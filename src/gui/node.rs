use cosmic_text::{Attrs, Buffer, Metrics, Shaping};
use taffy::{Cache, Layout, NodeId, Style};

pub enum Node<'a> {
    GridNode(GridNode),
    FlexNode(FlexNode),
    BlockNode(BlockNode),
    TextNode(TextNode<'a>),
}

pub struct GridNode {
    pub layout: Layout,
    pub style: Style,
    pub children: Vec<NodeId>,
    pub cache: Cache,
}

pub struct FlexNode {
    pub layout: Layout,
    pub style: Style,
    pub children: Vec<NodeId>,
    pub cache: Cache,
}

pub struct BlockNode {
    pub layout: Layout,
    pub style: Style,
    pub children: Vec<NodeId>,
    pub cache: Cache,
}

pub struct TextNode<'a> {
    pub layout: Layout,
    pub style: Style,
    pub children: Vec<NodeId>,
    pub cache: Cache,

    pub text: &'a str,

    metrics: Metrics, // Text metrics indicate the font size and line height of a buffer
    buffer: Buffer, // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
    attrs: Attrs<'a>, // Attributes indicate what font to choose
}

/* impl TextNode<'_> {
    pub fn compute_layout(&mut self) {
        let mut buffer = Buffer::new(&self.font_system, self.metrics);
    }
} */

impl Node<'_> {
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

    pub fn cache(&self) -> &Cache {
        match self {
            Node::GridNode(block_node) => &block_node.cache,
            Node::FlexNode(block_node) => &block_node.cache,
            Node::BlockNode(block_node) => &block_node.cache,
            Node::TextNode(text_node) => &text_node.cache,
        }
    }

    pub fn cache_mut(&mut self) -> &mut Cache {
        match self {
            Node::GridNode(block_node) => &mut block_node.cache,
            Node::FlexNode(block_node) => &mut block_node.cache,
            Node::BlockNode(block_node) => &mut block_node.cache,
            Node::TextNode(text_node) => &mut text_node.cache,
        }
    }
}
