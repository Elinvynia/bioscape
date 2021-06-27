// Anything that can be hovered over.
#[derive(Debug)]
pub struct Hoverable;

// The player, only one should exist at a time.
#[derive(Debug)]
pub struct Player;

// An island tile.
#[derive(Debug)]
pub struct Island;

// The currently selected island, only one should exist at a time.
#[derive(Debug)]
pub struct SelectedIsland;

// A sea tile.
#[derive(Debug)]
pub struct Sea;
