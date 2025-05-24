use crate::gpu::Instance;
use slotmap::{DefaultKey, SlotMap};
use taffy::{
    compute_cached_layout, compute_flexbox_layout, compute_grid_layout, compute_root_layout,
    prelude::*, Cache, Layout, Style,
};

#[derive(Debug, Copy, Clone)]
enum NodeKind {
    Flexbox,
    Grid,
}

pub struct Node {
    kind: NodeKind,
    style: Style,
    cache: Cache,
    pub layout: Layout,
    pub children: Vec<NodeId>,
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
    pub fn append_child(&mut self, node: NodeId) {
        self.children.push(node);
    }
}

pub struct Gui {
    pub root: NodeId,
    nodes: SlotMap<DefaultKey, Node>,
}

impl Gui {
    pub fn new() -> Self {
        let mut nodes = SlotMap::new();
        let root = nodes.insert(Self::create_root()).into();
        Self { root, nodes }
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

    pub fn create_node(&mut self, style: Style) -> NodeId {
        // todo block layout
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

        let id = self.nodes.insert(node);

        id.into()
    }

    pub fn append_child_to_root(&mut self, child_id: NodeId) -> () {
        self.append_child(self.root, child_id);
    }

    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        if let Some(parent) = self.nodes.get_mut(parent_id.into()) {
            parent.append_child(child_id)
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
        self.root = self.nodes.insert(Node::default()).into();
    }

    pub fn compute_layout(&mut self, width: f32, height: f32) {
        compute_root_layout(
            self,
            NodeId::from(self.root),
            Size {
                width: length(width),
                height: length(height),
            },
        );
    }

    pub fn get_instances(&mut self, width: f32, height: f32) -> Option<Vec<Instance>> {
        fn collect_instances(
            gui: &Gui,
            node_id: taffy::NodeId,
            offset_x: f32,
            offset_y: f32,
            instances: &mut Vec<Instance>,
        ) {
            let layout = gui.layout_from_id(node_id);
            let (x, y) = (offset_x + layout.location.x, offset_y + layout.location.y);
            instances.push(Instance::new(x, y, layout.size.width, layout.size.height));

            for child_id in gui.children_from_id(node_id) {
                collect_instances(gui, *child_id, x, y, instances);
            }
        }

        self.compute_layout(width, height);

        let mut instances = Vec::new();
        collect_instances(&self, self.root, 0.0, 0.0, &mut instances);

        Some(instances)
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
