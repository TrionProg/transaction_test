
use std::thread;

use my;
use my::master_storage::{create_master_storage,get_master_storage,delete_master_storage};
use my::resource::ResourceReference;
use my::building::{Building};
use my::wall::Wall;
use my::world::World;
use my::transaction::Transaction;

use my::building::{get_building_view1, get_building_view2, get_building_view3};
use my::world::get_world_view1;

use common::TransactionInfo;
use common::{BoundingBox,Point};

fn create_world() -> ResourceReference<World> {
    let transaction=Transaction::new(TransactionInfo::new(1,1));

    create_world_transaction(&transaction)
}

fn create_world_transaction(transaction:&Transaction) -> ResourceReference<World>{
    let mut buildings=Vec::with_capacity(20);

    for i in 0..20 {
        let building=create_building(transaction, i%2==0);
        //println!("Building:{:?}",building.get_address());
        buildings.push(building);
    }


    let world=get_master_storage().insert_resource(World::new(buildings));

    println!("World:{:?}",world.get_address());
    world
}

fn create_building(transaction:&Transaction, a:bool) -> ResourceReference<Building> {
    let mut walls=Vec::with_capacity(30);

    for i in 0..30 {
        let wall=Wall::new(BoundingBox::new(Point::new(-i as f32, -i as f32), Point::new(i as f32, i as f32)));
        let wall=get_master_storage().insert_resource(wall);
        //println!("Wall:{:?}",wall.get_address());
        walls.push(wall);
    }

    let bounding_box=if a {
        BoundingBox::new(Point::new(-30.0, -30.0), Point::new(30.0, 30.0))
    }else{
        BoundingBox::new(Point::new(-3.0, -3.0), Point::new(3.0, 3.0))
    };

    get_master_storage().insert_resource(Building::new(bounding_box, walls))
}

fn check_buildings(world:ResourceReference<World>, i:usize) -> Vec<ResourceReference<Building>>  {
    let transaction=Transaction::new(TransactionInfo::new(1,i as u32));

    check_buildings_transaction(&transaction, world)
}

fn check_buildings_transaction(transaction:&Transaction, world:ResourceReference<World>) -> Vec<ResourceReference<Building>> {
    let buildings=get_world_view1(&world, transaction);

    let mut buildings1=Vec::new();

    for building in buildings.get().iter() {
        let bounding_box=get_building_view1(building,transaction);

        if bounding_box.get().collide(Point::new(5.0,5.0)) {
            thread::sleep_ms(100);
            buildings1.push(building.clone());
        }
    }

    println!("yeah {}", buildings1.len());

    let mut buildings2=Vec::new();

    use rand::{Rng, thread_rng};
    let mut rng = thread_rng();

    while buildings1.len()>0 {
        let a:usize=rng.gen();
        let index=a%buildings1.len();

        buildings2.push(buildings1.remove(index))
    }

    buildings2
}

fn process_buildings(buildings:Vec<ResourceReference<Building>>, i:usize) {
    let transaction=Transaction::new(TransactionInfo::new(1,i as u32));

    process_buildings_transaction(&transaction, buildings)
}

fn process_buildings_transaction(transaction:&Transaction, buildings:Vec<ResourceReference<Building>>) {
    for building in buildings.iter() {
        if process_building(transaction, building) {
            //TODO
        }
    }
}

fn process_building(transaction:&Transaction, building:&ResourceReference<Building>) -> bool {
    let walls=get_building_view3(building,transaction);

    for wall in walls.get().iter() {
        //if wall.
        thread::sleep_ms(100);
    }

    false
}

pub fn test(thread_count:usize) {
    create_master_storage();

    let world=create_world();

    let mut threads1=Vec::with_capacity(thread_count);

    for i in 0..thread_count {
        let world2=world.clone();

        let jh=thread::spawn(move || {
            let buildings=check_buildings(world2, i);
            //process_buildings(buildings,i);
        });

        threads1.push(jh);
    }

    for jh in threads1 {
        jh.join();
    }

    delete_master_storage();
}