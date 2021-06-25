use crate::components::{Inventory, Player, SelectedIsland};
use crate::world::GameWorld;

// Returns the player inventory, assumes there is one and just one.
pub fn get_player_inventory(world: &GameWorld) -> Inventory {
    if world.ecs.query::<(&Inventory, &Player)>().iter().count() > 1 {
        panic!("There are multiple players!")
    }

    #[allow(clippy::never_loop)]
    for (_id, (inv, _player)) in world.ecs.query::<(&Inventory, &Player)>().iter() {
        return inv.clone();
    }

    panic!("Player does not exist!")
}

// Returns the inventory of the currently selected island.
pub fn get_island_inventory(world: &GameWorld) -> Inventory {
    if world.ecs.query::<(&Inventory, &SelectedIsland)>().iter().count() > 1 {
        panic!("There are multiple selected islands!")
    }

    #[allow(clippy::never_loop)]
    for (_id, (inv, _player)) in world.ecs.query::<(&Inventory, &SelectedIsland)>().iter() {
        return inv.clone();
    }

    panic!("Selected island does not exist!")
}
