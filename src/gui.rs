//! ## Example: Partial Tree with Directly Owned Children
//!
//! The following example demonstrate an implementation of Taffy's Partial trait and usage of the low-level compute APIs.
//! This example uses directly owned children with NodeId's being index's into vec on parent node.
//! Since an iterator created from a node can't access grandchildren, we are limited to only implement `TraversePartialTree`.
//! See the [`crate::tree::traits`] module for more details about the low-level traits.

use taffy::{
    compute_cached_layout, compute_flexbox_layout, compute_grid_layout, compute_root_layout,
    prelude::*, Cache, Layout, Style,
};

struct ChildIter(std::ops::Range<usize>);
impl Iterator for ChildIter {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(NodeId::from)
    }
}

#[derive(Debug, Copy, Clone)]
enum NodeKind {
    Flexbox,
    Grid,
}

pub struct Gui {
    nodes: Vec<Node>,
    root: usize,
}

impl Gui {
    pub fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.push(Node::default());
        let root = nodes.len() - 1;
        Self { nodes, root }
    }

    pub fn create_instance(&mut self, style: Style) -> usize {
        let kind = if style.display == Display::Grid {
            NodeKind::Grid
        } else {
            NodeKind::Flexbox
        };

        let node = Node {
            style,
            kind,
            ..Node::default()
        };

        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn append_child_to_container(&mut self, child: usize) -> () {
        self.nodes[self.root].append_child(child);
    }

    pub fn append_child(&mut self, parent: usize, child: usize) {
        self.nodes[parent].children.push(child);
    }
}

struct Node {
    kind: NodeKind,
    style: Style,
    cache: Cache,
    layout: Layout,
    children: Vec<usize>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            kind: NodeKind::Flexbox,
            style: Style::default(),
            cache: Cache::new(),
            layout: Layout::with_order(0),
            children: Vec::new(),
        }
    }
}

impl Node {
    pub fn append_child(&mut self, node: usize) {
        self.children.push(node);
    }

    pub fn compute_layout(&mut self, available_space: Size<AvailableSpace>) {
        compute_root_layout(self, NodeId::from(usize::MAX), available_space);
    }

    /// The methods on LayoutPartialTree need to be able to access:
    ///
    ///  - The node being laid out
    ///  - Direct children of the node being laid out
    ///
    /// Each must have an ID. For children we simply use it's index. For the node itself
    /// we use usize::MAX on the assumption that there will never be that many children.
    fn node_from_id(&self, node_id: NodeId) -> &Node {
        let idx = usize::from(node_id);
        if idx == usize::MAX {
            self
        } else {
            &self.children[idx]
        }
    }

    fn node_from_id_mut(&mut self, node_id: NodeId) -> &mut Node {
        let idx = usize::from(node_id);
        if idx == usize::MAX {
            self
        } else {
            &mut self.children[idx]
        }
    }
}

impl taffy::CacheTree for Node {
    fn cache_get(
        &self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<taffy::LayoutOutput> {
        self.node_from_id(node_id)
            .cache
            .get(known_dimensions, available_space, run_mode)
    }

    fn cache_store(
        &mut self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: taffy::RunMode,
        layout_output: taffy::LayoutOutput,
    ) {
        self.node_from_id_mut(node_id).cache.store(
            known_dimensions,
            available_space,
            run_mode,
            layout_output,
        )
    }

    fn cache_clear(&mut self, node_id: NodeId) {
        self.node_from_id_mut(node_id).cache.clear();
    }
}

impl taffy::TraversePartialTree for Node {
    type ChildIter<'a> = ChildIter;

    fn child_ids(&self, _node_id: NodeId) -> Self::ChildIter<'_> {
        ChildIter(0..self.children.len())
    }

    fn child_count(&self, _node_id: NodeId) -> usize {
        self.children.len()
    }

    fn get_child_id(&self, _node_id: NodeId, index: usize) -> NodeId {
        NodeId::from(index)
    }
}

impl taffy::LayoutPartialTree for Node {
    type CoreContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.node_from_id_mut(node_id).layout = *layout
    }

    fn compute_child_layout(
        &mut self,
        node_id: NodeId,
        inputs: taffy::tree::LayoutInput,
    ) -> taffy::tree::LayoutOutput {
        compute_cached_layout(self, node_id, inputs, |parent, node_id, inputs| {
            let node = parent.node_from_id_mut(node_id);

            match node.kind {
                NodeKind::Flexbox => compute_flexbox_layout(node, node_id, inputs),
                NodeKind::Grid => compute_grid_layout(node, node_id, inputs),
            }
        })
    }
}

impl taffy::LayoutFlexboxContainer for Node {
    type FlexboxContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    type FlexboxItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.node_from_id(child_node_id).style
    }
}

impl taffy::LayoutGridContainer for Node {
    type GridContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    type GridItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_grid_container_style(&self, node_id: NodeId) -> Self::GridContainerStyle<'_> {
        &self.node_from_id(node_id).style
    }

    fn get_grid_child_style(&self, child_node_id: NodeId) -> Self::GridItemStyle<'_> {
        &self.node_from_id(child_node_id).style
    }
}
