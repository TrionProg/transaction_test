
use my;
use my::master_storage::{create_master_storage,get_master_storage,delete_master_storage};
use my::resource::ResourceReference;
use my::building::{Building};
use my::transaction::Transaction;

use common::TransactionInfo;
use common::{BoundingBox,Point};

fn create_world() -> ResourceReference<Building> {
    let transaction=Transaction::new(TransactionInfo::new(1,1));

    create_world_transaction(&transaction)
}

fn create_world_transaction(transaction:&Transaction) -> ResourceReference<Building>{
    let bounding_box=BoundingBox::new(Point::new(-5.0,-5.0), Point::new(5.0,5.0));
    let building=get_master_storage().insert_resource(Building::new(bounding_box));

    //let building_view2=BuildingView2::get(&building, transaction);

    //println!("{}",building_view2.walls.len());

    building
}

pub fn test(thread_count:usize) {
    create_master_storage();

    create_world();

    delete_master_storage();
}