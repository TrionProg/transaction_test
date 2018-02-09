
pub struct Transaction {
    accesses:Vec<Box<AccessTrait>>,//Box:они могут лежать просто в стеке с транзакцией
    //mod_accesses:Vec<*mut AccessTrait> //TODO raw::TraitObject
    mod_accesses:Vec<u32>,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            accesses:Vec::with_capacity(4),
            mod_accesses:Vec::with_capacity(4)
        }
    }

    pub fn insert_access(&mut self, mut access:Box<AccessTrait>) {
        let index=self.accesses.len() as u32;
        access.set_index(index);
        self.accesses.push(access);
    }

    pub fn add_mod_access(&mut self, access_index:u32) {
        self.mod_accesses.push(access_index);
    }

    pub fn finish(&mut self) {//а если серверы сломались?
        /*
        for mod_access in self.mod_accesses.iter() {
            //let mod_access=unsafe{ &mut *(mod_access as *mut AccessTrait) };
            //(*mod_access).write_local();
            mod_access.write_local();
        }
        */

        for mod_access_index in self.mod_accesses.iter() {
            self.accesses[*mod_access_index as usize].write_local();
        }

        //while let Some(modifier) = self.m
        //TODO а если серверы сломались?
        // надо сделать в 2 этапа:
        //1) отправляем всем и получаем ответы
        //2) подтверждаем
    }

}

//Drop for transaction

pub trait AccessTrait {
    fn set_index(&mut self, index:u32);
    //fn is_local(cash)
    //fn is_remote
    fn write_local(&mut self);
}

pub struct Camera {
    name:String,
    x:f32,
    y:f32,
}

pub trait Instance:Sized {
    fn set_pos(self_ref:SDR<Self>, transaction:&mut Transaction, x:f32, y:f32);
    fn get_pos(self_ref:SDR<Self>, transaction:&mut Transaction);
}

impl Instance for Camera {
    fn set_pos(self_ref:SDR<Camera>, transaction:&mut Transaction, x:f32, y:f32){
        use std::collections::VecDeque;

        pub struct AccessCamera1{
            data:Option<Box<AccessCameraData1>>,
            data_ref:SDR<Camera>,
            modifiers:VecDeque<AccessCameraModifier1>,
            index:u32,
        }

        impl AccessTrait for AccessCamera1 {
            fn set_index(&mut self, index:u32) {
                self.index=index;
            }

            fn write_local(&mut self) {
                let local_ref=unsafe{ &mut *(self.data_ref.local_ref as *mut Camera) };

                while let Some(modifier) = self.modifiers.pop_front() {
                    modifier.write_local(local_ref)
                }
            }
        }

        pub struct AccessCameraData1{
            name:String,
            x:f32,
            y:f32,
        }

        pub struct AccessCameraView1<'a> {
            access:&'a mut AccessCamera1,
            name:&'a String,
            x:&'a f32,
            y:&'a f32,
        }

        impl<'a> AccessCameraView1<'a> {
            fn add_modifier(&mut self, transaction:&mut Transaction, modifier:AccessCameraModifier1) {
                if self.access.modifiers.len()==0 {
                    transaction.add_mod_access(self.access.index);
                }

                self.access.modifiers.push_back(modifier);
            }
        }

        impl<'a> AccessCameraView1<'a> {
            fn get(data_ref:&SDR<Camera>, transaction:&mut Transaction) -> Self {
                unsafe {
                //if data_ref.local_ref!=0 {
                    let data=data_ref.local_ref;

                    let access=AccessCamera1 {
                        data:None,
                        data_ref:data_ref.clone(),
                        modifiers:VecDeque::with_capacity(2),
                        index:0
                    };

                    let access=Box::new(access);
                    let access_ref=&mut *((access.as_ref() as *const AccessCamera1) as *mut AccessCamera1);

                    transaction.insert_access(access);

                    let view=AccessCameraView1 {
                        access:access_ref,
                        name:&*(&(*data).name as *const String),
                        x:&*(&(*data).x as *const f32),
                        y:&*(&(*data).y as *const f32)
                    };

                    view
                }
            }
        }

