use taffy::prelude::*;

#[derive(Debug)]
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
                    size: Size {
                        width: percent(100.0),
                        height: percent(100.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();
        Self { taffy, root }
    }

    pub fn create_node(&mut self, top: u32, left: u32, width: u32, height: u32) -> NodeId {
        self.taffy
            .new_with_children(
                Style {
                    margin: Rect {
                        top: length(top as f32),
                        left: length(left as f32),
                        bottom: length(0.0),
                        right: length(0.0),
                    },
                    size: Size {
                        width: length(width as f32),
                        height: length(height as f32),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap()
    }

    pub fn add_child_to_root(&mut self, node_id: NodeId) {
        self.taffy.add_child(self.root, node_id).unwrap();
    }

    pub fn clear(&mut self) {
        self.taffy.clear();
    }
}
