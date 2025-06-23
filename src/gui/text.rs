use crate::gui::node::TextNode;
use cosmic_text::{Buffer, FontSystem, SwashCache};
use taffy::{AvailableSpace, LayoutInput, LayoutOutput, NodeId, Size};

pub trait LayoutTextContainer {
    fn node_from_id_mut(&mut self, node_id: NodeId) -> &mut TextNode;
    fn node_from_id(&self, node_id: NodeId) -> &TextNode;
}

pub fn compute_text_layout(
    tree: &mut impl LayoutTextContainer,
    font_system: &mut FontSystem,
    node: NodeId,
    inputs: LayoutInput,
) -> LayoutOutput {
    let text_node = tree.node_from_id_mut(node);
    let mut buffer = text_node.buffer.borrow_with(font_system);

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

pub struct TextRenderer {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
}

impl TextRenderer {
    pub fn compute_text_layout(
        &mut self,
        tree: &mut impl LayoutTextContainer,
        node: NodeId,
        inputs: LayoutInput,
    ) -> LayoutOutput {
        // Borrow buffer together with the font system for more convenient method calls
        let text_node = tree.node_from_id_mut(node);
        let mut buffer = text_node.buffer.borrow_with(&mut self.font_system);

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
