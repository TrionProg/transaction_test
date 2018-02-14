
use server::ServerID;
use server::get_server_id;

pub type LocalProcessID=u32;

#[derive(Eq,PartialEq,Clone,Debug)]
pub struct ProcessID {
    server_id:ServerID,
    local_process_id:LocalProcessID,
}

impl ProcessID {
    fn new(local_process_id:LocalProcessID) -> Self {
        ProcessID {
            server_id:get_server_id(),
            local_process_id
        }
    }
}