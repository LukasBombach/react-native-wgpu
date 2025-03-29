use taffy::prelude::*;

pub struct UserInterface {
    taffy: TaffyTree<()>,
    root: NodeId,
}

impl UserInterface {
    pub fn new() -> Self {
        let mut taffy = TaffyTree::new();

        let root = taffy
            .new_with_children(
                Style {
                    gap: Size {
                        width: length(10.0),
                        height: zero(),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        Self { taffy, root }
    }

    pub fn update(&mut self) {
        // Update the UI here
    }
}
