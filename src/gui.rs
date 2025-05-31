use crate::app::CustomEvent;
use crate::gpu::Instance;
use slotmap::{DefaultKey, SlotMap};
use std::convert::From;
use std::sync::Arc;
use std::sync::Mutex;
use taffy::{
    compute_cached_layout, compute_flexbox_layout, compute_grid_layout, compute_root_layout,
    prelude::*, Cache, Layout, Style,
};
use winit::event_loop::EventLoopProxy;

#[derive(Debug, Copy, Clone)]
enum NodeKind {
    Flexbox,
    Grid,
    Text,
}

pub struct Node {
    kind: NodeKind,
    style: Style,
    background_color: [f32; 4],
    border_radius: f32,
    text: Option<String>,
    text_color: [f32; 4],
    font_size: f32,
    cache: Cache,
    pub layout: Layout,
    pub children: Vec<NodeId>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            kind: NodeKind::Flexbox,
            style: Style::default(),
            background_color: [0.0, 0.0, 0.0, 0.0],
            border_radius: 0.0,
            text: None,
            text_color: [1.0, 1.0, 1.0, 1.0], // White by default
            font_size: 16.0,
            cache: Cache::new(),
            layout: Layout::with_order(0),
            children: Vec::new(),
        }
    }
}

impl Node {
    pub fn append_child(&mut self, node: NodeId) {
        self.children.push(node);
        self.cache.clear();
    }
}

pub struct Gui {
    pub root: NodeId,
    nodes: SlotMap<DefaultKey, Node>,
    event_loop: Arc<Mutex<EventLoopProxy<CustomEvent>>>,
}

#[allow(dead_code)]
impl Gui {
    pub fn new(event_loop: Arc<Mutex<EventLoopProxy<CustomEvent>>>) -> Self {
        let mut nodes = SlotMap::new();
        let root = nodes.insert(Self::create_root()).into();
        Self {
            root,
            nodes,
            event_loop,
        }
    }

    fn create_root() -> Node {
        let style = Style {
            display: Display::Flex,
            size: Size {
                width: percent(1.0),
                height: percent(1.0),
            },
            ..Default::default()
        };

        Node {
            style,
            kind: NodeKind::Flexbox,
            ..Node::default()
        }
    }

    pub fn create_node(
        &mut self,
        style: Style,
        background_color: [f32; 4],
        border_radius: u32,
    ) -> NodeId {
        self.create_node_with_text(
            style,
            background_color,
            border_radius,
            None,
            [1.0, 1.0, 1.0, 1.0],
            16.0,
        )
    }

    pub fn create_node_with_text(
        &mut self,
        style: Style,
        background_color: [f32; 4],
        border_radius: u32,
        text: Option<String>,
        text_color: [f32; 4],
        font_size: f32,
    ) -> NodeId {
        // todo block layout
        let kind = if style.display == Display::Grid {
            NodeKind::Grid
        } else {
            NodeKind::Flexbox
        };

        let node = Node {
            style,
            background_color,
            border_radius: border_radius as f32,
            text,
            text_color,
            font_size,
            kind,
            ..Node::default()
        };

        let id = self.nodes.insert(node);

        id.into()
    }

    pub fn create_text_node(
        &mut self,
        text: String,
        text_color: [f32; 4],
        font_size: f32,
    ) -> NodeId {
        let node = Node {
            kind: NodeKind::Text,
            style: Style::default(),
            background_color: [0.0, 0.0, 0.0, 0.0], // Transparent background for text
            border_radius: 0.0,
            text: Some(text),
            text_color,
            font_size,
            cache: Cache::new(),
            layout: Layout::with_order(0),
            children: Vec::new(),
        };

        println!(
            "create_text_node {:?} {:?} {:?}",
            node.text, node.text_color, node.font_size
        );

        let id = self.nodes.insert(node);
        id.into()
    }

    pub fn append_child_to_root(&mut self, child_id: NodeId) -> () {
        self.append_child(self.root, child_id);
        self.notify_update();
    }

    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        if let Some(parent) = self.nodes.get_mut(parent_id.into()) {
            parent.append_child(child_id);
            self.notify_update();
        }
    }

    #[inline(always)]
    pub fn node_from_id(&self, node_id: NodeId) -> &Node {
        &self.nodes.get(node_id.into()).unwrap()
    }

    #[inline(always)]
    pub fn node_from_id_mut(&mut self, node_id: NodeId) -> &mut Node {
        self.nodes.get_mut(node_id.into()).unwrap()
    }

    #[inline(always)]
    pub fn children_from_id(&self, node_id: NodeId) -> &Vec<NodeId> {
        &self.nodes.get(node_id.into()).unwrap().children
    }

    #[inline(always)]
    pub fn layout_from_id(&self, node_id: NodeId) -> &Layout {
        &self.nodes.get(node_id.into()).unwrap().layout
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.root = self.nodes.insert(Self::create_root()).into();
        self.notify_update();
    }

    pub fn compute_layout(&mut self, width: u32, height: u32) {
        compute_root_layout(
            self,
            NodeId::from(self.root),
            Size {
                width: length(width as f32),
                height: length(height as f32),
            },
        );
    }

    pub fn into_instances(&mut self) -> Vec<Instance> {
        fn collect_instances(
            gui: &Gui,
            node_id: taffy::NodeId,
            offset_x: f32,
            offset_y: f32,
            instances: &mut Vec<Instance>,
        ) {
            let node = gui.node_from_id(node_id);
            let (x, y) = (
                offset_x + node.layout.location.x,
                offset_y + node.layout.location.y,
            );

            // Only create background rectangles for non-text nodes
            if !matches!(node.kind, NodeKind::Text) {
                let instance = Instance::new(
                    x,
                    y,
                    node.layout.size.width,
                    node.layout.size.height,
                    node.background_color,
                    node.border_radius,
                );
                instances.push(instance);
            }

            for child_id in gui.children_from_id(node_id) {
                collect_instances(gui, *child_id, x, y, instances);
            }
        }

        let mut instances = Vec::new();
        collect_instances(&self, self.root, 0.0, 0.0, &mut instances);
        return instances;
    }

    pub fn collect_text_instances(&self) -> Vec<(String, f32, f32, f32, [f32; 4])> {
        fn collect_text(
            gui: &Gui,
            node_id: taffy::NodeId,
            offset_x: f32,
            offset_y: f32,
            text_items: &mut Vec<(String, f32, f32, f32, [f32; 4])>,
        ) {
            let node = gui.node_from_id(node_id);
            let (x, y) = (
                offset_x + node.layout.location.x,
                offset_y + node.layout.location.y,
            );

            // Only collect text from Text nodes
            if matches!(node.kind, NodeKind::Text) {
                if let Some(ref text) = node.text {
                    text_items.push((text.clone(), x, y, node.font_size, node.text_color));
                }
            }

            for child_id in gui.children_from_id(node_id) {
                collect_text(gui, *child_id, x, y, text_items);
            }
        }

        let mut text_items = Vec::new();
        collect_text(&self, self.root, 0.0, 0.0, &mut text_items);
        text_items
    }

    fn notify_update(&self) {
        if let Ok(proxy) = self.event_loop.lock() {
            proxy.send_event(CustomEvent::GuiUpdate).unwrap();
        }
    }
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
