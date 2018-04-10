
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

use super::array::Array;

use super::wall::Wall;

pub struct Building {
    arbiter:Arbiter<BuildingBitMask>,
    bounding_box:BoundingBox,
    walls:Vec<ResourceReference<Wall>>
}

impl ResourceTrait for Building {}

impl Building {
    pub fn new(bounding_box:BoundingBox, walls:Vec<ResourceReference<Wall>>) -> Self {
        Building{
            arbiter:Arbiter::new(),
            bounding_box,
            walls
        }
    }
}

impl Drop for Building {
    fn drop(&mut self) {
        //println!("Drop building");
    }
}

#[derive(Eq,PartialEq,Debug)]
pub struct BuildingBitMask {
    bits:u8
}

impl BitMask for BuildingBitMask {
    fn field_count() -> usize {
        2
    }
    fn zeroed() -> Self {
        BuildingBitMask {
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
        BuildingBitMask{bits:self.bits & other.bits}
    }
    fn or(&self, other:&Self) -> Self {
        BuildingBitMask{bits:self.bits | other.bits}
    }
    fn not(&self) -> Self {
        BuildingBitMask{bits:!self.bits}
    }
}

impl BuildingBitMask {
    pub fn new(bits:u8) -> Self {
        BuildingBitMask {
            bits
        }
    }
}

pub struct BuildingView {
    resource_reference:ResourceReference<Building>,
    involved:BuildingBitMask,
    mode:BuildingBitMask,
    modifiers:Vec<*const Box<ModifierTrait>>,
    bounding_box:Option<*const FieldView<BoundingBox,BuildingView>>,
    walls:Option<*const FieldView<Vec<ResourceReference<Wall>>,BuildingView>>
}

impl ObjectViewTrait for BuildingView{
    fn add_modifier(&mut self, offset:usize, modifier_address:*const Box<ModifierTrait>) -> bool {
        let is_empty=self.modifiers.len()==0;
        self.modifiers.push(modifier_address);
        is_empty
    }

    fn release(&self,transaction:&TransactionInfo) {
        //println!("release building");
        self.resource_reference.get_resource().arbiter.unlock(transaction)
    }
}

impl BuildingView {
    pub fn new(resource_reference:ResourceReference<Building>) -> Self {
        BuildingView {
            resource_reference,
            involved:BuildingBitMask::zeroed(),
            mode:BuildingBitMask::zeroed(),
            modifiers:Vec::new(),
            bounding_box:None,
            walls:None
        }
    }
}

pub fn get_building_view1(resource_reference:&ResourceReference<Building>, transaction:&Transaction) -> &'static FieldView<BoundingBox,BuildingView> {
    let object_view=match transaction.get_object_view(&resource_reference.get_address()) {
        Some(object_view) => object_view,
        None => {
            let object_view=Box::new(ObjectView::new(BuildingView::new(resource_reference.clone())));
            transaction.add_object_view(resource_reference.get_address(), object_view)
        }
    };

    let building_view:&ObjectView<BuildingView>=unsafe {
        (&*object_view).downcast_ref_unchecked()
    };

    {
        let building_view2 = building_view.get_mut();

        let involved_o = BuildingBitMask::new(1);
        let mode_o = BuildingBitMask::new(0);

        let check_mask = building_view2.involved.not().and(&involved_o).or(&building_view2.involved.and(&mode_o).and(&building_view2.mode.not()));
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
        let field_view = Box::new(FieldView::new(building_view, 0, value, transaction));
        let field_view = transaction.add_field_view(field_view);

        let field_view: &FieldView<BoundingBox, BuildingView> = unsafe {
            (&*field_view).downcast_ref_unchecked()
        };

        building_view2.bounding_box = Some(field_view);
        building_view2.involved=building_view2.involved.or(&involved_o);
        building_view2.mode=building_view2.mode.or(&mode_o);

        &*field_view
    }
}

pub fn get_building_view2(resource_reference:&ResourceReference<Building>, transaction:&Transaction) -> &'static FieldView<BoundingBox,BuildingView> {
    let object_view=match transaction.get_object_view(&resource_reference.get_address()) {
        Some(object_view) => object_view,
        None => {
            let object_view=Box::new(ObjectView::new(BuildingView::new(resource_reference.clone())));
            transaction.add_object_view(resource_reference.get_address(), object_view)
        }
    };

    let building_view:&ObjectView<BuildingView>=unsafe {
        (&*object_view).downcast_ref_unchecked()
    };

    {
        let building_view2 = building_view.get_mut();

        let involved_o = BuildingBitMask::new(1);
        let mode_o = BuildingBitMask::new(1);

        let check_mask = building_view2.involved.not().and(&involved_o).or(&building_view2.involved.and(&mode_o).and(&building_view2.mode.not()));
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
        let field_view = Box::new(FieldView::new(building_view, 0, value, transaction));
        let field_view = transaction.add_field_view(field_view);

        let field_view: &FieldView<BoundingBox, BuildingView> = unsafe {
            (&*field_view).downcast_ref_unchecked()
        };

        building_view2.bounding_box = Some(field_view);
        building_view2.involved=building_view2.involved.or(&involved_o);
        building_view2.mode=building_view2.mode.or(&mode_o);

        &*field_view
    }
}

