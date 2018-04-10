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
    arbiter:Arbiter<WallBitMask>,
    bounding_box:BoundingBox,
    health:f32,
}

impl Wall {
    pub fn new(bounding_box:BoundingBox) -> Self {
        Wall {
            arbiter:Arbiter::new(),
            bounding_box,
            health:10.0
        }
    }
}

impl ResourceTrait for Wall {}

#[derive(Eq,PartialEq,Debug)]
pub struct WallBitMask {
    bits:u8
}

impl BitMask for WallBitMask {
    fn field_count() -> usize {
        2
    }
    fn zeroed() -> Self {
        WallBitMask {
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
        WallBitMask{bits:self.bits & other.bits}
    }
    fn or(&self, other:&Self) -> Self {
        WallBitMask{bits:self.bits | other.bits}
    }
    fn not(&self) -> Self {
        WallBitMask{bits:!self.bits}
    }
}

impl WallBitMask {
    pub fn new(bits:u8) -> Self {
        WallBitMask {
            bits
        }
    }
}


pub struct WallView {
    resource_reference:ResourceReference<Wall>,
    involved:WallBitMask,
    mode:WallBitMask,
    modifiers:Vec<*const Box<ModifierTrait>>,
    bounding_box:Option<*const FieldView<BoundingBox,WallView>>,
    health:Option<*const FieldView<f32,WallView>>,
}

impl ObjectViewTrait for WallView{
    fn add_modifier(&mut self, offset:usize, modifier_address:*const Box<ModifierTrait>) -> bool {
        let is_empty=self.modifiers.len()==0;
        self.modifiers.push(modifier_address);
        is_empty
    }

    fn release(&self,transaction:&TransactionInfo) {
        //println!("release wall");
        self.resource_reference.get_resource().arbiter.unlock(transaction)
    }
}

impl WallView {
    pub fn new(resource_reference:ResourceReference<Wall>) -> Self {
        WallView {
            resource_reference,
            involved:WallBitMask::zeroed(),
            mode:WallBitMask::zeroed(),
            modifiers:Vec::new(),
            bounding_box:None,
            health:None
        }
    }
}

pub fn get_wall_view1(resource_reference:&ResourceReference<Wall>, transaction:&Transaction) -> (&'static FieldView<BoundingBox,WallView>, &'static FieldView<f32,WallView>) {
    let object_view=match transaction.get_object_view(&resource_reference.get_address()) {
        Some(object_view) => object_view,
        None => {
            let object_view=Box::new(ObjectView::new(WallView::new(resource_reference.clone())));
            transaction.add_object_view(resource_reference.get_address(), object_view)
        }
    };

    let wall_view:&ObjectView<WallView>=unsafe {
        (&*object_view).downcast_ref_unchecked()
    };

    {
        let wall_view2 = wall_view.get_mut();

        let involved_o = WallBitMask::new(3);
        let mode_o = WallBitMask::new(3);

        let check_mask = wall_view2.involved.not().and(&involved_o).or(&wall_view2.involved.and(&mode_o).and(&wall_view2.mode.not()));
        let involved=involved_o.and(&check_mask);
        let mode=mode_o.and(&check_mask);

        //println!("{} {}",involved.bits,mode.bits);

        let access = Access {
            transaction: transaction.get_info(),
            priority: 10,
            involved,
            mode
        };

        resource_reference.get_resource().arbiter.lock(access);

        let value = unsafe { &resource_reference.get_resource().bounding_box as *const BoundingBox };
        let field_view = Box::new(FieldView::new(wall_view, 0, value, transaction));
        let field_view = transaction.add_field_view(field_view);

        let field_view: &FieldView<BoundingBox, WallView> = unsafe {
            (&*field_view).downcast_ref_unchecked()
        };

        wall_view2.bounding_box = Some(field_view);

        let value = unsafe { &resource_reference.get_resource().health as *const f32 };
        let field_view2 = Box::new(FieldView::new(wall_view, 0, value, transaction));
        let field_view2 = transaction.add_field_view(field_view2);

        let field_view2: &FieldView<f32, WallView> = unsafe {
            (&*field_view2).downcast_ref_unchecked()
        };

        wall_view2.health = Some(field_view2);


        wall_view2.involved=wall_view2.involved.or(&involved_o);
        wall_view2.mode=wall_view2.mode.or(&mode_o);

        (&*field_view, &*field_view2)
    }
}

pub struct SetHealthModifier {
    pub health:f32
}

impl ModifierTrait for SetHealthModifier {
    fn apply(self) {}
}

