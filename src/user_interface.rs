use taffy::prelude::*;

#[derive(Debug)]
pub struct UserInterface {
    pub taffy: TaffyTree<()>,
    pub root: NodeId,
}

impl UserInterface {
    pub fn new() -> Self {
        let mut taffy = TaffyTree::new();
        let root = Self::create_root(&mut taffy);
        Self { taffy, root }
    }

    pub fn create_node(&mut self, style: Style) -> NodeId {
        self.taffy.new_with_children(style, &[]).unwrap()
    }

    pub fn add_child_to_root(&mut self, node_id: NodeId) {
        self.taffy.add_child(self.root, node_id).unwrap();
    }

    pub fn clear(&mut self) {
        self.taffy.clear();
        self.root = Self::create_root(&mut self.taffy);
    }

    pub fn compute_layout(&mut self, width: f32, height: f32) {
        self.taffy
            .compute_layout(
                self.root,
                Size {
                    width: length(width),
                    height: length(height),
                },
            )
            .unwrap();
    }

    pub fn debug(&mut self) {
        self.taffy.print_tree(self.root);
    }

    fn create_root(taffy: &mut TaffyTree<()>) -> NodeId {
        let style = Style {
            size: Size {
                width: percent(1.0),
                height: percent(1.0),
            },
            ..Default::default()
        };

        // print!("\n root {style:?}");

        taffy.new_with_children(style, &[]).unwrap()
    }
}
