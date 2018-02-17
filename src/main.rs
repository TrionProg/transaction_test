

extern crate generic_array;

pub mod server;
//pub use server::{set_server_id, get_server_id};

pub mod process;

pub mod resource;

pub mod local_storage;

pub mod arbiter;
//pub mod arbiter_wp;
pub mod arbiter_rp;

pub mod access;

pub mod transaction;
/*
pub use transaction::{Transaction, AccessTrait};
*/

pub mod bitset;

pub mod prelude;

/*
const CAMERA_FIELDS_NUM:usize=3;
//TODO Camera all affected

#[derive(Eq,PartialEq,Clone,Debug)]
pub struct ProcessID {
    server:u32,
    process:u32,
}

pub struct CameraGuard {
    states:[u16;CAMERA_FIELDS_NUM],
    waiting_processes:Vec<CameraAccessProcess>,
    locking_processes:Vec<CameraAccessProcess>
}

impl CameraGuard {
    fn new() -> Self {
        CameraGuard {
            states:[1;3],
            waiting_processes:Vec::with_capacity(4),
            locking_processes:Vec::with_capacity(2),
        }
    }

    fn try_lock(&self, access_mode:&CameraAccessMode) -> bool {
        for i in 0..CAMERA_FIELDS_NUM {
            if (access_mode.affect & 1<<i) > 0 {
                if (access_mode.mode & 1<<i) > 0 {//write
                    if self.states[i] > 1 {//read
                        return false;
                    }else if self.states[i]==0 {//write
                        return false;
                    }
                }else{//read
                    if self.states[i] > 1 {//write
                        return false;
                    }
                }
            }
        }

        true
    }

    fn lock(&mut self, access_process:CameraAccessProcess){
        for i in 0..CAMERA_FIELDS_NUM {
            if (access_process.access_mode.affect & 1<<i) > 0 {
                if (access_process.access_mode.mode & 1<<i) > 0 {//write
                    self.states[i]=0;
                }else{//read
                    self.states[i] += 1;
                }
            }
        }

        self.locking_processes.push(access_process);
    }

    fn unlock(&mut self, process_id:ProcessID) -> Option<ProcessID> {
        {
            let states=&mut self.states;
            self.locking_processes.retain(|access_process|{
                if access_process.process_id == process_id{
                    for i in 0..CAMERA_FIELDS_NUM {
                        if (access_process.access_mode.affect & 1<<i) > 0 {
                            if (access_process.access_mode.mode & 1<<i) > 0 {//write
                                states[i]=1;
                            }else{//read
                                states[i] -= 1;
                            }
                        }
                    }

                    false
                }else{
                    true
                }
            });
        }

        match self.waiting_processes.pop() {
            Some(access_process) => {
                if self.try_lock(&access_process.access_mode) {
                    let process_id=access_process.process_id.clone();
                    self.lock(access_process);
                    Some(process_id)
                }else{
                    self.waiting_processes.push(access_process);
                    None
                }
            },
            None => None
        }
    }
}

pub struct OutCameraGuard(Mutex<CameraGuard>);

impl OutCameraGuard {
    fn new() -> Self {
        OutCameraGuard(Mutex::new(CameraGuard::new()))
    }

    fn lock(&self, access_process:CameraAccessProcess) -> bool {
        let mut camera_guard=match self.0.lock() {
            Ok(camera_guard) => camera_guard,
            Err(_) => unimplemented!()
        };

        if camera_guard.try_lock(&access_process.access_mode) {
            camera_guard.lock(access_process);
            true
        }else{
            camera_guard.waiting_processes.push(access_process);
            false
        }
    }

    fn unlock(&self, process_id:ProcessID) -> Option<ProcessID> {
        let mut camera_guard=match self.0.lock() {
            Ok(camera_guard) => camera_guard,
            Err(_) => unimplemented!()
        };

        camera_guard.unlock(process_id)
    }
}

pub struct CameraAccessMode {
    affect:u8,
    mode:u8,
}

pub struct CameraAccessProcess {
    process_id:ProcessID,
    priority:u8,
    access_mode:CameraAccessMode
}
*/

use prelude::*;

struct Camera {
    a:f32,
    b:f32,
    c:f32
}

impl RT for Camera{}

fn foo() {
    let rr:RR<Camera>=RR::new(RA::new(2));

    let rr2=rr.clone();
    let rr3=rr.clone();
}

fn main() {
    use server::set_server_id;
    use local_storage::{create_local_storage,delete_local_storage};

    set_server_id(1);
    create_local_storage();

    foo();

    delete_local_storage();

    /*
    let mut camera_guard=OutCameraGuard::new();

    let access=CameraAccessProcess {
        process_id:ProcessID{server:32,process:1},
        priority:3,
        access_mode:CameraAccessMode {
            affect:0b101,
            mode:0b001
        }
    };

    println!("{}", camera_guard.lock(access));

    let access=CameraAccessProcess {
        process_id:ProcessID{server:32,process:3},
        priority:5,
        access_mode:CameraAccessMode {
            affect:0b10,
            mode:0b10
        }
    };

    println!("{}", camera_guard.lock(access));

    let access=CameraAccessProcess {
        process_id:ProcessID{server:32,process:5},
        priority:5,
        access_mode:CameraAccessMode {
            affect:0b10,
            mode:0b10
        }
    };

    println!("{}", camera_guard.lock(access));

    println!("{:?}", camera_guard.unlock(ProcessID{server:32,process:3}));
    */

}
