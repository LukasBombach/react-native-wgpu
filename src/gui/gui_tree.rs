use crate::app::CustomEvent;
use crate::gpu::Instance;
use crate::gui::block_node::BlockNode;
use crate::gui::text_node::TextNode;
use slotmap::{DefaultKey, SlotMap};
use std::convert::From;
use std::sync::Arc;
use std::sync::Mutex;
use taffy::{
    compute_cached_layout, compute_flexbox_layout, compute_grid_layout, compute_root_layout,
    prelude::*, Cache, Layout, Style,
};
use winit::event_loop::EventLoopProxy;

pub struct Gui {
    pub root: NodeId,
    block_nodes: SlotMap<DefaultKey, BlockNode>,
    text_nodes: SlotMap<DefaultKey, TextNode>,
    event_loop: Arc<Mutex<EventLoopProxy<CustomEvent>>>,
}

pub struct ChildIter<'a>(std::slice::Iter<'a, NodeId>);

impl Iterator for ChildIter<'_> {
    type Item = NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().map(NodeId::from)
    }
}

impl taffy::TraversePartialTree for Gui {
    type ChildIter<'a> = ChildIter<'a>;

    fn child_ids(&self, node_id: NodeId) -> Self::ChildIter<'_> {
        ChildIter(self.node_from_id(node_id).children.iter())
    }

    fn child_count(&self, node_id: NodeId) -> usize {
        self.node_from_id(node_id).children.len()
    }

    fn get_child_id(&self, node_id: NodeId, index: usize) -> NodeId {
        NodeId::from(self.node_from_id(node_id).children[index])
    }
}

impl taffy::TraverseTree for Gui {}

impl taffy::LayoutPartialTree for Gui {
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
        compute_cached_layout(self, node_id, inputs, |gui, node_id, inputs| {
            let node = gui.node_from_id_mut(node_id);

            match node.kind {
                NodeKind::Flexbox => compute_flexbox_layout(gui, node_id, inputs),
                NodeKind::Grid => compute_grid_layout(gui, node_id, inputs),
                NodeKind::Text => {
                    // Text nodes are leaf nodes with intrinsic size
                    taffy::tree::LayoutOutput {
                        size: taffy::Size {
                            width: node.font_size
                                * node.text.as_ref().map_or(0, |t| t.len()) as f32
                                * 0.6, // Rough text width estimation
                            height: node.font_size,
                        },
                        content_size: taffy::Size::ZERO,
                        first_baselines: taffy::Point::NONE,
                        top_margin: taffy::CollapsibleMarginSet::ZERO,
                        bottom_margin: taffy::CollapsibleMarginSet::ZERO,
                        margins_can_collapse_through: false,
                    }
                }
            }
        })
    }
}

impl taffy::LayoutFlexboxContainer for Gui {
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

impl taffy::LayoutGridContainer for Gui {
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

impl taffy::CacheTree for Gui {
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
