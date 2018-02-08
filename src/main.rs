
pub struct Transaction {
    accesses:Vec<Box<AccessTrait>>,//Box:они могут лежать просто в стеке с транзакцией
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            accesses:Vec::with_capacity(4),
        }
    }

    pub fn insert_access(&mut self, access:Box<AccessTrait>) {
        self.accesses.push(access);
    }

    pub fn finish(&mut self) {//а если серверы сломались?
        //надо сделать в 2 этапа:
        //1) отправляем всем и получаем ответы
        //2) подтверждаем
    }


}

//Drop for transaction

pub trait AccessTrait {
    //fn write(self);
}

pub struct Camera {
    name:String,
    x:f32,
    y:f32,
}

pub trait Instance:Sized {
    fn set_pos(self_ref:SDR<Self>, transaction:&mut Transaction, x:f32, y:f32);
}

impl Instance for Camera {
    fn set_pos(self_ref:SDR<Camera>, transaction:&mut Transaction, x:f32, y:f32){
        pub struct AccessCamera1{
            data:Option<Box<AccessCameraData1>>,
            data_ref:SDR<Camera>,
            modifiers:Vec<AccessCameraModifier1>,
        }

        impl AccessTrait for AccessCamera1 {

        }

        pub struct AccessCameraData1{
            name:String,
            x:f32,
            y:f32,
        }

        pub struct AccessCameraView1<'a> {
            access:&'a mut AccessCamera1,
            name:&'a String,
            x:&'a mut f32,
            y:&'a mut f32,
        }

        impl<'a> AccessCameraView1<'a> {
            fn get(data_ref:&SDR<Camera>, transaction:&mut Transaction) -> Self {
                unsafe {
                //if data_ref.local_ref!=0 {
                    let data=data_ref.local_ref;

                    let access=AccessCamera1 {
                        data:None,
                        data_ref:data_ref.clone(),
                        modifiers:Vec::with_capacity(2)
                    };

                    let access=Box::new(access);
                    let access_ref=&mut *((access.as_ref() as *const AccessCamera1) as *mut AccessCamera1);

                    transaction.insert_access(access);

                    //insert access

                    let view=AccessCameraView1 {
                        access:access_ref,
                        name:&*(&(*data).name as *const String),
                        x:&mut *((&(*data).x as *const f32) as *mut f32),
                        y:&mut *((&(*data).y as *const f32) as *mut f32)
                    };

                    view
                }
            }
        }

        pub enum AccessCameraModifier1 {
            V1(String),
            V2(f32,f32)
        }

        let view=AccessCameraView1::get(&self_ref, transaction);

        println!("{}", view.name);
    }
}

///SharedDataAddress
pub struct SDA {
    server:u32,
    data:u64,
}

//если сервер этот же, то значит, что данные хранятся на этом сервере(будь Storage или RAM)

impl Clone for SDA {
    fn clone(&self) -> Self {
        SDA{
            server:self.server,
            data:self.data
        }
    }
}

///SharedDataRef
pub struct SDR<T> {
    local_ref:*const T,//если данные есть на этом сервере(кэш)
    address:SDA
}

impl<T> Clone for SDR<T> {
    fn clone(&self) -> Self {
        SDR{
            local_ref:self.local_ref,
            address:self.address.clone()
        }
    }
}

//Transaction
fn do_transaction1(camera_ref:SDR<Camera>) {
    let mut transaction=Transaction::new();

    <Camera as Instance>::set_pos(camera_ref, &mut transaction, 0.4, 0.8);
}

fn main() {
    let camera=Camera {
        name:"cam1".to_string(),
        x:0.3,
        y:0.6,
    };

    let camera_ref=unsafe{SDR {
        local_ref:&camera as *const Camera,
        address:SDA{server:0,data:0}
    }};

    //Transaction
    do_transaction1(camera_ref.clone());
}
