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

use super::building::Building;

pub struct World {
    arbiter:Arbiter<WorldBitMask>,
    buildings:Vec<ResourceReference<Building>>
}

impl Drop for World {
    fn drop(&mut self) {
        //println!("Drop world");
    }
}

impl ResourceTrait for World {}

impl World {
    pub fn new(buildings:Vec<ResourceReference<Building>>) -> Self {
        World {
            arbiter:Arbiter::new(),
            buildings
        }
    }
}

#[derive(Eq,PartialEq,Debug)]
pub struct WorldBitMask {
    bits:u8
}

impl BitMask for WorldBitMask {
    fn field_count() -> usize {
        2
    }
    fn zeroed() -> Self {
        WorldBitMask {
            bits:0
        }
    }
    fn get(&self, index:usize) -> bool {
        self.bits & 1<<index != 0
    }
    fn set(&mut self, index:usize) {
        self.bits |= 1<<index;
    }
    fn clear(&mut self, index:usize) {
        self.bits &= !(1<<index);
    }
    fn and(&self, other: &Self) -> Self {
        WorldBitMask{bits:self.bits & other.bits}
    }
    fn or(&self, other:&Self) -> Self {
        WorldBitMask{bits:self.bits | other.bits}
    }
    fn not(&self) -> Self {
        WorldBitMask{bits:!self.bits}
    }
}

impl WorldBitMask {
    pub fn new(bits:u8) -> Self {
        WorldBitMask {
            bits
        }
    }
}


pub struct WorldView {
    resource_reference:ResourceReference<World>,
    involved:WorldBitMask,
    mode:WorldBitMask,
    modifiers:Vec<*const Box<ModifierTrait>>,
    buildings:Option<*const FieldView<Vec<ResourceReference<Building>>,WorldView>>,
}

impl ObjectViewTrait for WorldView{
    fn add_modifier(&mut self, offset:usize, modifier_address:*const Box<ModifierTrait>) -> bool {
        let is_empty=self.modifiers.len()==0;
        self.modifiers.push(modifier_address);
        is_empty
    }

    fn release(&self,transaction:&TransactionInfo) {
        println!("release world");
        self.resource_reference.get_resource().arbiter.unlock(transaction)
    }
}

impl WorldView {
    pub fn new(resource_reference:ResourceReference<World>) -> Self {
        WorldView {
            resource_reference,
            involved:WorldBitMask::zeroed(),
            mode:WorldBitMask::zeroed(),
            modifiers:Vec::new(),
            buildings:None
        }
    }
}

pub fn get_world_view1(resource_reference:&ResourceReference<World>, transaction:&Transaction) -> &'static FieldView<Vec<ResourceReference<Building>>,WorldView> {
    let object_view=match transaction.get_object_view(&resource_reference.get_address()) {
        Some(object_view) => object_view,
        None => {
            let object_view=Box::new(ObjectView::new(WorldView::new(resource_reference.clone())));
            transaction.add_object_view(resource_reference.get_address(), object_view)
        }
    };

    let world_view:&ObjectView<WorldView>=unsafe {
        (&*object_view).downcast_ref_unchecked()
    };

    {
        let world_view2 = world_view.get_mut();

        let involved_o = WorldBitMask::new(1);
        let mode_o = WorldBitMask::new(0);

        let check_mask = world_view2.involved.not().and(&involved_o).or(&world_view2.involved.and(&mode_o).and(&world_view2.mode.not()));
        let involved=involved_o.and(&check_mask);
        let mode=mode_o.and(&check_mask);

        println!("{} {}",involved.bits,mode.bits);

        let access = Access {
            transaction: transaction.get_info(),
            priority: 10,
            involved,
            mode
        };

        resource_reference.get_resource().arbiter.lock(access);

        let value = unsafe { &resource_reference.get_resource().buildings as *const Vec<ResourceReference<Building>> };
        let field_view = Box::new(FieldView::new(world_view, 0, value, transaction));
        let field_view = transaction.add_field_view(field_view);

        let field_view: &FieldView<Vec<ResourceReference<Building>>, WorldView> = unsafe {
            (&*field_view).downcast_ref_unchecked()
        };

        world_view2.buildings = Some(field_view);
        world_view2.involved=world_view2.involved.or(&involved_o);
        world_view2.mode=world_view2.mode.or(&mode_o);

        &*field_view
    }
}

