
use common::BoundingBox;

use super::building::Building;

pub struct Chunk {
    bounding_box:BoundingBox,
    buildings:Vec<Building>
}