pub fn get_building_view3(resource_reference:&ResourceReference<Building>, transaction:&Transaction) -> &'static FieldView<Vec<ResourceReference<Wall>>,BuildingView> {
    let object_view=match transaction.get_object_view(&resource_reference.get_address()) {
        Some(object_view) => object_view,
        None => {
            let object_view=Box::new(ObjectView::new(BuildingView::new(resource_reference.clone())));
            transaction.add_object_view(resource_reference.get_address(), object_view)
        }
    };

    let building_view:&ObjectView<BuildingView>=unsafe {
        (&*object_view).downcast_ref_unchecked()
    };

    {
        let building_view2 = building_view.get_mut();

        let involved_o = BuildingBitMask::new(2);
        let mode_o = BuildingBitMask::new(2);

        let check_mask = building_view2.involved.not().and(&involved_o).or(&building_view2.involved.and(&mode_o).and(&building_view2.mode.not()));
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

        let value = unsafe { &resource_reference.get_resource().walls as *const Vec<ResourceReference<Wall>> };
        let field_view = Box::new(FieldView::new(building_view, 0, value, transaction));
        let field_view = transaction.add_field_view(field_view);

        let field_view: &FieldView<Vec<ResourceReference<Wall>>, BuildingView> = unsafe {
            (&*field_view).downcast_ref_unchecked()
        };

        building_view2.walls = Some(field_view);
        building_view2.involved=building_view2.involved.or(&involved_o);
        building_view2.mode=building_view2.mode.or(&mode_o);

        &*field_view
    }
}

/*
pub fn get_view1(resource_reference:&ResourceReference<Building>, transaction:&Transaction) -> &FieldView<BoundingBox,BuildingView> {
    let bv=BuildingView::new(resource_reference.clone());
}
*/

/*
pub struct BuildingAddWallModifier {
    building:ResourceReference<Building>,
    wall:ResourceReference<Wall>
}

impl ModifierTrait for BuildingAddWallModifier {
    fn apply(self) {
        let walls=unsafe{&mut *((&self.building.get_resource().walls as *const Vec<ResourceReference<Wall>>) as *mut Vec<ResourceReference<Wall>>)};
        walls.push(self.wall);
    }
}

pub fn get_view1(resource_reference:&ResourceReference<Building>, transaction:&Transaction) -> &'static FieldView<BoundingBox,BuildingView> {
    let bv=BuildingView::new(resource_reference.clone());
}
*/

/*

pub struct BuildingView1 {
    pub bounding_box:&'static BoundingBox,
}

impl BuildingView1 {//TODO extern crate>
    pub fn get(resource_reference:&ResourceReference<Building>, transaction:&mut Transaction) -> BuildingView1 {
        let mut involved=BuildingBitMask::zeroed();
        involved.set(1);

        let mut mode=BuildingBitMask::zeroed();
        mode.set(1);

        let access=Access {
            transaction:transaction.get_info(),
            priority:10,
            involved,
            mode
        };

        resource_reference.get_resource().arbiter.lock(access);

        BuildingView1 {
            bounding_box:unsafe{&*(&resource_reference.get_resource().bounding_box as *const BoundingBox)}
        }
    }
}


pub struct BuildingView2 {
    pub walls:&'static Vec<ResourceReference<Wall>>
}

impl BuildingView2 {//TODO extern crate>
    pub fn get(resource_reference:&ResourceReference<Building>, transaction:&mut Transaction) -> BuildingView2 {
        let mut involved=BuildingBitMask::zeroed();
        involved.set(0);

        let mut mode=BuildingBitMask::zeroed();

        let access=Access {
            transaction:transaction.get_info(),
            priority:11,
            involved,
            mode
        };

        resource_reference.get_resource().arbiter.lock(access);

        BuildingView2 {
            walls:unsafe{&*(&resource_reference.get_resource().walls as *const Vec<ResourceReference<Wall>>)}
        }
    }
}
*/