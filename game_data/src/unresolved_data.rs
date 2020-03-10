/// A ship
#[derive(Debug, Clone)]
pub struct Ship {
    /// it's name
    pub name: String,
    /// it's sprite
    pub sprite: String,
    /// it's outfits
    pub outfits: Vec<String>,
    /// it's drag
    pub drag: f32,
    /// it's mass
    pub mass: u32,
}
