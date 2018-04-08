
use std::marker::PhantomData;

use prelude::{RT,AT,AMT};

//data stores in cash
pub struct MasterLocalContainer<R:RT,A:Arbiter<AM>,AM:AMT> {
    arbiter:A<AM>,
    _phantom_data:PhantomData(R),
}

//DB is not Local => Shared, but not shared DB
pub struct MasterDBContainer<R:RT,A:Arbiter<AM>,AM:AMT> {
    arbiter:A<AM>,
    _phantom_data:PhantomData(R),
    db:(),
    subscribe:()
}