use crate::*;

//================================-================================-================================
pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        
    }
}

//================================-================================-================================
#[derive(Component, Clone, Copy)]
pub struct InventoryItem {
    pub origin: IVec2,
    pub dim: IVec2,
    pub inventory_item_type: InventoryItemType,
}

impl Default for InventoryItem {
    fn default() -> Self {
        Self {
            origin: IVec2::default(),
            dim: IVec2::default(),
            inventory_item_type: InventoryItemType::Ramen,
        }
    }
}

impl InventoryItem {
    pub fn new(
        origin: &IVec2,
        dim: &IVec2,
        inventory_item_type: InventoryItemType,
    ) -> Self {
        Self {
            origin: *origin,
            dim: *dim,
            inventory_item_type,
        }
    }

    pub fn get_def(
        &self,
    ) -> &'static InventoryItemTypeDef {
        &INVENTORY_ITEM_TYPE_DEFS[self.inventory_item_type as usize]
    }
}

//================================-================================-================================
#[derive(Component, Clone)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
    pub dim: IVec2,
    pub active_mask: Bitmask,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: vec![InventoryItem::default()],
            dim: IVec2::new(1, 1),
            active_mask: Bitmask::from_dim2d(&IVec2::new(1, 1), false),
        }
    }
}

impl Inventory {
    pub fn new(
        dim: &IVec2,
    ) -> Self {
        Self {
            items: vec![InventoryItem::default(); (dim.x * dim.y) as usize],
            dim: *dim,
            active_mask: Bitmask::from_dim2d(&dim, false),
            ..default()
        }
    }

    pub fn index_from_coord(
        &self,
        coord: &IVec2,
    ) -> usize {
        (coord.x + coord.y * self.dim.x) as usize
    }

    pub fn is_item_placeable(
        &self,
        item: &InventoryItem,
    ) -> bool {
        let mut placeable = true;

        for y in 0..item.dim.y { for x in 0..item.dim.x {
            if self.active_mask.is_bit_on(self.index_from_coord(&(IVec2::new(x, y) + item.origin))) {
                placeable = false;
            }
        }}

        placeable
    }

    pub fn place_item(
        &mut self,
        item: &InventoryItem,
    ) {
        for y in 0..item.dim.y { for x in 0..item.dim.x {
            let index = self.index_from_coord(&(IVec2::new(x, y) + item.origin));
            self.items[index] = *item;
            self.active_mask.set_bit_on(index);
        }}
    }

    pub fn add_inventory_item(
        &mut self,
        item: &InventoryItem,
    ) {
        if self.is_item_placeable(item) {
            self.place_item(item);
        } else {
            println!("No room for item!");
        }
    }
}

//================================-================================-================================
#[derive(Default, Component)]
pub struct InventoryViewer {
    pub inventories: Vec<Entity>,
}