
pub type ServerID=u32;

static mut SERVER_ID:ServerID=0;

pub fn set_server_id(server_id:ServerID) {
    unsafe{
        SERVER_ID=server_id;
    }
}

pub fn get_server_id() -> ServerID {
    unsafe{
        SERVER_ID
    }
}