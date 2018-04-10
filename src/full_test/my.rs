
use std::thread;

use my;
use my::master_storage::{create_master_storage,get_master_storage,delete_master_storage};
use my::resource::ResourceReference;
use my::building::{Building};
use my::transaction::Transaction;

use my::building::get_view1;

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

    let view=get_view1(&building,transaction);

    println!("{:?}",view.get());

    building
}

fn check_building(building:ResourceReference<Building>, i:usize) {
    let transaction=Transaction::new(TransactionInfo::new(1,i as u32));

    check_building_transaction(&transaction, building)
}

fn check_building_transaction(transaction:&Transaction, building:ResourceReference<Building>) {
    let view=get_view1(&building,transaction);

    thread::sleep_ms(100);

    println!("{:?}",view.get());
}

pub fn test(thread_count:usize) {
    create_master_storage();

    let building=create_world();

    let mut threads1=Vec::with_capacity(thread_count);

    for i in 0..thread_count {
        let building2=building.clone();

        let jh=thread::spawn(move || {
            check_building(building2, i);
        });

        threads1.push(jh);
    }

    for jh in threads1 {
        jh.join();
    }

    delete_master_storage();
}