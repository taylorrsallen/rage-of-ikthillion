use crate::*;

//================================-================================-================================
pub struct GridItemPlugin;
impl Plugin for GridItemPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        
    }
}

//================================-================================-================================
#[derive(Component, Clone, Copy)]
pub struct GridItem {
    pub coord: IVec3,
    pub inventory_item_type: InventoryItemType,
}

impl GridItem {
    pub fn new(
        coord: &IVec3,
        inventory_item_type: InventoryItemType,
    ) -> Self {
        Self {
            coord: *coord,
            inventory_item_type,
        }
    }
}