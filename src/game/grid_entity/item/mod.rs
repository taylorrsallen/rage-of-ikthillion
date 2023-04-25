use crate::*;

mod grid_item;
mod inventory;
pub use grid_item::*;
pub use inventory::*;

//================================-================================-================================
pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugin(GridItemPlugin)
            .add_plugin(InventoryPlugin);
    }
}