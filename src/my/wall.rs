use core::ptr;
use std::ops::BitAnd;

use common::BoundingBox;
use common::TransactionInfo;

use super::resource::ResourceTrait;
use super::resource::{ResourceAddress,ResourceReference};
use super::transaction::ModifierTrait;
use super::transaction::Transaction;

use super::access::BitMask;
use super::arbiter::Arbiter;
use super::access::Access;
use super::view::{ObjectViewTrait, ObjectView, FieldView};

pub struct Wall {
    bounding_box:BoundingBox,
    health:f32,
}

impl Wall {
    pub fn new(bounding_box:BoundingBox) -> Self {
        Wall {
            bounding_box,
            health:10.0
        }
    }
}

impl ResourceTrait for Wall {}