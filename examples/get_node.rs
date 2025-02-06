use godot::classes::{ITileMapLayer, Label, Node, PackedScene, TileMapLayer};
use godot::obj::NewGd;
use godot::prelude::*;

/// A custom TileMapLayer that instantiates a `PackedScene` and modifies a `Label`.
#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct Tilesm {
    /// The underlying TileMapLayer.
    #[base]
    base: Base<TileMapLayer>,

    /// A `PackedScene` that we'll instantiate in code.
    #[export]
    plays: Gd<PackedScene>,
}

#[godot_api]
impl ITileMapLayer for Tilesm {
    /// Called when Godot instantiates this script.
    fn init(base: Base<TileMapLayer>) -> Self {
        Self {
            base,
            plays: PackedScene::new_gd(), // Default-empty Gd<PackedScene> for safety.
        }
    }

    /// Runs once this `TileMapLayer` is ready in the scene tree.
    fn ready(&mut self) {
        self.instantiate_and_configure_scene();
    }
}

#[godot_api]
impl Tilesm {
    /// Attempts to instantiate the `plays` scene, add it as a child, and configure a Label node.
    #[func]
    fn instantiate_and_configure_scene(&mut self) {
        // Safely attempt to instantiate the 'plays' scene
        let Some(plays_scene) = self.plays.to_option() else {
            godot_error!("Tilesm: 'plays' scene is not assigned. Aborting instantiation.");
            return;
        };

        let instance_result = plays_scene.instantiate();
        let Some(instance) = instance_result else {
            godot_error!("Tilesm: Failed to instantiate 'plays' PackedScene.");
            return;
        };

        // Add the newly created instance to 'self' (this TileMapLayer)
        self.base_mut().add_child(&instance);

        // Attempt to fetch the Label using a relative path or an absolute path:
        //   - If it's relative to the instance: "PLAYERS/Label"
        //   - If it's an absolute path in the scene tree: "/root/Tilesm/PLAYERS/Label"
        //   (Adjust based on how your scene is structured)
        let label_path = "PLAYERS/Label"; // or "/root/Tilesm/PLAYERS/Label"
        let label_node = instance
            .get_node_as::<Label>(label_path)
            .or_else(|| {
                godot_warn!("Tilesm: Could not find Label at path: {label_path}");
                None
            });

        // If found, set the text; if not, skip gracefully
        if let Some(label) = label_node {
            label.set_text("Hello from ptit (Rust)!");
        } else {
            godot_warn!("Tilesm: No Label found to update text.");
        }
    }
}
