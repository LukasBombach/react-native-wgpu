use crate::app::CustomEvent;
use crate::gpu::Instance;
use slotmap::{DefaultKey, SlotMap};
use std::convert::From;
use std::sync::Arc;
use std::sync::Mutex;
use taffy::util::print_tree;
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
    cache: Cache,
    text_buffer: Option<glyphon::Buffer>,
    pub layout: Layout,
    pub children: Vec<NodeId>,
    pub text_content: Option<String>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            kind: NodeKind::Flexbox,
            style: Style::default(),
            background_color: [0.0, 0.0, 0.0, 0.0],
            border_radius: 0.0,
            cache: Cache::new(),
            layout: Layout::with_order(0),
            children: Vec::new(),
            text_content: None,
            text_buffer: None,
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
    font_system: glyphon::FontSystem,
    event_loop: Arc<Mutex<EventLoopProxy<CustomEvent>>>,
}

#[allow(dead_code)]
impl Gui {
    pub fn new(event_loop: Arc<Mutex<EventLoopProxy<CustomEvent>>>) -> Self {
        let mut nodes = SlotMap::new();
        let root = nodes.insert(Self::create_root()).into();
        let font_system = glyphon::FontSystem::new();
        Self {
            root,
            nodes,
            font_system,
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
            kind,
            ..Node::default()
        };

        println!("Creating node with border_radius: {:?}", node.border_radius);

        let id = self.nodes.insert(node);

        id.into()
    }

    pub fn create_text_node(&mut self, text: String) -> NodeId {
        let node = Node {
            kind: NodeKind::Text,
            text_content: Some(text),
            ..Node::default()
        };

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

    /* #[inline(always)]
    pub fn text_buffer_from_id_mut(&self, node_id: NodeId) -> &mut Option<glyphon::Buffer> {
        &self.nodes.get_mut(node_id.into()).unwrap().text_buffer
    } */

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
            let instance = Instance::new(
                x,
                y,
                node.layout.size.width,
                node.layout.size.height,
                node.background_color,
                node.border_radius,
            );

            instances.push(instance);

            for child_id in gui.children_from_id(node_id) {
                collect_instances(gui, *child_id, x, y, instances);
            }
        }

        let mut instances = Vec::new();
        collect_instances(&self, self.root, 0.0, 0.0, &mut instances);
        return instances;
    }

    pub fn into_text_areas(&mut self) -> Vec<glyphon::Buffer> {
        fn collect_text_buffers(
            gui: &Gui,
            node_id: taffy::NodeId,
            buffers: &mut Vec<glyphon::Buffer>,
        ) {
            let node = gui.node_from_id(node_id);
            if let Some(buffer) = &node.text_buffer {
                buffers.push(buffer.clone());
            }

            for child_id in gui.children_from_id(node_id) {
                collect_text_buffers(gui, *child_id, buffers);
            }
        }

        let mut buffers = Vec::new();
        collect_text_buffers(self, self.root, &mut buffers);
        return buffers;
    }

    /* pub fn update_text_content(&mut self, node_id: NodeId, new_text: String) {
        if let Some(node) = self.nodes.get_mut(node_id.into()) {
            node.text_content = Some(new_text.clone());
            // Clear the text buffer so it gets recreated with the new content
            node.text_buffer = None;
            // Clear cache to force layout recalculation
            node.cache.clear();
            self.notify_update();
        }
    } */

    fn notify_update(&self) {
        if let Ok(proxy) = self.event_loop.lock() {
            proxy.send_event(CustomEvent::GuiUpdate).unwrap();
        }
    }

    pub fn print_tree(&mut self) {
        print_tree(self, self.root);
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
            let node_kind = gui.node_from_id(node_id).kind;

            match node_kind {
                NodeKind::Flexbox => compute_flexbox_layout(gui, node_id, inputs),
                NodeKind::Grid => compute_grid_layout(gui, node_id, inputs),
                NodeKind::Text => {
                    // Get text content first
                    let text_content = gui.node_from_id(node_id).text_content.clone();
                    let needs_buffer_creation = gui.node_from_id(node_id).text_buffer.is_none();

                    // Create buffer if needed
                    if needs_buffer_creation && text_content.is_some() {
                        let text = text_content.as_ref().unwrap();
                        let mut buffer = glyphon::Buffer::new(
                            &mut gui.font_system,
                            glyphon::Metrics::new(16.0, 20.0),
                        );
                        let attrs = glyphon::Attrs::new().family(glyphon::Family::SansSerif);
                        buffer.set_text(
                            &mut gui.font_system,
                            text,
                            &attrs,
                            glyphon::Shaping::Advanced,
                        );
                        gui.node_from_id_mut(node_id).text_buffer = Some(buffer);
                    }

                    // Now handle layout computation - split mutable borrows properly
                    let node = gui.node_from_id_mut(node_id);

                    if text_content.is_some() && node.text_buffer.is_some() {
                        let available_space = inputs.available_space;
                        let known_dimensions = inputs.known_dimensions;

                        // Set width constraint
                        let width_constraint =
                            known_dimensions.width.or(match available_space.width {
                                AvailableSpace::MinContent => Some(0.0),
                                AvailableSpace::MaxContent => None,
                                AvailableSpace::Definite(width) => Some(width),
                            });

                        // We need to get font_system reference after getting the node reference
                        // This creates a borrowing issue, so we need a different approach

                        // Store the text buffer temporarily to avoid double borrow
                        let mut temp_buffer = node.text_buffer.take().unwrap();

                        // Now we can borrow font_system mutably
                        temp_buffer.set_size(&mut gui.font_system, width_constraint, None);
                        temp_buffer.shape_until_scroll(&mut gui.font_system, false);

                        // Determine measured size of text
                        let (width, total_lines) = temp_buffer
                            .layout_runs()
                            .fold((0.0, 0usize), |(width, total_lines), run| {
                                (run.line_w.max(width), total_lines + 1)
                            });
                        let height = total_lines as f32 * temp_buffer.metrics().line_height;

                        // Put the buffer back
                        gui.node_from_id_mut(node_id).text_buffer = Some(temp_buffer);

                        return taffy::tree::LayoutOutput {
                            size: taffy::Size { width, height },
                            content_size: taffy::Size::ZERO,
                            first_baselines: taffy::Point::NONE,
                            top_margin: taffy::CollapsibleMarginSet::ZERO,
                            bottom_margin: taffy::CollapsibleMarginSet::ZERO,
                            margins_can_collapse_through: false,
                        };
                    }

                    // Fallback for empty text or no buffer
                    return taffy::tree::LayoutOutput {
                        size: taffy::Size {
                            width: 0.0,
                            height: 0.0,
                        },
                        content_size: taffy::Size::ZERO,
                        first_baselines: taffy::Point::NONE,
                        top_margin: taffy::CollapsibleMarginSet::ZERO,
                        bottom_margin: taffy::CollapsibleMarginSet::ZERO,
                        margins_can_collapse_through: false,
                    };
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

impl taffy::PrintTree for Gui {
    fn get_debug_label(&self, node_id: NodeId) -> &'static str {
        match self.node_from_id(node_id).kind {
            NodeKind::Flexbox => "FLEX",
            NodeKind::Grid => "GRID",
            NodeKind::Text => "TEXT",
        }
    }

    fn get_final_layout(&self, node_id: NodeId) -> &Layout {
        &self.node_from_id(node_id).layout
    }
}
