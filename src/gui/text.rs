use cosmic_text::{Buffer, FontSystem, Metrics, SwashCache};
use taffy::{AvailableSpace, LayoutInput, Size};

pub struct TextRenderer {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
}

impl TextRenderer {
    pub fn compute_text_layout(&mut self, text: &str, metrics: Metrics, inputs: LayoutInput) {
        // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
        let mut buffer = Buffer::new(&mut self.font_system, metrics);

        // Borrow buffer together with the font system for more convenient method calls
        let mut buffer = buffer.borrow_with(&mut self.font_system);

        // determine the width constraint
        let available_space = inputs.available_space;
        let known_dimensions = inputs.known_dimensions;

        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });

        buffer.set_size(width_constraint, None);

        buffer.shape_until_scroll(true);
    }
}
