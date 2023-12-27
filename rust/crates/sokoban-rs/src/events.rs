#[derive(Debug)]
pub enum EventKind {
    // Fired when the player hits an obstacle like a wall
    PlayerHitObstacle,

    // Fired when an entity is moved
    EntityMoved(EntityMoved),

    // Fired when the box is placed on a spot
    BoxPlacedOnSpot(BoxPlacedOnSpot),
}

pub type EntityId = u32;

#[derive(Debug)]
pub struct EntityMoved {
    pub id: EntityId,
}

#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool,
}
