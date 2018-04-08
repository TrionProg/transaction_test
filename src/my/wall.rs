
use common::BoundingBox;

use super::resource::ResourceTrait;

pub struct Wall {
    bounding_box:BoundingBox
}

impl Wall {
    pub fn new(bounding_box:BoundingBox) -> Self {
        Wall {
            bounding_box
        }
    }
}

impl ResourceTrait for Wall {}