        pub enum AccessCameraModifier1 {
            V1(String),
            V2(f32,f32)
        }

        impl AccessCameraModifier1 {
            fn write_local(self, local_ref:&mut Camera) {
                match self {
                    AccessCameraModifier1::V1(s) => {},
                    AccessCameraModifier1::V2(x,y) => {
                        local_ref.x=x;
                        local_ref.y=y;
                    }
                }
            }
        }

        {
            let mut view = AccessCameraView1::get(&self_ref, transaction);

            view.add_modifier(transaction, AccessCameraModifier1::V2(0.5, 0.9));
            view.add_modifier(transaction, AccessCameraModifier1::V1("hello".to_string()));

            println!("{}", view.name);
            println!("{}", view.x)
        }

        transaction.finish();
    }


    fn get_pos(self_ref:SDR<Camera>, transaction:&mut Transaction){
        use std::collections::VecDeque;

        pub struct AccessCamera1{
            data:Option<Box<AccessCameraData1>>,
            data_ref:SDR<Camera>,
            modifiers:VecDeque<AccessCameraModifier1>,
            index:u32,
        }

        impl AccessTrait for AccessCamera1 {
            fn set_index(&mut self, index:u32) {
                self.index=index;
            }

            fn write_local(&mut self) {
                let local_ref=unsafe{ &mut *(self.data_ref.local_ref as *mut Camera) };

                while let Some(modifier) = self.modifiers.pop_front() {
                    modifier.write_local(local_ref)
                }
            }
        }

        pub struct AccessCameraData1{
            name:String,
            x:f32,
            y:f32,
        }

        pub struct AccessCameraView1<'a> {
            access:&'a mut AccessCamera1,
            name:&'a String,
            x:&'a f32,
            y:&'a f32,
        }

        impl<'a> AccessCameraView1<'a> {
            fn add_modifier(&mut self, transaction:&mut Transaction, modifier:AccessCameraModifier1) {
                if self.access.modifiers.len()==0 {
                    transaction.add_mod_access(self.access.index);
                }

                self.access.modifiers.push_back(modifier);
            }
        }

        impl<'a> AccessCameraView1<'a> {
            fn get(data_ref:&SDR<Camera>, transaction:&mut Transaction) -> Self {
                unsafe {
                    //if data_ref.local_ref!=0 {
                    let data=data_ref.local_ref;

                    let access=AccessCamera1 {
                        data:None,
                        data_ref:data_ref.clone(),
                        modifiers:VecDeque::with_capacity(2),
                        index:0
                    };

                    let access=Box::new(access);
                    let access_ref=&mut *((access.as_ref() as *const AccessCamera1) as *mut AccessCamera1);

                    transaction.insert_access(access);

                    let view=AccessCameraView1 {
                        access:access_ref,
                        name:&*(&(*data).name as *const String),
                        x:&*(&(*data).x as *const f32),
                        y:&*(&(*data).y as *const f32)
                    };

                    view
                }
            }
        }

        pub enum AccessCameraModifier1 {
            V1(String),
            V2(f32,f32)
        }

        impl AccessCameraModifier1 {
            fn write_local(self, local_ref:&mut Camera) {
                match self {
                    AccessCameraModifier1::V1(s) => {},
                    AccessCameraModifier1::V2(x,y) => {
                        local_ref.x=x;
                        local_ref.y=y;
                    }
                }
            }
        }

        {
            let mut view = AccessCameraView1::get(&self_ref, transaction);

            //view.add_modifier(transaction, AccessCameraModifier1::V2(0.5, 0.9));
            //view.add_modifier(transaction, AccessCameraModifier1::V1("hello".to_string()));

            println!("{}", view.name);
            println!("{}", view.x)
        }
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

    transaction.finish();
}

//Transaction
fn do_transaction2(camera_ref:SDR<Camera>) {
    let mut transaction=Transaction::new();

    <Camera as Instance>::get_pos(camera_ref, &mut transaction);

    transaction.finish();
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
    do_transaction2(camera_ref.clone());
}
