use crate::app::CustomEvent;
use crate::gui::node::Node;
use cosmic_text::{FontSystem, SwashCache};
use rustyscript::extensions::deno_io::fs;
use slotmap::{DefaultKey, SlotMap};
use std::cell::RefCell;
use std::convert::From;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use taffy::{
    compute_block_layout, compute_cached_layout, compute_flexbox_layout, compute_grid_layout,
    compute_root_layout, prelude::*, Layout, LayoutOutput, Style,
};
use winit::event_loop::EventLoopProxy;

pub struct Gui<'a> {
    root: NodeId,
    nodes: SlotMap<DefaultKey, Node<'a>>,
    event_loop: Arc<Mutex<EventLoopProxy<CustomEvent>>>,
    text_renderer: crate::gui::text::TextRenderer,
    pub font_system: Rc<RefCell<FontSystem>>,
    pub swash_cache: Rc<RefCell<SwashCache>>,
}

impl<'a> Gui<'a> {
    pub fn recompute_layout(&mut self, width: u32, height: u32) {
        let width = length(width as f32);
        let height = length(height as f32);
        compute_root_layout(self, self.root, Size { width, height });
    }
}

impl<'a> Gui<'a> {
    #[inline(always)]
    pub fn node_from_id(&self, node_id: NodeId) -> &Node<'a> {
        &self.nodes.get(node_id.into()).unwrap()
    }

    #[inline(always)]
    pub fn node_from_id_mut(&mut self, node_id: NodeId) -> &mut Node<'a> {
        self.nodes.get_mut(node_id.into()).unwrap()
    }
}

pub struct ChildIter<'a>(std::slice::Iter<'a, NodeId>);

impl Iterator for ChildIter<'_> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(NodeId::from)
    }
}

impl taffy::TraverseTree for Gui<'_> {}

impl taffy::TraversePartialTree for Gui<'_> {
    type ChildIter<'a>
        = ChildIter<'a>
    where
        Self: 'a;

    fn child_ids(&self, node_id: NodeId) -> Self::ChildIter<'_> {
        ChildIter(self.node_from_id(node_id).children().iter())
    }

    fn child_count(&self, node_id: NodeId) -> usize {
        self.node_from_id(node_id).children().len()
    }

    fn get_child_id(&self, node_id: NodeId, index: usize) -> NodeId {
        NodeId::from(self.node_from_id(node_id).children()[index])
    }
}

impl taffy::LayoutPartialTree for Gui<'_> {
    type CoreContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self.node_from_id(node_id).style()
    }

    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.node_from_id_mut(node_id).set_layout(*layout)
    }

    fn compute_child_layout(
        &mut self,
        node_id: NodeId,
        inputs: taffy::tree::LayoutInput,
    ) -> taffy::tree::LayoutOutput {
        compute_cached_layout(self, node_id, inputs, |gui, node_id, inputs| {
            // Only borrow font_system mutably when needed, before any mutable borrow of gui
            let node_ref = gui.node_from_id(node_id);

            match node_ref {
                Node::GridNode(block_node) => compute_grid_layout(gui, node_id, inputs),
                Node::FlexNode(block_node) => compute_flexbox_layout(gui, node_id, inputs),
                Node::BlockNode(block_node) => compute_block_layout(gui, node_id, inputs),
                Node::TextNode(_) => {
                    let fs = gui.font_system.clone();
                    let mut fs = fs.borrow_mut();
                    // Get a mutable reference to the TextNode
                    let text_node_mut = match gui.node_from_id_mut(node_id) {
                        Node::TextNode(text_node_mut) => text_node_mut,
                        _ => unreachable!(),
                    };
                    let mut buffer = text_node_mut.buffer.borrow_with(&mut fs);

                    // determine the width the text has to fit into
                    let available_space = inputs.available_space;
                    let known_dimensions = inputs.known_dimensions;

                    let width_constraint = known_dimensions.width.or(match available_space.width {
                        AvailableSpace::MinContent => Some(0.0),
                        AvailableSpace::MaxContent => None,
                        AvailableSpace::Definite(width) => Some(width),
                    });

                    buffer.set_size(width_constraint, None);

                    // Perform shaping as desired
                    buffer.shape_until_scroll(true);

                    // Determine measured width and height of text
                    let (width, total_lines) = buffer
                        .layout_runs()
                        .fold((0.0, 0usize), |(width, total_lines), run| {
                            (run.line_w.max(width), total_lines + 1)
                        });

                    let height = total_lines as f32 * buffer.metrics().line_height;

                    return LayoutOutput::from_outer_size(Size { width, height });
                }
            }
        })
    }
}

impl taffy::LayoutFlexboxContainer for Gui<'_> {
    type FlexboxContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    type FlexboxItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_flexbox_container_style(&self, node_id: NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.node_from_id(node_id).style()
    }

    fn get_flexbox_child_style(&self, child_node_id: NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.node_from_id(child_node_id).style()
    }
}

impl taffy::LayoutGridContainer for Gui<'_> {
    type GridContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    type GridItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_grid_container_style(&self, node_id: NodeId) -> Self::GridContainerStyle<'_> {
        &self.node_from_id(node_id).style()
    }

    fn get_grid_child_style(&self, child_node_id: NodeId) -> Self::GridItemStyle<'_> {
        &self.node_from_id(child_node_id).style()
    }
}

impl taffy::LayoutBlockContainer for Gui<'_> {
    type BlockContainerStyle<'a>
        = &'a Style
    where
        Self: 'a;

    type BlockItemStyle<'a>
        = &'a Style
    where
        Self: 'a;

    fn get_block_container_style(&self, node_id: NodeId) -> Self::BlockContainerStyle<'_> {
        &self.node_from_id(node_id).style()
    }

    fn get_block_child_style(&self, child_node_id: NodeId) -> Self::BlockItemStyle<'_> {
        &self.node_from_id(child_node_id).style()
    }
}

impl taffy::CacheTree for Gui<'_> {
    fn cache_get(
        &self,
        node_id: NodeId,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
        run_mode: taffy::RunMode,
    ) -> Option<taffy::LayoutOutput> {
        self.node_from_id(node_id)
            .cache()
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
        self.node_from_id_mut(node_id).cache_mut().store(
            known_dimensions,
            available_space,
            run_mode,
            layout_output,
        )
    }

    fn cache_clear(&mut self, node_id: NodeId) {
        self.node_from_id_mut(node_id).cache_mut().clear();
    }
}
